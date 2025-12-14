use crate::color::Color;
use crate::styling::alignment::Alignment;

#[derive(Clone, PartialEq)]
pub struct CellStyles {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub bg: Color,
    pub fg: Color,
    pub alignment: Alignment,
}

impl Default for CellStyles {
    fn default() -> CellStyles {
        CellStyles {
            bold: false,
            italic: false,
            underline: false,
            bg: Color::White,
            fg: Color::Black,
            alignment: Alignment::Left,
        }
    }
}

impl CellStyles {
    pub fn serialize(&self) -> String {
        format!(
            "{};{};{};{};{};{}",
            self.bold as u8,
            self.italic as u8,
            self.underline as u8,
            self.bg as u8,
            self.fg as u8,
            self.alignment as u8
        )
    }

    pub fn deserialize(s: &str) -> Option<CellStyles> {
        let parts: Vec<&str> = s.split(';').collect();
        if parts.len() < 5 {
            return None;
        }

        let bold = parts[0].parse::<u8>().ok()? != 0;
        let italic = parts[1].parse::<u8>().ok()? != 0;
        let underline = parts[2].parse::<u8>().ok()? != 0;
        let bg = parts[3].parse::<u8>().ok()?;
        let fg = parts[4].parse::<u8>().ok()?;
        let alignment = parts.get(5)
            .and_then(|a| a.parse::<u8>().ok())
            .and_then(|a| Alignment::from_u8(a))
            .unwrap_or_default();

        Some(CellStyles {
            bold,
            italic,
            underline,
            bg: Color::from_u8(bg)?,
            fg: Color::from_u8(fg)?,
            alignment,
        })
    }
}