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
        }
    }

    pub fn get_status_message(state: &State) -> String {
        match state.mode {
            Mode::Nav => format!(
                "Cursor: {}",
                match state.cursor {
                    Cursor::Single((r, c)) => {
                        let cell = state.get_at((r, c));
                        format!(
                            "{}{} ERR:{} {}",
                            r + 1,
                            State::col_name(c as u8 + 1),
                            cell.computed.error,
                            cell.computed.display
                        )
                    }
                    Cursor::Row(r) => format!("{r}:{r}", r = r + 1),
                    Cursor::Column(c) => format!("{c}:{c}", c = State::col_name(c as u8 + 1)),
                }
            ),
            Mode::Edit => format!(
                "={}",
                match state.cursor {
                    Cursor::Single(addr) => &state.get_at(addr).value,
                    _ => panic!("Editing with non-single select!"),
                },
            ),
        }
    }
}
