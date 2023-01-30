use yew::prelude::*;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <div>
            <h1> { "Error 404: Cesta nenalezena. " } </h1>
        </div>
    }
}