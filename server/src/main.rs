use axum::{
    routing::{get, get_service},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json, Router,
    Extension,
};
use tower_http::services::{ServeDir, ServeFile};
use whale::{Package, Index};
use tokio::{fs::File, io::AsyncReadExt};
use std::{net::SocketAddr, path::Path};
use std::io;

const CONTENT_FOLDER: &str = "./content";

async fn handle_error(_err: io::Error) -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Invalid route")
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let index_file = get_index().await.expect("Failed to load index");

    let app = Router::new()
        .route("/api/packages", get(index))
        .route("/api/package/:id", get(package))
        .fallback_service(get_service(ServeDir::new("../app/dist").not_found_service(ServeFile::new("../app/dist/index.html"))).handle_error(handle_error))
        .layer(Extension(index_file));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn package(Extension(index): Extension<Index>, axum::extract::Path(id): axum::extract::Path<String>) -> Response {
    println!("GET package:{} called!", id);
    if index.is_allowed(&id) {
        if let Ok(mut file) = File::open(&Path::new(&format!("{}/{}.json", CONTENT_FOLDER, id))).await {
            let mut content = String::new();
            if let Ok(_) = file.read_to_string(&mut content).await {
                if let Ok(package) = serde_json::from_str::<Package>(&content) {
                    (StatusCode::OK, Json(package)).into_response()
                } else {
                    println!("Failed to serialize requested file");
                    (StatusCode::INTERNAL_SERVER_ERROR, "Failed to serialize requested file").into_response()
                }
            } else {
                println!("Failed to read requested file");
                (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read requested file").into_response()
            }
        } else {
            println!("Failed to open requested file");
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to open requested file").into_response()
        }
    } else {
        println!("Can't access requested file");
        (StatusCode::FORBIDDEN, "Can't access requested file").into_response()
    }


}

async fn index(Extension(index): Extension<Index>) -> Json<Index> {
    println!("GET index called!");
    Json(index)
}

async fn get_index() -> Result<Index, Box<dyn std::error::Error>> {
    let mut file = File::open(format!("{}/index.json", CONTENT_FOLDER)).await?;

    let mut content = String::new();
    file.read_to_string(&mut content).await?;

    let index: Index  = serde_json::from_str(&content)?;

    Ok(index)
}