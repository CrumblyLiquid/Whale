use gloo_console::log;
use yew::prelude::*;

use whale::{Package, Input};
use gloo_net::http::Request;
use web_sys:: HtmlInputElement;
use wasm_bindgen::JsCast;

use js_sys::Math;

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
    // A word is first moved from unchecked to selected and then either deleted or moved to failed
    unchecked: Vec<Vec<String>>, // Words which haven't been checked yet
    failed: Vec<Vec<String>>, // Words which have been checked but failed
    done: usize, // How many words we've succesfully completed
    selected: Vec<String>, // Currently selected word

    completed: bool, // If we've completed the whole wordset

    inputs: Vec<Input>, // Drawing inputs
    // input_refs: Vec<NodeRef>,
    starting: usize, // Pre-filled input

    input_values: Vec<String>, // Values of our inputs (updated through callbacks)
}

impl PracticeComponent {
    // Generate random index to choose a word
    fn gen_rand(size: usize) -> usize {
        let rand = Math::random();
        (rand * (size - 1) as f64) as usize
    }

    fn next(&mut self, success: bool) {
        let unchecked_len = self.unchecked.len();
        let failed_len = self.failed.len();

        // If both are empty
        if unchecked_len + failed_len > 0 {
            // If previous word was passed successfully
            if success {
                self.done += 1;
            } else {
                self.failed.push(self.selected.clone())
            }

            // Pick a new word (with odds of a failed one slightly higher)
            if unchecked_len > 0 && failed_len == 0 { // No failed but some unchecked
                let selected_index = PracticeComponent::gen_rand(unchecked_len);
                self.selected = self.unchecked.remove(selected_index);
            } else if unchecked_len == 0 && failed_len > 0 { // Some failed but no unchecked
                let selected_index = PracticeComponent::gen_rand(failed_len);
                self.selected = self.failed.remove(selected_index);
            } else { // Some failed and some unchecked
                let rnd_num = Math::random();

                if rnd_num > 0.4 && failed_len > 0 {
                    let selected_index = PracticeComponent::gen_rand(failed_len);
                    self.selected = self.failed.remove(selected_index);
                } else {
                    let selected_index = PracticeComponent::gen_rand(unchecked_len);
                    self.selected = self.unchecked.remove(selected_index);
                }
            }

            log!(self.selected.join(", "));

            // Reset inputs and focus the first correct input
            let mut focused = false;
            for input_index in 0..self.inputs.len() {
                if let Some(window) = web_sys::window() {
                    if let Some(document) = window.document() {
                        if let Some(element) = document.get_element_by_id(&input_index.to_string()) {
                            if let Ok(input) = element.dyn_into::<HtmlInputElement>() {
                                input.set_value("");
                                // Focus first available input that is not a starting input
                                if !focused && input_index != self.starting {
                                    if let Ok(_) = input.focus() {
                                        focused = true;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        // Mark as completed
        // TODO: Display a message
        } else {
            self.completed = true;
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

        // Cloning so I can remove words from the vector
        let mut unchecked = package.words.clone();

        // Choose first word
        let selected_index = PracticeComponent::gen_rand(unchecked.len());
        let selected = unchecked.remove(selected_index);

        Self {
            unchecked: unchecked,
            failed: Vec::new(),
            done: 0,
            // input_refs: vec![NodeRef::default(); package.inputs.len()],
            selected: selected,
            completed: false,
            inputs: package.inputs.clone(),
            starting: 0, // TODO: Make starting input configurable
            // Start with empty values so we don't get invalid index
            input_values: vec!["".to_string(); package.inputs.len()],
        }
    }

    // Process messages
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Input(i, val) => {
                // Update input value
                self.input_values[i] = val;

                // if let Some(input) = self.input_refs[i].cast::<HtmlInputElement>() {
                //     self.values[i] = input.value();
                //     if self.values[i] == self.selected[i] && i-1 < self.input_refs.len() {
                //         if let Some(following_input) = self.input_refs[i+1].cast::<HtmlInputElement>() {
                //             following_input.focus();
                //         }
                //     }
                // }

                // Check if the specific input is correct and if so, focus on the following one or move to a new word
                if self.input_values[i] == self.selected[i] {
                    let mut next_focus: Option<HtmlInputElement> = None;

                    for input_index in 0..self.inputs.len() {
                        // Check if input is not starting index (won't have value)
                        // and if the input holds the correct value
                        if input_index != self.starting && self.input_values[input_index] != self.selected[input_index] {
                            if let Some(window) = web_sys::window() {
                                if let Some(document) = window.document() {
                                    if let Some(element) = document.get_element_by_id(&input_index.to_string()) {
                                        if let Ok(input) = element.dyn_into::<HtmlInputElement>() {
                                            next_focus = Some(input);
                                        }
                                    }
                                }
                            }
                        }

                        if input_index > i && next_focus.is_some() {
                            break;
                        }
                    }

                    // Advance to the next input in line or the next word
                    if let Some(input) = next_focus {
                        if let Err(e) = input.focus() {
                            log!(String::from("Failed to focus input"), e);
                        }
                    } else {
                        self.next(true);
                    }
                }
            },
            Msg::Check => {},
            Msg::GiveUp => {},
            Msg::Next => self.next(true),
        };

        true // Whether to render the component after update
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let package = &ctx.props().package;

        let check = ctx.link().callback(|_| Msg::Check);
        let give_up = ctx.link().callback(|_| Msg::GiveUp);
        let next = ctx.link().callback(|_| Msg::Next);

        // Calculate progress bar width
        let progress: String = format!("width: {0}%;", (self.done as f32/(self.done+self.unchecked.len()+self.failed.len()) as f32)*99.0);

        html! {
            <div class="m-6 mx-10 space-y-4">
                // Title (and maybe other info in the future)
                <div>
                    <span class="text-xl">{ package.name.clone() }</span>
                </div>
                // Progress bar
                <div class="border-2 border-gray-700">
                    <div class="m-0.5 h-3 bg-green-500" style={progress}>
                    </div>
                </div>
                // Main input are
                <div class="flex flex-col space-y-4">
                {
                    self.inputs
                        .iter()
                        .enumerate()
                        .map( |(i, input)| {
                            let val = if i == 0 { Some(self.selected[i].clone()) } else { None };

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
                // Buttons lol
                <div class="flex space-x-6 m-6">
                    <button class="border border-gray-700 p-2 hover:bg-black hover:text-white transition-colors" onclick={check}>{"Zkontrolovat"}</button>
                    <button class="border border-gray-700 p-2 hover:bg-black hover:text-white transition-colors" onclick={give_up}>{"Prozradit"}</button>
                    <button class="border border-gray-700 p-2 hover:bg-black hover:text-white transition-colors" onclick={next}>{"Další"}</button>
                </div>
            </div>
        }
    }
}