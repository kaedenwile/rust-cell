use crate::state::{Cursor, DisplayCell, Mode, State};
use crate::window::Window;
use termion::color;
use termion::color::Color;

pub enum StatusBar {}

impl StatusBar {
    pub fn draw(window: &mut dyn Window, state: &State) {
        let (width, height) = window.size();

        write!(
            window,
            "{}{}",
            color::Bg(Self::get_color(state)),
            color::Fg(color::Black)
        );

        let status_message = Self::get_status_message(state);

        for y in 0..height {
            window.go_to(1, y + 1);

            for x in 0..width {
                let mut chars = status_message.chars();
                write!(
                    window,
                    "{}",
                    chars.nth(x as usize).unwrap_or(' ').to_string()
                )
            }
        }
    }

    pub fn get_color(State { mode, .. }: &State) -> &dyn Color {
        match mode {
            Mode::Nav => &color::LightBlue,
            Mode::Edit => &color::LightGreen,
        }
    }

    pub fn get_status_message(
        State {
            mode,
            content,
            cursor,
            ..
        }: &State,
    ) -> String {
        let blank_cell = DisplayCell::blank();

        match mode {
            Mode::Nav => format!(
                "Cursor: {}",
                match cursor {
                    Cursor::Single((r, c)) => format!("{}{}", r + 1, State::col_name(*c as u8 + 1)),
                    Cursor::Row(r) => format!("{r}:{r}", r = r + 1),
                    Cursor::Column(c) => format!("{c}:{c}", c = State::col_name(*c as u8 + 1)),
                }
            ),
            Mode::Edit => format!(
                "={}",
                match cursor {
                    Cursor::Single((r, c)) =>
                        &content
                            .get((*r) as usize)
                            .and_then(|x| x.get((*c) as usize))
                            .unwrap_or(&blank_cell)
                            .value,
                    _ => panic!("Editing with non-single select!"),
                }
            ),
        }
    }
}
