use enum_iterator::{all, Sequence};
use int_enum::IntEnum;

#[repr(i8)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, IntEnum, Sequence)]
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

pub fn color_to_value(_color: ResistorColor) -> u32 {
    _color.int_value() as u32
}

pub fn value_to_color_string(value: u32) -> String {
    if value > 9 {
        return String::from("value out of range");
    } else {
        match ResistorColor::from_int(value as i8) {
            Ok(c) => format!("{:?}", c),
            _ => String::from("value out of range"),
        }
    }
}

pub fn colors() -> Vec<ResistorColor> {
    all::<ResistorColor>().collect::<Vec<_>>()
}
