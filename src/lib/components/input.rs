use std::error::Error;
use yew::prelude::*;
use yew::{function_component, html, Html, Properties, use_state};
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub is_loading: bool,
}

#[function_component]
pub fn HelloWorld(props: &Props) -> Html {
    let holder = "";

    let value = use_state(String::default);
    let va = (*value).clone();
    let onchange= {
        let value = value.clone();
        Callback::from( move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                value.set(input.value())
            }
        })
    };

    if props.is_loading.clone() {
        html! {
            <>
            <input placeholder={holder} value={va} onchange={onchange}/>
            {(*value).clone()}
            </>
        }
    } else {
        html! { "Hello world" }
    }
}

