use yew::prelude::*;

fn default_alt() -> AttrValue {
    AttrValue::from("There was not a description provided by the user")
}

#[derive(PartialEq, Properties)]
pub struct ThumbnailProps {
    pub src: AttrValue,
    #[prop_or_else(default_alt)]
    pub alt: AttrValue,
    #[prop_or_default]
    pub class_name: Option<Classes>
}

#[function_component]
pub fn Thumbnail(props: &ThumbnailProps) -> Html{

    let ThumbnailProps { class_name, alt , src} = props;

    html!{
        <div class={classes!(class_name.clone())}>
            <img src={src.clone()} alt={alt}/>
        </div>
    }
}
