use crate::color::Color;
use std::rc::Rc;

#[derive(Clone, Copy)]
pub struct RichTextChar {
    pub ch: char,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub fg: Option<Color>,
    pub bg: Option<Color>,
}

impl Default for RichTextChar {
    fn default() -> RichTextChar {
        RichTextChar {
            ch: ' ',
            bold: false,
            italic: false,
            underline: false,
            fg: None,
            bg: None,
        }
    }
}


pub enum RichTextStyle {
    Bold,
    NoBold,
    Italic,
    NoItalic,
    Underline,
    NoUnderline,
    Foreground(Color),
    ForegroundReset,
    Background(Color),
    BackgroundReset,
}

#[derive(Clone)]
pub struct RichText {
    rich_text_chars: Rc<[RichTextChar]>,
}

impl RichText {
    pub fn new(words: Vec<RichTextWord>) -> RichText {
        let mut rich_text_chars = Vec::new();
        let mut current_bold = false;
        let mut current_italic = false;
        let mut current_underline = false;
        let mut current_fg: Option<Color> = None;
        let mut current_bg: Option<Color> = None;

        for word in words {
            match word {
                RichTextWord::Text(text) => {
                    for ch in text.chars() {
                        rich_text_chars.push(RichTextChar {
                            ch,
                            bold: current_bold,
                            italic: current_italic,
                            underline: current_underline,
                            fg: current_fg,
                            bg: current_bg,
                        });
                    }
                }
                RichTextWord::Style(style) => {
                    match style {
                        RichTextStyle::Bold => current_bold = true,
                        RichTextStyle::NoBold => current_bold = false,
                        RichTextStyle::Italic => current_italic = true,
                        RichTextStyle::NoItalic => current_italic = false,
                        RichTextStyle::Underline => current_underline = true,
                        RichTextStyle::NoUnderline => current_underline = false,
                        RichTextStyle::Foreground(color) => current_fg = Some(color),
                        RichTextStyle::ForegroundReset => current_fg = None,
                        RichTextStyle::Background(color) => current_bg = Some(color),
                        RichTextStyle::BackgroundReset => current_bg = None,
                    }
                }
            }
        }

        RichText { rich_text_chars: Rc::from(rich_text_chars) }
    }

    pub fn empty() -> RichText {
        RichText { rich_text_chars: Rc::from([]) }
    }

    pub fn get_at(&self, index: usize) -> Option<&RichTextChar> {
        self.rich_text_chars.get(index)
    }
}

impl Default for RichText {
    fn default() -> RichText {
        RichText::empty()
    }
}

pub enum RichTextWord {
    Text(String),
    Style(RichTextStyle),
}

pub trait RichTextWordConversion {
    fn rich_text_word(self) -> RichTextWord;
}

impl RichTextWordConversion for &str {
    fn rich_text_word(self) -> RichTextWord {
        RichTextWord::Text(self.to_string())
    }
}

impl RichTextWordConversion for RichTextStyle {
    fn rich_text_word(self) -> RichTextWord {
        RichTextWord::Style(self)
    }
}

#[macro_export]
macro_rules! rich_text {
    ($($word:expr),* $(,)?) => {
        {
            let mut words = Vec::new();
            $(
                words.push($crate::RichTextWordConversion::rich_text_word($word));
            )*
            $crate::RichText::new(words)
        }
    };
}