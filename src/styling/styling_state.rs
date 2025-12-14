use crate::color::Color;
use crate::styling::alignment::Alignment;
use crate::styling::CellStyles;

#[derive(Clone)]
pub struct StylingState {
    pub cursor: u16,

    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub bg: Color,
    pub fg: Color,
    pub alignment: Alignment,
}

impl StylingState {
    pub fn from_cell_styles(styles: CellStyles) -> StylingState {
        StylingState {
            cursor: 0,
            bold: styles.bold,
            italic: styles.italic,
            underline: styles.underline,
            bg: styles.bg,
            fg: styles.fg,
            alignment: styles.alignment,
        }
    }

    pub fn to_cell_styles(&self) -> CellStyles {
        CellStyles {
            bold: self.bold,
            italic: self.italic,
            underline: self.underline,
            bg: self.bg,
            fg: self.fg,
            alignment: self.alignment,
        }
    }
}

impl Default for StylingState {
    fn default() -> StylingState {
        StylingState {
            cursor: 0,
            bold: false,
            italic: false,
            underline: false,
            bg: Color::White,
            fg: Color::Black,
            alignment: Alignment::Left,
        }
    }
}