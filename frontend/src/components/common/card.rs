use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct CardProps {
    pub children: Children,
    #[prop_or_default]
    pub class_name: Option<Classes>,
}

#[function_component]
pub fn Card (props: &CardProps)-> Html{
    html!{
        <div class={classes!("p-4", "card", "bg-secondary", "shadow", "rounded-3", props.class_name.clone())}>
            {for props.children.iter()}
        </div>
    }
}