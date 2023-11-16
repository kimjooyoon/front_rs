mod app;
mod lib;

use yew::prelude::*;
use crate::lib::components::input::HelloWorld;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let loading = use_state(|| false);
    let onclick = {
        let counter = counter.clone();
        let loading = loading.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
            loading.set(true)
        }
    };

    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
            <HelloWorld is_loading={ *loading } />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}