use yew::prelude::*;

pub mod routing;
use routing::Router;

#[function_component(Main)]
pub fn main() -> Html {
    html! {
        <main class="lg:p-7 min-h-full">
            <Router />
        </main>
    }
}
