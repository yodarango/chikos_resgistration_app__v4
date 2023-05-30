use crate::components::common::{
    input::Input, 
    icon::{Icon, IconName},
    paragraph::Paragraph    
};  
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
pub struct FormTextInputProps {
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
pub fn FormTextInput (props: &FormTextInputProps)-> Html {

    let FormTextInputProps { 
        placeholder, 
        class_name, 
        value, 
        label, 
        align } = props;   

    let alignment_class = match align {
        Alignment::Vertical => String::from("layout-vertical"),
        Alignment::Horizontal => String::from("layout-horizontal"),
    };
    
    html!{
        <div class={classes!(alignment_class, class_name.clone())}>
            <Paragraph class_name={Classes::from("form_text_input_label")} text={label.clone()} />
            <Input input_type="text" placeholder={placeholder} value={value} />
            <Icon name={IconName::Edit} color="#F2F2F2" />
        </div>
    }
}