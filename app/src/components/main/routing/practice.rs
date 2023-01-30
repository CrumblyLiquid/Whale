use gloo_console::console;
use yew::prelude::*;

use whale::{Package, Input};
use gloo_net::http::Request;
use web_sys::{Event, HtmlInputElement};
use wasm_bindgen::JsCast;

#[derive(Properties, PartialEq)]
pub struct PracticeParams {
    pub id: String,
}

#[function_component(Practice)]
pub fn practice(params: &PracticeParams) -> Html {
    let id = params.id.clone();
    let package: UseStateHandle<Option<Package>> = use_state_eq( || None );

    wasm_bindgen_futures::spawn_local({
        let package = package.clone();

        async move {
            let package = package.clone();
            match Request::get(&("http://127.0.0.1:3000/api/package/".to_owned() + &id))
            .send()
            .await {
                Ok(result) => {
                    match result
                    .json::<Package>()
                    .await {
                        Ok(pac) => {
                            package.set(Some(pac));
                        },
                        Err(e) => {
                            gloo_console::log!(format!("An error occured while parsing /api/package/{} response to JSON ({:#?})", id, e));
                        }
                    };
                },
                Err(e) => {
                    gloo_console::log!(format!("Failed to fetch /api/package/{} ({:#?})", id, e));
                }
            }
        }
    });

    html! {
        if let Some(pac) = &*package {
            <PracticeComponent package={pac.clone()} />
        } else {
            <div class="min-h-full flex flex-col justify-center items-center">
                <h1> { "Chyba při načítání balíčku :(" } </h1>
                <h1> { "Pro více info se podívej do konzole." } </h1>
            </div>
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct PracticeComponentProperties {
    pub package: Package,
}

pub struct PracticeComponent {
    words: Vec<Vec<String>>,
    inputs: Vec<Input>,
    // input_refs: Vec<NodeRef>,
    index: usize,
    current: Vec<String>,
    values: Vec<String>,
}

impl PracticeComponent {
    fn next(&mut self) {
        self.index += 1;
        if self.index >= self.words.len() {
            self.index = 0;
        }
        self.current = self.words[self.index].clone();

        for i in 0..self.inputs.len() {
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Some(element) = document.get_element_by_id(&(i).to_string()) {
                        if let Ok(input) = element.dyn_into::<HtmlInputElement>() {
                            input.set_value("");
                            if i == 1 {
                                input.focus();
                            }
                        }
                    }
                }
            }
        }
    }
}

pub enum Msg {
    Input(usize, String),
    Check,
    GiveUp,
    Next,
}

impl Component for PracticeComponent {
    type Message = Msg;

    type Properties = PracticeComponentProperties;

    fn create(ctx: &Context<Self>) -> Self {
        let package = &ctx.props().package;
        Self {
            words: package.words.clone(),
            inputs: package.inputs.clone(),
            // input_refs: vec![NodeRef::default(); package.inputs.len()],
            index: 0,
            current: package.words[0].clone(),
            values: vec!["".to_string(); package.inputs.len()],
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Input(i, val) => {
                self.values[i] = val;
                // if let Some(input) = self.input_refs[i].cast::<HtmlInputElement>() {
                //     self.values[i] = input.value();
                //     if self.values[i] == self.current[i] && i-1 < self.input_refs.len() {
                //         if let Some(following_input) = self.input_refs[i+1].cast::<HtmlInputElement>() {
                //             following_input.focus();
                //         }
                //     }
                // }
                if self.values[i] == self.current[i] {
                    if let Some(window) = web_sys::window() {
                        if let Some(document) = window.document() {
                            if let Some(element) = document.get_element_by_id(&(i+1).to_string()) {
                                if let Ok(input) = element.dyn_into::<HtmlInputElement>() {
                                    input.focus();
                                }
                            } else {
                                self.next(); // TODO: We have to trigger next only after every filed is correct
                            }
                        }
                    }
                }
            },
            Msg::Check => {},
            Msg::GiveUp => {},
            Msg::Next => self.next(),
        };

        true // Whether to render the component after update
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let package = &ctx.props().package;

        let check = ctx.link().callback(|_| Msg::Check);
        let give_up = ctx.link().callback(|_| Msg::GiveUp);
        let next = ctx.link().callback(|_| Msg::Next);

        html! {
            <div class="m-6 mx-10">
                <div class="mb-4">
                    <span class="text-xl">{ package.name.clone() }</span>
                </div>
                // TODO: Add a progress bar
                <div class="flex flex-col space-y-4">
                {
                    self.inputs
                        .iter()
                        .enumerate()
                        .map( |(i, input)| {
                            let val = if i == 0 { Some(self.current[i].clone()) } else { None };

                            // Instead of using get_element_by_id I could use a list of NodeRefs
                            // bound to input_refs attribute
                            // (for some reaseon only the first input is bound to the NodeRef and the following only
                            // reference the first input - hence why I'm using get_element_by_id)
                            // let nref = self.input_refs[i].clone();
                            // if let Some(inp) = nref.cast::<HtmlInputElement>() {
                            //     todo!();
                            // }
                            html!
                            {
                                <label for={ i.to_string() } class="flex flex-col">
                                <span>{ input.name.clone() }</span>
                                <input
                                        // ref={ nref }
                                        class="border border-gray-700"
                                        id={ i.to_string() }
                                        type="text"
                                        placeholder={ input.example.clone() }
                                        disabled={ i == 0 }
                                        oninput={ ctx.link().callback( move |e: InputEvent| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            Msg::Input(i, input.value())
                                        }) }
                                        value={ val }
                                    />
                                </label>
                            }
                        }
                        )
                        .collect::<Html>()
                }
                </div>
                <div class="flex space-x-6 m-6">
                    <button class="border border-gray-700 p-2 hover:bg-black hover:text-white transition-colors" onclick={check}>{"Zkontrolovat"}</button>
                    <button class="border border-gray-700 p-2 hover:bg-black hover:text-white transition-colors" onclick={give_up}>{"Prozradit"}</button>
                    <button class="border border-gray-700 p-2 hover:bg-black hover:text-white transition-colors" onclick={next}>{"Další"}</button>
                </div>
            </div>
        }
    }
}