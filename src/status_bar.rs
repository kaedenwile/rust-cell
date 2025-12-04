use crate::color::Color;
use crate::cursor::Cursor;
use crate::state::{Address, Mode, State};
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
                Cursor::Single((r, c)) =>
                    format!(
                        "{}{} {}",
                        r + 1,
                        State::col_name(c as u8 + 1),
                        StatusBar::cell_display_value(state, (r, c))
                    ),
                // TODO improve range display
                Cursor::Range((r, c), _) =>
                    format!(
                        "{}{} {}",
                        r + 1,
                        State::col_name(c as u8 + 1),
                        StatusBar::cell_display_value(state, (r, c))
                    ),
                Cursor::Row(r) => format!("{r}:{r}", r = r + 1),
                Cursor::Column(c) => format!("{c}:{c}", c = State::col_name(c as u8 + 1)),
            },
            Mode::Edit => state.edit_buffer.to_string(),
            Mode::Save => format!("Saving to ./{}", &state.edit_buffer),
        }
    }

    fn cell_display_value(state: &State, addr: Address) -> &str {
        if let Some(value) = state.sheet.cells.get_at(addr) {
            if let Some(cell) = state.sheet.rendered_cells.get_at(addr) {
                if cell.error {
                    &cell.display
                } else {
                    value
                }
            } else {
                value
            }
        } else {
            ""
        }
    }
}
