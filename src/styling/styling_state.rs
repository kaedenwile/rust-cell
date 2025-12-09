use crate::color::Color;

#[derive(Clone)]
pub struct StylingState {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,

    pub bg: Color,
    pub fg: Color,
}

impl Default for StylingState {
    fn default() -> StylingState {
        StylingState {
            bold: false,
            italic: false,
            underline: false,
            bg: Color::White,
            fg: Color::Black,
        }
    }
}