use crate::color::Color::Black;

#[derive(Clone, Copy, PartialEq)]
pub enum Alignment {
    Left,
    Right,
}
use Alignment::*;


impl Default for Alignment {
    fn default() -> Alignment {
        Left
    }
}

impl Alignment {
    pub fn name(&self) -> &'static str {
        match self {
            Left => "Left",
            Right => "Right",
        }
    }

    pub fn from_u8(value: u8) -> Option<Alignment> {
        match value {
            0 => Some(Left),
            1 => Some(Right),
            _ => None,
        }
    }
}