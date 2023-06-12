use gloo_console::log;
use yew::prelude::*;
use crate::components::fragments::form_row_input::FormTextInput;
use crate::components::common::card::Card;

#[derive(Properties, PartialEq, Clone)]
pub struct RegisterFormProps {


}


#[function_component]
pub fn RegisterForm ()-> Html{
    let handle_change = Callback::from(move |value: String| {
        log!("Value", value);
    });

    html!{
        <Card >
            <h3 class={classes!(String::from("mb-3 mx-0d mt-0"))}>{"Child"}</h3>
            <div class={classes!(String::from("d-flex align-items-start justify-content-center flex-wrap"))}>
                <FormTextInput label="First name" placeholder="Enter first name" value="" {handle_change} class_name={Classes::from("w-100")} is_editing={true}/>
            </div>
        </Card>
    }
}