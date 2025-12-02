use crate::color::Color;
use crate::cursor::Cursor;
use crate::state::{Mode, State};
use crate::window::Window;

pub enum StatusBar {}

impl StatusBar {
    pub fn draw(window: &mut dyn Window, state: &State) {
        let (width, _) = window.size();

        let status_message = Self::get_status_message(state);
        let bg = Self::get_color(state);

        for x in 0..width {
            let c = status_message.chars().nth(x as usize).unwrap_or(' ');

            if matches!(state.mode, Mode::Edit | Mode::Save) {
                // Edit offset is 0, Save offset is length of "Saving to ./"
                let offset = match state.mode {
                    Mode::Edit => 0,
                    Mode::Save => 12,
                    _ => 0,
                };

                // Implement a cursor by inverting the colors at the cursor position
                if x as usize == state.edit_cursor + offset {
                    window.write_at(x, 0, c, Color::Black, Color::White);
                    continue;
                }
            }

            window.write_at(x, 0, c, bg, Color::Black);
        }
    }

    pub fn get_color(State { mode, .. }: &State) -> Color {
        match mode {
            Mode::Nav => Color::LightBlue,
            Mode::Edit => Color::LightGreen,
            Mode::Save => Color::LightYellow,
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
                // TODO improve range display
                Cursor::Range((r, c), _) => {
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
            Mode::Edit => state.edit_buffer.to_string(),
            Mode::Save => format!("Saving to ./{}", &state.edit_buffer),
        }
    }
}
