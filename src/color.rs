use termion::color;

#[derive(Clone, Copy, PartialEq)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Gray,
    LightBlack,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    LightWhite,
}
use Color::*;

static LIGHT_GRAY_FG: &str = "\x1B[38;2;220;220;220m";
static LIGHT_GRAY_BG: &str = "\x1B[48;2;220;220;220m";

static GRAY_FG: &str = "\x1B[38;2;150;150;150m";
static GRAY_BG: &str = "\x1B[48;2;150;150;150m";

impl Color {
    pub fn bg(&self) -> String {
        match self {
            Black => color::Black.bg_str(),
            Red => color::Red.bg_str(),
            Green => color::Green.bg_str(),
            Yellow => color::Yellow.bg_str(),
            Blue => color::Blue.bg_str(),
            Magenta => color::Magenta.bg_str(),
            Cyan => color::Cyan.bg_str(),
            White => color::White.bg_str(),
            Gray => GRAY_BG,
            LightBlack => color::LightBlack.bg_str(),
            LightRed => color::LightRed.bg_str(),
            LightGreen => color::LightGreen.bg_str(),
            LightYellow => color::LightYellow.bg_str(),
            LightBlue => color::LightBlue.bg_str(),
            LightMagenta => color::LightMagenta.bg_str(),
            LightCyan => color::LightCyan.bg_str(),
            LightWhite => LIGHT_GRAY_BG,
        }.to_string()
    }

    pub fn fg(&self) -> String {
        match self {
            Black => color::Black.fg_str(),
            Red => color::Red.fg_str(),
            Green => color::Green.fg_str(),
            Yellow => color::Yellow.fg_str(),
            Blue => color::Blue.fg_str(),
            Magenta => color::Magenta.fg_str(),
            Cyan => color::Cyan.fg_str(),
            White => color::White.fg_str(),
            Gray => GRAY_FG,
            LightBlack => color::LightBlack.fg_str(),
            LightRed => color::LightRed.fg_str(),
            LightGreen => color::LightGreen.fg_str(),
            LightYellow => color::LightYellow.fg_str(),
            LightBlue => color::LightBlue.fg_str(),
            LightMagenta => color::LightMagenta.fg_str(),
            LightCyan => color::LightCyan.fg_str(),
            LightWhite => LIGHT_GRAY_FG,
        }.to_string()
    }
}