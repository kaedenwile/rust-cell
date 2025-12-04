use crate::filesystem::save;
use crate::keyboard::Key;
use crate::state::{Mode, State};

impl State {
    pub fn enter_save_mode(&mut self) {
        self.mode = Mode::Save;
        self.edit_buffer = self.filename.clone();
        self.edit_cursor = self.edit_buffer.len();
    }

    pub fn handle_input_save_mode(&mut self, input: Key) {
        let state = self;
        match input {
            Key::Enter => {
                state.filename = state.edit_buffer.clone();
                save(&state, &state.edit_buffer);
                state.mode = Mode::Nav;
            }
            Key::Esc => state.mode = Mode::Nav,

            Key::Char(l) => {
                state.edit_buffer.insert(state.edit_cursor, l);
                state.edit_cursor += 1;
            }

            Key::Backspace => {
                if state.edit_cursor == 0 {
                    return;
                }
                state.edit_buffer.remove(state.edit_cursor - 1);
                state.edit_cursor -= 1;
            }
            _ => {}
        }
    }
}