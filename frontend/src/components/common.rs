use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct ProfilePhotoProps {
    pub src: AttrValue,
    #[prop_or_default]
    pub alt: Option<AttrValue>,
    #[prop_or_default]
    pub class_name: Option<Classes>
}

#[function_component]
pub fn ProfilePhoto(props: &ProfilePhotoProps) -> Html{

    let ProfilePhotoProps { class_name, alt , src} = props;

    let alt: AttrValue = match &props.alt {
        Some(string)=> AttrValue::from(string.clone()),
        None => AttrValue::from("There was not a description provided by the user")
    };

    html!{
        <div class={classes!(class_name.clone())}>
            <img src={props.src.clone()} alt={alt}/>
        </div>
    }
}

// --------------------------------------------------------------
#[derive(PartialEq, Properties)]
pub struct InputProps {
    #[prop_or_default]
    pub placeholder: Option<AttrValue>,
    #[prop_or_default]
    pub input_type: Option<AttrValue>,
    #[prop_or_default]
    pub class_name: Option<Classes>,
    pub value: Option<AttrValue>,
}

#[function_component]
pub fn Input(props: &InputProps) -> Html{

    let InputProps {
        placeholder, 
        class_name, 
        input_type,
        value } = props;

    let placeholder = placeholder.as_ref().unwrap_or(&AttrValue::from("")).clone();
    let input_type =input_type.as_ref().unwrap_or(&AttrValue::from("text")).clone();
    let value = value.as_ref().unwrap_or(&AttrValue::from("")).clone();


    html!{
        <div class={classes!(class_name.clone())}>
            <input type={input_type} placeholder={placeholder} value={value}/>
        </div>
    }
}