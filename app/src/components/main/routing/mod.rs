use yew::prelude::*;
use yew_router::prelude::*;

mod info;
use info::Info;
mod index;
use index::Index;
mod practice;
use practice::Practice;
mod error;
use error::{NotFound};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Index,
    #[at("/info")]
    Info,
    #[at("/practice/:id")]
    Practice { id: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Info => html! { <Info /> },
        Route::Index => html! { <Index /> },
        Route::Practice { id } => html! { <Practice id={id} /> },
        Route::NotFound => html! { <NotFound /> },
    }
}

#[function_component(Router)]
pub fn router() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}