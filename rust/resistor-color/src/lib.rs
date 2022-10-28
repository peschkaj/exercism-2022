use enum_iterator::{all, Sequence};
use int_enum::IntEnum;
use std::fmt::Display;

#[derive(Copy, Clone, Debug, PartialEq, Eq, IntEnum, Sequence)]
#[repr(u32)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

impl Display for ResistorColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn color_to_value(_color: ResistorColor) -> u32 {
    _color.int_value()
}

pub fn value_to_color_string(value: u32) -> String {
    let res = ResistorColor::from_int(value);
    match res {
        Ok(color) => color.to_string(),
        _ => String::from("value out of range")
    }
}

pub fn colors() -> Vec<ResistorColor> {
    all::<ResistorColor>().collect::<Vec<_>>()
}
