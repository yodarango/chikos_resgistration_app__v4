use crate::components::enums::Alignment;
use yew::prelude::*;   

#[derive(PartialEq, Properties)]
pub struct ParagraphProps {
    pub children: Children,
    #[prop_or_default]
    pub class_name: Option<Classes>,
    #[prop_or_else(default_size)]
    pub size: u16,
    #[prop_or_default]
    pub color: String,
    #[prop_or_else(default_align)]
    pub alignment: Alignment,
    #[prop_or_default]
    pub style: AttrValue
}

fn default_size () ->u16 {
    16
}

fn default_align()-> Alignment{
    Alignment::Left 
}

#[function_component]
pub fn Paragraph (props: &ParagraphProps)-> Html{
    let ParagraphProps {children, class_name, size, color, alignment, style} = props;
    let color = format!("color: {};", color);
    let size = format!("font-size: {}px;", size);

    html!{
            <p class={classes!("chikios-paragraph", class_name.clone(), alignment.get_class())} style={color + &size + &style.clone().to_string()}>{for children.iter()}</p>
    }   
}