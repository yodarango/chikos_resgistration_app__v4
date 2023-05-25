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
}

#[function_component]
pub fn Input(props: &InputProps) -> Html{

    let InputProps {
        placeholder, 
        class_name, 
        input_type,
        value } = props;


    html!{
        <div class={classes!(class_name.clone())}>
            <input type={input_type} placeholder={placeholder} value={value}/>
        </div>
    }
}