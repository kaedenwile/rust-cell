use termion::color;

#[derive(Clone, Copy, PartialEq)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Pink,
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
    CursorWhite,
}
use Color::*;

static CURSOR_WHITE_FG: &str = "\x1B[38;2;228;228;228m";
static CURSOR_WHITE_BG: &str = "\x1B[48;2;228;228;228m";

static LIGHT_GRAY_FG: &str = "\x1B[38;2;220;220;220m";
static LIGHT_GRAY_BG: &str = "\x1B[48;2;220;220;220m";

static GRAY_FG: &str = "\x1B[38;2;150;150;150m";
static GRAY_BG: &str = "\x1B[48;2;150;150;150m";

// static LIGHT_MAGENTA_FG: &str = "\x1B[38;2;150;150;150m";
static PINK_FG: &str = "\x1B[38;2;255;100;150m";
static PINK_BG: &str = "\x1B[48;2;255;150;200m";

// Used for serializing colors as u8
static ALL_COLORS: [Color; 19] = [
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Pink,
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
    CursorWhite
];
static COLOR_ORDER: [Color; 10] = [
    Black,
    Red,
    Magenta,
    Blue,
    Cyan,
    Yellow,
    Green,
    LightBlack,
    LightWhite,
    White,
];

impl Color {
    pub fn name(&self) -> &'static str {
        match self {
            Black => "Black",
            Red => "Red",
            Green => "Green",
            Yellow => "Yellow",
            Blue => "Blue",
            Magenta => "Magenta",
            Cyan => "Cyan",
            White => "White",
            Gray => "Gray",
            Pink => "Pink",
            LightBlack => "Light Black",
            LightRed => "Light Red",
            LightGreen => "Light Green",
            LightYellow => "Light Yellow",
            LightBlue => "Light Blue",
            LightMagenta => "Light Magenta",
            LightCyan => "Light Cyan",
            LightWhite => "Light White",
            CursorWhite => "Cursor White",
        }
    }

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
            Pink => PINK_BG,
            LightBlack => color::LightBlack.bg_str(),
            LightRed => color::LightRed.bg_str(),
            LightGreen => color::LightGreen.bg_str(),
            LightYellow => color::LightYellow.bg_str(),
            LightBlue => color::LightBlue.bg_str(),
            LightMagenta => color::LightMagenta.bg_str(),
            LightCyan => color::LightCyan.bg_str(),
            LightWhite => LIGHT_GRAY_BG,
            CursorWhite => CURSOR_WHITE_BG,
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
            Pink => PINK_FG,
            LightBlack => color::LightBlack.fg_str(),
            LightRed => color::LightRed.fg_str(),
            LightGreen => color::LightGreen.fg_str(),
            LightYellow => color::LightYellow.fg_str(),
            LightBlue => color::LightBlue.fg_str(),
            LightMagenta => color::LightMagenta.fg_str(),
            LightCyan => color::LightCyan.fg_str(),
            LightWhite => LIGHT_GRAY_FG,
            CursorWhite => CURSOR_WHITE_FG,
        }.to_string()
    }

    pub fn next_color(&self) -> Color {
        for (i, c) in COLOR_ORDER.iter().enumerate() {
            if c == self {
                return COLOR_ORDER[(i + 1) % COLOR_ORDER.len()];
            }
        }
        Black
    }

    pub fn prev_color(&self) -> Color {
        for (i, c) in COLOR_ORDER.iter().enumerate() {
            if c == self {
                return COLOR_ORDER[(i + COLOR_ORDER.len() - 1) % COLOR_ORDER.len()];
            }
        }
        Black
    }

    pub fn contrast_color(&self) -> Color {
        match self {
            Black | LightBlack => White,
            _ => Black,
        }
    }

    pub fn to_u8(&self) -> u8 {
        for (i, c) in ALL_COLORS.iter().enumerate() {
            if c == self {
                return i as u8;
            }
        }
        0
    }

    pub fn from_u8(value: u8) -> Option<Color> {
        ALL_COLORS.get(value as usize).copied()
    }
}