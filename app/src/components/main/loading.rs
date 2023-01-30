use yew::prelude::*;

#[function_component(Loading)]
pub fn loading() -> Html {
    html! {
        <div class="w-full h-full flex items-center justify-center">
            <div class="w-20 h-20 loader"></div>
        </div>
    }
}
