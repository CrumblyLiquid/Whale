use yew::prelude::*;
use yew_router::prelude::*;

use whale::Index as WhaleIndex;
use std::collections::HashMap;
use gloo_net::http::Request;

use crate::components::main::routing::Route;

#[function_component(Index)]
pub fn index() -> Html {
    let index: UseStateHandle<WhaleIndex> = use_state_eq( || WhaleIndex::new(HashMap::new()) );

    wasm_bindgen_futures::spawn_local({
        let index = index.clone();

        async move {
            let index = index.clone();
            match Request::get("http://127.0.0.1:3000/api/packages")
            .send()
            .await {
                Ok(result) => {
                    match result
                    .json::<WhaleIndex>()
                    .await {
                        Ok(i) => {
                            index.set(i);
                        },
                        Err(e) => {
                            gloo_console::log!(format!("An error occured while parsing /api/packages response to JSON ({:#?})", e));
                        }
                    };
                },
                Err(e) => {
                    gloo_console::log!(format!("Failed to fetch /api/packages ({:#?})", e));
                }
            }
        }
    });

    html! {
        // <div class="shadow-xl menu h-full min-h-full border border-gray-50 rounded-lg bg-gray-50">
        <div class="menu h-full min-h-full border border-gray-200 bg-gray-100">
            // <div class="w-full flex justify-center items-center rounded-t-lg border border-inherit border-b-gray-200 py-1">
            <div class="w-full flex justify-center items-center border border-gray-100 border-b-gray-200 py-1">
                <span>{ "Balíčky slovíček" }</span>
            </div>
            <div class="px-5 py-4 overflow-y-scroll space-y-4 last:pb-8">
            {
                index
                .to_vec()
                .iter()
                .map(
                    |info| html! {
                        // <div class="flex shadow-lg rounded-md text-xs space-x-4 hover:cursor-pointer p-4 bg-white hover:shadow-xl">
                        <div class="hover:shadow-sm">
                            <Link<Route> to={Route::Practice { id: info.filename.clone() }}>
                                <div class="flex text-xs space-x-4 p-4 bg-white border border-gray-200">
                                    <div class="flex flex-col w-fit">
                                        <div class="text-gray-600">{ "Název:" }</div>
                                        <div class="w-max">{ &info.name }</div>
                                        <div class="text-gray-600">{ "Autor:" }</div>
                                        <div class="w-max">{ &info.author }</div>
                                    </div>
                                    <div class="flex flex-col w-fit">
                                        <div class="text-gray-600">{ "Popis:" }</div>
                                        <div>{ &info.summary }</div>
                                    </div>
                                </div>
                            </Link<Route>>
                        </div>
                    }
                )
                .collect::<Html>()
            }
            </div>
        </div>
    }
}