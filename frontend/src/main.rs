use yew::prelude::*;
mod components;

use components::common::thumbnail::Thumbnail;
use components::fragments::form_text_input::FormTextInput;


#[function_component(App)]
fn app() -> Html {

    html! {
        <div>
            <Thumbnail src="https://images.unsplash.com/photo-1682695795798-1b31ea040caf?ixlib=rb-4.0.3&ixid=M3wxMjA3fDF8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&auto=format&fit=crop&w=3270&q=80"/>
            <FormTextInput label="Name" placeholder="Enter your name" value="John Doe"/>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}