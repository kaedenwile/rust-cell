use crate::cursor::Cursor;
use crate::keyboard::Key;
use crate::state::{Mode, State};

impl State {
    pub fn handle_input_edit_mode(&mut self, input: Key) {
        let Cursor::Single(addr) = self.cursor else {
            panic!("DEV ERROR: Non-single cursor in EDIT mode");
        };

        match input {
            Key::Enter => {
                self.save_edits();
                self.cursor = self.cursor.move_v(1);
            }
            Key::Tab => {
                self.save_edits();
                self.cursor = self.cursor.move_h(1);
            }
            Key::Esc => self.mode = Mode::Nav,

            // Terminal shortcuts
            Key::Ctrl('a') => self.edit_cursor = 0,
            Key::Ctrl('e') => self.edit_cursor = self.edit_buffer.len(),
            Key::Alt('f') => self.input_jump_word_forward(),
            Key::Alt('b') => self.input_jump_word_backward(),

            // Edit text
            Key::Char(l) => {
                self.edit_buffer.insert(self.edit_cursor, l);
                self.edit_cursor += 1;
            }

            Key::Backspace => {
                if self.edit_cursor == 0 {
                    return;
                }
                self.edit_buffer.remove(self.edit_cursor - 1);
                self.edit_cursor -= 1;
            }

            Key::Left if self.edit_cursor > 0 => self.edit_cursor -= 1,
            Key::Right if self.edit_cursor < self.edit_buffer.len() => {
                self.edit_cursor += 1
            }

            _ => {}
        }
    }

    fn input_jump_word_forward(&mut self) {
        self.edit_cursor = self.edit_buffer[self.edit_cursor..]
            .find(" ")
            .and_then(|idx| Some(idx + self.edit_cursor + 1))
            .unwrap_or(self.edit_buffer.len())
    }

    fn input_jump_word_backward(&mut self) {
        self.edit_cursor = self.edit_buffer[..self.edit_cursor]
            .rfind(" ")
            .and_then(|idx| if idx == 0 { None } else { Some(idx - 1) })
            .unwrap_or(0)
    }
}