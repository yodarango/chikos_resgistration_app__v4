use crate::components::common::paragraph::Paragraph;
use web_sys::HtmlInputElement;
use gloo_console::log;
use yew::prelude::*;

fn default_input_type() -> AttrValue {
    AttrValue::from("text")
}

#[derive(PartialEq, Properties)]
pub struct InputProps {
    #[prop_or_default]
    pub placeholder: Option<AttrValue>,
    #[prop_or_else(default_input_type)]
    pub input_type: AttrValue,
    #[prop_or_default]
    pub class_name: Option<Classes>,
    #[prop_or_default]
    pub value: Option<AttrValue>,
    #[prop_or(false)]
    pub is_editing: bool,
    pub handle_change: Callback<String>,
}

#[function_component]
pub fn Input(props: &InputProps) -> Html{

    let InputProps {
        placeholder, 
        class_name, 
        input_type,
        value, 
        is_editing,
        ..
    } = props;

let handle_change = props.handle_change.clone();
let onchange= Callback::from(move |event: Event|{
    let input = event.target_dyn_into::<HtmlInputElement>(); 
    if let Some(input) = input {
        handle_change.emit(input.value());
    }
});

    html!{
        <div class={classes!(class_name.clone())}>
            if !*is_editing {
                <Paragraph class_name={Classes::from("form_text_input_")} >{value.clone()}</Paragraph>
            }else {
                <input type={input_type} placeholder={placeholder} value={value} {onchange}/>
            }
        </div>
    }
}