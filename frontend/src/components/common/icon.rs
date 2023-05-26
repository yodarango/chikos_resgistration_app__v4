
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum IconName {
    Add,
    Edit,
    Check,
    X
}

#[derive(Properties, PartialEq,  Clone)]
pub struct IconProps {
    pub name: IconName,
    #[prop_or_default]
    pub class_name: Option<Classes>,
    #[prop_or(24)]
    pub size: u16,
    #[prop_or_default]
    pub color: AttrValue
}

#[function_component]
pub fn Icon(props: &IconProps) -> Html{
    let IconProps { name, class_name, size, color } = props;

    let icon: Html = match name {
                IconName::Add =>  html!{
                    <svg class={classes!(class_name.clone())} 
                        height={size.clone().to_string()}
                        width={size.clone().to_string()}
                        viewBox="0 0 512 512">
                        <path fill="none" stroke={color.clone()}
                        stroke-linecap="round" stroke-linejoin="round" 
                        stroke-width="32"
                        d="M256 112v288M400 256H112"
                        />
                    </svg>},
                IconName::Edit => html!{
                    <svg class={classes!(class_name.clone())} 
                        height={size.clone().to_string()}
                        width={size.clone().to_string()}
                        viewBox="0 0 512 512">
                        <path d="M384 224v184a40 40 0 01-40 40H104a40 40 0 01-40-40V168a40 40 0 0140-40h167.48" 
                        fill="none" 
                        stroke={color.clone()}
                        stroke-linecap="round" 
                        stroke-linejoin="round" 
                        stroke-width="32"/>
                        <path d="M459.94 53.25a16.06 16.06 0 00-23.22-.56L424.35 65a8 8 0 000 11.31l11.34 11.32a8 8 
                        0 0011.34 0l12.06-12c6.1-6.09 6.67-16.01.85-22.38zM399.34 90L218.82 270.2a9 9 0 00-2.31 
                        3.93L208.16 299a3.91 3.91 0 004.86 4.86l24.85-8.35a9 9 0 003.93-2.31L422 112.66a9 9 0 000-12.66l-9.95-10a9 
                        9 0 00-12.71 0z"
                        fill={color.clone()}
                        />
                    </svg>
                },
                IconName::Check => html!{
                    <svg 
                         class={classes!(class_name.clone())}
                        height={size.clone().to_string()}
                        width={size.clone().to_string()}
                        viewBox="0 0 512 512">
                        <path d="M448 256c0-106-86-192-192-192S64 150 64 256s86 192 192 192 192-86 192-192z" 
                        fill="none" stroke={color.clone()} stroke-miterlimit="10" 
                        stroke-width="32"/>
                        <path fill="none" 
                        stroke={color.clone()} 
                        stroke-linecap="round" 
                        stroke-linejoin="round" 
                        stroke-width="32" 
                        d="M352 176L217.6 336 160 272"/>
                    </svg>
                },
                IconName::X => html! {
                    <svg 
                        height={size.clone().to_string()}
                        width={size.clone().to_string()}
                        class={classes!(class_name.clone())} 
                        viewBox="0 0 512 512">
                        <path d="M448 256c0-106-86-192-192-192S64 150 64 256s86 192 192 192 192-86 192-192z" 
                        fill="none" stroke={color.clone()} 
                        stroke-miterlimit="10" 
                        stroke-width="32"/>
                        <path fill="none" stroke={color.clone()} 
                        stroke-linecap="round" 
                        stroke-linejoin="round" 
                        stroke-width="32" 
                        d="M320 320L192 192M192 320l128-128"/>
                    </svg>
                }
    };

    icon
}

