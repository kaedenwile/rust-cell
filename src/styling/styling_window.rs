use crate::color::Color::{Black, Gray, LightWhite};
use crate::{bold, rich_text, underline};
use crate::rich_text::RichTextChar;
use crate::state::State;
use crate::window::Window;
use crate::RichTextStyle;

pub enum StylingWindow {}

pub static STYLING_WINDOW_WIDTH: u16 = 32;
static PADDING: u16 = 2;

impl StylingWindow {
    pub fn draw(window: &mut dyn Window, State { styling_state, .. }: &State) {
        let (width, height) = window.size();

        let lines = [
            rich_text!(""),
            bold!("Styling Options"),
            rich_text!(""),
            rich_text!("[", if styling_state.bold { "B" } else { " " }, "] ", underline!("B"), "old"),
            rich_text!("[", if styling_state.italic { "I" } else { " " }, "] ", underline!("I"), "talic"),
            rich_text!("[", if styling_state.underline { "U" } else { " " }, "] ", underline!("U"), "nderline"),
            rich_text!(""),
            rich_text!("Foreground: ",
                RichTextStyle::Foreground(styling_state.fg),
                // RichTextStyle::Background(state.styling_state.fg.contrast_color()), // TODO if fg is gray
                styling_state.fg.name(),
                RichTextStyle::ForegroundReset,
                // RichTextStyle::BackgroundReset
            ),
            rich_text!("Background: ",
                RichTextStyle::Foreground(styling_state.bg.contrast_color()),
                RichTextStyle::Background(styling_state.bg),
                styling_state.bg.name(),
                RichTextStyle::ForegroundReset,
                RichTextStyle::BackgroundReset
            ),
        ];

        for y in 0..height {
            for x in 0..width {
                let ch = if x < PADDING || x > width - PADDING {
                    RichTextChar::default()
                } else if y as usize >= lines.len() {
                    RichTextChar::default()
                } else {
                    lines[y as usize].get_at((x - PADDING) as usize).copied().unwrap_or_default()
                };

                let is_selected = match y {
                    3 if styling_state.cursor == 0 => true,
                    4 if styling_state.cursor == 1 => true,
                    5 if styling_state.cursor == 2 => true,
                    7 if styling_state.cursor == 3 => true,
                    8 if styling_state.cursor == 4 => true,
                    _ => false,
                };

                let (fg, bg) = if is_selected {
                    (ch.fg.unwrap_or(Black), ch.bg.unwrap_or(LightWhite))
                } else {
                    (ch.fg.unwrap_or(Black), ch.bg.unwrap_or(Gray))
                };

                window.write_at(x, y, ch.ch, bg, fg, ch.bold, ch.italic, ch.underline);
            }
        }
    }
}
