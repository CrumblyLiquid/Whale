use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::main::routing::Route;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
            <div>{ "Home" }</div>
            <Link<Route> to={Route::Index}>{ "Index" }</Link<Route>>
        </>
    }
}