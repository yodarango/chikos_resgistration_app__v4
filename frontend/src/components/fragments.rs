use crate::components::common::{Input};
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum Alignment {
    Vertical,
    Horizontal
}

fn default_align() -> Alignment {
   Alignment::Vertical
}


#[derive(PartialEq, Properties)]
pub struct FormInputProps {
    #[prop_or_else(default_align)]
    pub align: Alignment,
    #[prop_or_default]
    pub class_name: Option<Classes>,
    pub label: String,
    #[prop_or_default]
    pub placeholder: Option<AttrValue>,
    #[prop_or_default]
    pub value: Option<AttrValue>,

}


#[function_component]
pub fn FormInput (props: &FormInputProps)-> Html {

    let FormInputProps { 
        placeholder, 
        class_name, 
        value, 
        label, 
        align } = props;   

    html!{
        <div class={classes!(class_name.clone())}>
            <p>{label.clone()}</p>
            <Input input_type="text" placeholder={placeholder} value={value} />
        </div>
    }
}