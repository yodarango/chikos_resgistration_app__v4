use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct ProfilePhotoProps {
    pub src: String,
    pub alt: Option<String>,
    pub class_name: Option<String>
}

#[function_component]
pub fn ProfilePhoto(props: &ProfilePhotoProps) -> Html{

    let class_name: String = match &props.class_name {
        Some(string)=> String::from(string),
        None => String::from("")
    };

    let alt: String = match &props.alt {
        Some(string)=> String::from(string),
        None => String::from("There was not a description provided by the user")
    };


    html!{
        <div class={class_name}>
            <img src={props.src.clone()} alt={alt}/>
        </div>
    }
}

// --------------------------------------------------------------
#[derive(PartialEq, Properties)]
pub struct InputProps {
    input_type: Option<String>,
    class_name: Option<String>,
    placeholder: Option<String>,
    value: Option<String>,
}

#[function_component]
pub fn Input(props: &InputProps) -> Html{

    let class_name = props.class_name.as_ref().unwrap_or(&String::from("")).clone();
    let input_type = props.input_type.as_ref().unwrap_or(&String::from("text")).clone();
    let placeholder = props.placeholder.as_ref().unwrap_or(&String::from("")).clone();
    let value = props.value.as_ref().unwrap_or(&String::from("")).clone();


    html!{
        <div class={class_name}>
            <input type={input_type} placeholder={placeholder} value={value}/>
        </div>
    }
}