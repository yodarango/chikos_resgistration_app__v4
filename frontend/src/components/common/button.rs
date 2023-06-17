use crate::components::common::icon::Icon;
use yew::prelude::*;

fn default_style () -> String {
    String::from("")
}

#[derive(PartialEq, Properties)]
pub struct ButtonProps {
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    pub children: Children,
    #[prop_or_default]
    pub class_name: Option<Classes>,
    #[prop_or_else(default_style)]
    style: String,

}

#[function_component]
pub fn Button (props : &ButtonProps)-> Html{
    let ButtonProps {children, class_name, style, onclick, } = props;

    let onclick = onclick.clone();



    html!{
        <button class={classes!("btn", "rounded-1", class_name)} style={style.clone()} {onclick}>
            <span>{for children.iter()}</span>
        </button>
    }
}