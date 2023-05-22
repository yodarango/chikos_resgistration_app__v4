use yew::prelude::*;
use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize)]
struct Video {
    id: usize,
    title: String,
    speaker: String,
    url: String,
}

#[function_component(App)]
fn app() -> Html {


    html! {
        <div>
            <h1>{"Heyyy"}</h1>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}