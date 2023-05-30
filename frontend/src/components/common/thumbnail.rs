use yew::prelude::*;

fn default_alt() -> AttrValue {
    AttrValue::from("There was not a description provided by the user")
}

fn default_size() -> u16 {
    50
}   
#[derive(PartialEq, Properties)]
pub struct ThumbnailProps {
    #[prop_or_default]
    pub class_name: Option<Classes>,
    #[prop_or_else(default_size)]
    pub size: u16,
    pub src: AttrValue,
    #[prop_or_else(default_alt)]
    pub alt: AttrValue,
}

#[function_component]
pub fn Thumbnail(props: &ThumbnailProps) -> Html{

    let ThumbnailProps { class_name, alt , src, size} = props;

    let mut styles = String::from("");
    let size_styles = format!("height: {}px; width: {}px;", size, size);
    styles.push_str(&size_styles);

    html!{
        <div class={classes!(class_name.clone())} style={size_styles}>
            <img src={src.clone()} alt={alt.clone()}/>
        </div>
    }
}
