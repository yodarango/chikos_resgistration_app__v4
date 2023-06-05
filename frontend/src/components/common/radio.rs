use gloo_console::log;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct FormRadioProps {
    //pub handle_change: Callback<String>,  
    pub value: [AttrValue; 2],
    pub name: AttrValue,
    pub checked: [bool; 2],
    pub label: AttrValue,
}

#[function_component]
pub fn Radio (props: &FormRadioProps)-> Html {
    let FormRadioProps { 
        //handle_change, 
        checked, 
        label, 
        value, 
        name,
        } = props;   

        let value = value.clone();

    let onchange = Callback::from(move |value: Event| {
        let input = value.target_dyn_into::<HtmlInputElement>();

            log!("Value", input);
        
        // if let Some(input) = input {
        //     handle_change.emit(input.value());
        // }
    });

    html!{
        <div class="form_radio">
            <input type="radio" name={name.clone()} checked={checked[0]} value={value.get(0).map(|&val| val)} onchange={&onchange}/>
            <input type="radio" name={name.clone()} checked={checked[1]} value={value.get(0).map(|&val| val)} onchange={&onchange}/>
            <label>{label}</label>
        </div>
    }
}