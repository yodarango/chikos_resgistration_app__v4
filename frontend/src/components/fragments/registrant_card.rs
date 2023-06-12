use yew::prelude::*;
use crate::components::common::card::Card;
use crate::components::common::thumbnail::Thumbnail;
use crate::components::common::icon::{Icon, IconName};

#[derive(PartialEq, Properties)]
pub struct RegistrantCardProps {
    pub thumbnail: AttrValue,
    pub first_name: String,
    pub last_name: String,
    #[prop_or(false)]
    pub is_logged_in: bool,
    pub age: u8
}

#[function_component]
pub fn RegistrantCard (props: &RegistrantCardProps) -> Html {
    let RegistrantCardProps { thumbnail, first_name, last_name, age,is_logged_in } = props;

    let is_logged_in = is_logged_in.clone();

    let border_class = if is_logged_in {
        "border border-success"
    } else {
        "border border-danger"
    };

    let check_bg = if is_logged_in {
        "bg-success"
    } else {
        "bg-danger"
    };
    html!{
        <Card class_name={Classes::from(format!("d-flex align-items-center justify-content-start {}", border_class))}>
            <Thumbnail src={thumbnail.clone()} alt={AttrValue::from("registrant ID photo")} class_name={Classes::from("me-3 flex-shrink-0")} />
            <h3 class={classes!("font-color", "m-0", "w-100")}>{format!("{} {}, {}", first_name.clone(), last_name.clone(), age.clone())}</h3>
            <button class={classes!("btn", "btn-primary", "ms-auto", check_bg, "rounded-circle", "p-0", "d-flex", "align-items-center", "justify-content-center",)}>
                {match is_logged_in {
                    true => html!{<Icon name={IconName::Check} color={AttrValue::from("#175515")} class_name={Classes::from("flex-shrink-0 p-0")}/>},
                    false => html!{<Icon name={IconName::X} color={AttrValue::from("#8a2121")} class_name={Classes::from("flex-shrink-0 p-0")}/>}
                }}
            </button>
        </Card>
    }
}