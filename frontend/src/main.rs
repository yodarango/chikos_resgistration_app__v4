use yew::prelude::*;
mod components;

use components::common::ProfilePhoto;


#[function_component(App)]
fn app() -> Html {


    html! {
        <ProfilePhoto src="https://images.unsplash.com/photo-1682695795798-1b31ea040caf?ixlib=rb-4.0.3&ixid=M3wxMjA3fDF8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&auto=format&fit=crop&w=3270&q=80"/>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}