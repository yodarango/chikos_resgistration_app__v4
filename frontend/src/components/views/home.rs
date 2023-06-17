use yew::prelude::*;

use crate::components::layouts::register_form::RegisterForm;

#[function_component]
pub fn Home () -> Html {
    html! {
        <div>
            <RegisterForm />
        </div>
    }
}