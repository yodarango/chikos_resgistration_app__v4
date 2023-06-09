use yew::prelude::*;
use crate::components::common::card::Card;
use crate::components::common::thumbnail::Thumbnail;
use crate::components::common::icon::{Icon, IconName};

#[derive(PartialEq, Properties)]
pub struct RegistrantCardProps {
    pub thumbnail: AttrValue,
    pub first_name: String,
    pub last_name: String,
    pub age: u8
}

#[function_component]
pub fn RegistrantCard (props: &RegistrantCardProps) -> Html {
    let RegistrantCardProps { thumbnail, first_name, last_name, age } = props;
    html!{
        <Card class_name={Classes::from("d-flex align-items-center justify-content-start")}>
            <Thumbnail src={thumbnail.clone()} alt={AttrValue::from("registrant ID photo")} class_name={Classes::from("me-3 flex-shrink-0")} />
            <h3 class={classes!("font-color", "m-0", "w-100")}>{format!("{} {}, {}", first_name.clone(), last_name.clone(), age.clone())}</h3>
            <button class={classes!("btn", "btn-primary", "ms-auto", "bg-success", "rounded-circle", "p-0", "d-flex", "align-items-center", "justify-content-center",)}>
                <Icon name={IconName::Check} color={AttrValue::from("#175515")} class_name={Classes::from("flex-shrink-0 p-0")}/>
            </button>
        </Card>
    }
}