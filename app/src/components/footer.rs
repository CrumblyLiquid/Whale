use yew::prelude::*;
use yew::Properties;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub version: String,
    pub author: String,
    pub author_link: String,
}

#[function_component(Footer)]
pub fn footer(props: &Props) -> Html {
    html! {
        // <footer class="flex items-center place-content-between shadow-inner p-2 px-3 space-x-4 bg-gray-50">
        <footer class="flex items-center place-content-between p-2 px-3 space-x-4 bg-gray-100 border border-gray-100 border-t-gray-200">
            <div>
                { format!("v{}", props.version.clone())}
            </div>
            <div>
                <a class="link" target="_blank" href={props.author_link.clone()}>{ format!("by {}", props.author.clone()) }</a>
            </div>
        </footer>
    }
}
