use crate::color::Color;

#[derive(Clone)]
pub struct CellStyles {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,

    pub bg: Color,
    pub fg: Color,
}

impl Default for CellStyles {
    fn default() -> CellStyles {
        CellStyles {
            bold: false,
            italic: false,
            underline: false,
            bg: Color::White,
            fg: Color::Black,
        }
    }
}

impl CellStyles {
    pub fn colorful() -> CellStyles {
        CellStyles {
            bold: false,
            italic: false,
            underline: false,
            bg: Color::LightCyan,
            fg: Color::Magenta,
        }
    }
}