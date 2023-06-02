use crate::components::common::paragraph::Paragraph;
use gloo_console::log;
use web_sys::HtmlInputElement;
use yew::prelude::*;


#[derive(PartialEq, Properties)]
pub struct InputNumberProps {
    #[prop_or_default]
    pub placeholder: Option<AttrValue>,
    #[prop_or_default]
    pub class_name: Option<Classes>,
    #[prop_or_default]
    pub value: Option<AttrValue>,
    #[prop_or(false)]
    pub is_editing: bool,
    pub handle_change: Callback<u8>,
}

#[function_component]
pub fn InputNumber(props: &InputNumberProps) -> Html{

    let InputNumberProps {
        placeholder, 
        class_name, 
        value, 
        is_editing,
        ..
    } = props;

let handle_change = props.handle_change.clone();

let onchange= Callback::from(move |event: Event|{
    let input = event.target_dyn_into::<HtmlInputElement>(); 
    if let Some(input) = input {
       let value = input.value().parse::<u8>().unwrap_or(1);
        handle_change.emit(value);
    }
});

    html!{
        <div class={classes!("form_input_number",class_name.clone())}>
            if !*is_editing {
                <Paragraph class_name={Classes::from("form_text_input_number")} >{value.clone()}</Paragraph>
            }else {
                <input type="number" placeholder={placeholder} value={value} {onchange}/>
            }
        </div>
    }
}