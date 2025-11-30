use crate::state::{Cursor, Mode, State};
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

                if let Mode::Edit = state.mode {
                    if x as usize == state.edit_cursor + 1 {
                        write!(
                            window,
                            "{}{}",
                            color::Bg(color::Black),
                            color::Fg(color::White)
                        );
                    } else if x as usize == state.edit_cursor + 2 {
                        write!(
                            window,
                            "{}{}",
                            color::Bg(Self::get_color(state)),
                            color::Fg(color::Black)
                        );
                    }
                }

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
            Mode::Save => &color::LightYellow,
        }
    }

    pub fn get_status_message(state: &State) -> String {
        match state.mode {
            Mode::Nav => match state.cursor {
                Cursor::Single((r, c)) => {
                    let cell = state.get_at((r, c));
                    format!(
                        "{}{} {}",
                        r + 1,
                        State::col_name(c as u8 + 1),
                        if cell.computed.error { &cell.computed.display } else { &cell.value }
                    )
                }
                Cursor::Row(r) => format!("{r}:{r}", r = r + 1),
                Cursor::Column(c) => format!("{c}:{c}", c = State::col_name(c as u8 + 1)),
            },
            Mode::Edit => format!("={}", &state.edit_buffer),
            Mode::Save => format!("Saving to ./{}", &state.edit_buffer),
        }
    }
}
