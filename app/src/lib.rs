use yew::prelude::*;

mod components;
use components::header::Header;
use components::main::Main;
use components::footer::Footer;

mod constants;
use constants::{VERSION, AUTHORS};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <Header name={"Whale"} icon_path={"/whale.svg"} />
            <Main />
            <Footer version={VERSION.unwrap_or("0.1")} author={AUTHORS.unwrap_or("CrumblyLiquid").replace(":", ", ")} author_link={"https://github.com/CrumblyLiquid/"} />
        </>
    }
}
