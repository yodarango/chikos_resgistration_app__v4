use crate::components::common::{
    input::Input, 
    input_number::InputNumber,
    icon::{Icon, IconName},
    paragraph::Paragraph    
};  
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum Alignment {
    Vertical,
    Horizontal
}

#[derive(Clone, PartialEq)]
pub enum InputType {
    Text,
    Number,
}

fn default_align() -> Alignment {
   Alignment::Vertical
}

fn default_input_type() -> InputType {
    InputType::Text
}


#[derive(PartialEq, Properties)]
pub struct FormTextInputProps {
    #[prop_or_else(default_align)]
    pub align: Alignment,
    #[prop_or_else(default_input_type)]
    pub input_type: InputType,
    #[prop_or_default]
    pub class_name: Option<Classes>,
    pub label: String,
    #[prop_or_default]
    pub placeholder: Option<AttrValue>,
    #[prop_or_default]
    pub value: Option<AttrValue>,
    #[prop_or_default]
    pub handle_change: Callback<String>,
    #[prop_or_default]
    pub handle_change_number: Callback<u8>,
    #[prop_or(false)]
    pub can_edit: bool,
    #[prop_or(false)]
    pub is_editing: bool,
}


#[function_component]
pub fn FormTextInput (props: &FormTextInputProps)-> Html {
    let is_editing = use_state(|| props.is_editing.clone());

    let on_edit = {
        let is_editing = is_editing.clone();
        Callback::from(move |_| {
            is_editing.set(!*is_editing);
        })
    };

    let FormTextInputProps { 
        placeholder, 
        class_name, 
        value, 
        label, 
        align,
        input_type,
         .. } = props;   

    let alignment_class = match align {
        Alignment::Vertical => String::from("flex-wrap"),
        Alignment::Horizontal => String::from("flex-nowrap"),
    };
    
    html!{
        <div class={classes!("d-flex", "align-items-center", "flex-nowrap", alignment_class, class_name.clone())}>
            <Paragraph class_name={Classes::from("me-3 flex-shrink-0")} >{format!("{}:",label)}</Paragraph>
            {match input_type {
                InputType::Text => html!{<Input input_type="text" placeholder={placeholder} value={value} is_editing={*is_editing} handle_change={props.handle_change.clone()} class_name={Classes::from("w-100")} />},
                InputType::Number => html!{<InputNumber placeholder={placeholder} value={value} is_editing={*is_editing} handle_change={props.handle_change_number.clone()} class_name={Classes::from("w-100")}/>},
            }}
                
            if props.can_edit { 
                    <button onclick={on_edit} class={classes!("d-block", "ms-3")}>
                    if !*is_editing{
                    <Icon name={IconName::Edit} color="#F2F2F2" />
                    } else {
                    <Icon name={IconName::Check} color="#F2F2F2" />
                    }
                    </button>
                }
            
        </div>
    }
}