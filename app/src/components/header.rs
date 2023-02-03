use yew::prelude::*;
use yew::Properties;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub icon_path: String,
}

#[function_component(Header)]
pub fn header(props: &Props) -> Html {
    html! {
        // <header class="flex items-center justify-center shadow-sm text-lg space-x-4 p-1 bg-gray-50">
        <header class="">
            <a href="/" class="flex items-center justify-center text-lg space-x-4 p-1 bg-gray-100 border border-gray-100 border-b-gray-200">
                <img class="w-10 h-10" src={props.icon_path.clone()}/>
                <span>{props.name.clone()}</span>
            </a>
        </header>
    }
}