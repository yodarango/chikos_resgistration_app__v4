use yew::prelude::*;
use gloo_console::log;
mod components;

use components::common::thumbnail::Thumbnail;
use components::fragments::form_row_input::{InputType, FormTextInput};
use components::common::button::Button;
use components::common::radio::Radio;


#[function_component(App)]
fn app() -> Html {

    let handle_change = Callback::from(move |value: String| {
        log!("Value", value);
    });

    let handle_change_number = Callback::from(move |value: u8| {
            log!("Value", value);
    });

    let radio_change = Callback::from(move |value: String| {
        log!("Value", value);
    });

    let onclick = Callback::from(move |_| {
        log!("Hello World");
    });

    html! {
        <div>
            <Thumbnail src="https://images.unsplash.com/photo-1682695795798-1b31ea040caf?ixlib=rb-4.0.3&ixid=M3wxMjA3fDF8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&auto=format&fit=crop&w=3270&q=80"/>
            <FormTextInput label="Name" placeholder="Enter your name" value="John Doe" {handle_change}/>
            <FormTextInput label="Age"  handle_change_number={handle_change_number} input_type={InputType::Number}/>
            <Radio name={"gender" } value={[AttrValue::from("male"), AttrValue::from("female")]} checked={[false, false]} />
            <Button {onclick}> {"Hello World"}</Button>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}