use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub src: String,
    pub alt: Option<String>,
    pub class_name: Option<String>
}

#[function_component]
pub fn ProfilePhoto(props: &Props) -> Html{

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