use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct ButtonProps {
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    pub children: Children,
    #[prop_or_default]
    pub class_name: Option<Classes>,
    #[prop_or_default]
    pub color: String,
    #[prop_or_default]
    pub bg_color: String,
}

#[function_component]
pub fn Button (props : &ButtonProps)-> Html{
    let ButtonProps {children, class_name, bg_color, color, onclick} = props;

    let bg_color = format!("background-color: {};", bg_color);
    let text_color = format!("color: {};", color);

    let onclick = onclick.clone();



    html!{
        <button class={classes!("btn","primary", class_name)} style={bg_color} {onclick}>
            <span style={text_color}>{for children.iter()}</span>
        </button>
    }
}