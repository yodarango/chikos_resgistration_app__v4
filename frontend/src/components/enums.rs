#[derive(PartialEq)]
pub enum Alignment {
    Left,
    Center,
    Right,
}

impl Alignment {
 pub fn get_class(&self)-> String {
    match self {
        Alignment::Left => String::from("text-left"),
        Alignment::Center => String::from("text-center"),
        Alignment::Right => String::from("text-right"),
    }
 }
}