use gloo_console::log;
use yew::prelude::*;

use crate::components::fragments::form_row_input::{FormTextInput, InputType};
use crate::components::layouts::upload_photo::UploadPhoto;
use crate::components::common::paragraph::Paragraph;
use crate::components::common::button::Button;
use crate::components::common::card::Card;
use common::models::Registrant;
use std::ops::Deref;

#[derive(Properties, PartialEq, Clone)]
pub struct RegisterFormProps {


}


#[function_component]
pub fn RegisterForm ()-> Html{

    let form_data = use_state(|| Registrant::default());
    
    let cloned_state = form_data.clone();
    let handle_child_first_name = Callback::from(move |value: String| {
        let mut form_data = cloned_state.deref().clone();
        form_data.first_name = value;
        log!(format!("{:?}", &form_data));
        cloned_state.set(form_data);
    });

    let cloned_state = form_data.clone();
    let handle_child_last_name = Callback::from(move |value: String| {
        let mut form_data = cloned_state.deref().clone();
        form_data.last_name = value;
            log!(format!("{:?}", &form_data));
        cloned_state.set(form_data);
    });

    let cloned_state = form_data.clone();
    let handle_child_age = Callback::from(move |value: u8| {
        let mut form_data: Registrant = cloned_state.deref().clone();
        form_data.age = value;
            log!(format!("{:?}", &form_data));
        cloned_state.set(form_data);
    });

    let cloned_state = form_data.clone();
    let handle_guardian_first_name = Callback::from(move |value: String| {
        let mut form_data = cloned_state.deref().clone();
         if let Some(guardian) = &mut form_data.guardian {
                    guardian.first_name = value;
        }

        cloned_state.set(form_data);
    });

    let cloned_state = form_data.clone();
    let handle_guardian_last_name = Callback::from(move |value: String| {
        let mut form_data = cloned_state.deref().clone();
         if let Some(guardian) = &mut form_data.guardian {
                    guardian.last_name = value;
        }

        cloned_state.set(form_data);
    });

    let cloned_state = form_data.clone();
    let handle_guardian_phone_num = Callback::from(move |value: String| {
        let mut form_data = cloned_state.deref().clone();

        let parsed_number: Result<u64, _> = value.clone().parse();
        let parsed_number = match parsed_number {
            Ok(num) => num,
            Err(_) => 0,
        };

         if let Some(guardian) = &mut form_data.guardian {
            guardian.phone_number = parsed_number;
        }

        cloned_state.set(form_data);
    });

    let label_styles = AttrValue::from("min-width: 100px;");

    let handle_submit = Callback::from(move |_| {
        log!(format!("{:?}", &form_data))
    });

    html!{
        <Card>
            <UploadPhoto />
            <h3 class={classes!(String::from("mb-3 mx-0d mt-0"))}>{"Child"}</h3>
            <div class={classes!("mb-5")}>
                <FormTextInput label_style={&label_styles} label="First name" placeholder="Enter first name" handle_change={&handle_child_first_name} class_name={Classes::from("w-100 mb-4")} is_editing={true}/>
                <FormTextInput label_style={&label_styles} label="Last name" placeholder="Enter last name"  handle_change={&handle_child_last_name} class_name={Classes::from("w-100 mb-4")} is_editing={true}/>
                <FormTextInput label_style={&label_styles} input_type={InputType::Number} label="Age" placeholder="0" handle_change_number={&handle_child_age} class_name={Classes::from("w-100 mb-4")} is_editing={true}/>
            </div>

            <h3 class={classes!(String::from("mb-3 mx-0d mt-0"))}>{"Guardian"}</h3>
            <div class={classes!("mb-5")}>
                <FormTextInput label_style={&label_styles} label="First name" placeholder="Enter first name"  handle_change={&handle_guardian_first_name} class_name={Classes::from("w-100 mb-4")} is_editing={true}/>
                <FormTextInput label_style={&label_styles} label="Last name" placeholder="Enter last name"  handle_change={&handle_guardian_last_name} class_name={Classes::from("w-100 mb-4")} is_editing={true}/>
                <FormTextInput label_style={&label_styles} label="Phone #" placeholder="Enter phone number"  handle_change={&handle_guardian_phone_num} class_name={Classes::from("w-100 mb-4")} is_editing={true}/>
                <div class={classes!(String::from("d-flex align-items-start justify-content-center flex-nowrap mb-4"))}>
                    <Paragraph style={label_styles} class_name={Classes::from("mb-2")}>{"Notes: "}</Paragraph>
                    <textarea class={classes!(String::from("form_textarea d-block bg-secondary p-3 rounded-3 color-tertiary w-100"))} placeholder="Enter address" rows=3></textarea>
                </div>
            </div>

            <Button onclick={handle_submit} class_name={classes!(String::from("primary w-100"))}>{"Register"}</Button>
        </Card>
    }
}