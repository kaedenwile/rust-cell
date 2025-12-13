use crate::color::Color;
use crate::{rich_text, underline, RichText, RichTextStyle};
use crate::state::{Mode, State};
use crate::window::Window;

pub enum Menu {}

impl Menu {
    pub fn draw(window: &mut dyn Window, state: &State) {
        let (width, _) = window.size();
        let message = rich_text!(
            Menu::format_menu(state), "  ",
            Menu::edit_menu(state), "  ",
            Menu::save_menu(state), "  ",
            underline!("Q"), "uit"
        );

        for x in 0..width {
            let char = message.get_at(x as usize).cloned().unwrap_or_default();

            window.write_at(x, 0, char.ch, char.bg.unwrap_or(Color::Black), char.fg.unwrap_or(Color::White), char.bold, char.italic, char.underline);
        }
    }

    fn format_menu(state: &State) -> RichText {
        let title = rich_text!(underline!("F"), "ormat");
        match state.mode {
            Mode::Format => rich_text!(
                RichTextStyle::Background(Color::Pink),
                RichTextStyle::Foreground(Color::Black),
                title,
                RichTextStyle::ForegroundReset,
                RichTextStyle::BackgroundReset
            ),
            _ => title,
        }
    }

    fn edit_menu(state: &State) -> RichText {
        let title = rich_text!(underline!("E"), "dit");
        match state.mode {
            Mode::Edit => rich_text!(
                RichTextStyle::Background(Color::LightGreen),
                RichTextStyle::Foreground(Color::Black),
                title,
                RichTextStyle::ForegroundReset,
                RichTextStyle::BackgroundReset
            ),
            _ => title,
        }
    }

    fn save_menu(state: &State) -> RichText {
        let title = rich_text!(underline!("S"), "ave");
        match state.mode {
            Mode::Save => rich_text!(
                RichTextStyle::Background(Color::LightYellow),
                RichTextStyle::Foreground(Color::Black),
                title,
                RichTextStyle::ForegroundReset,
                RichTextStyle::BackgroundReset
            ),
            _ => title,
        }
    }
}
