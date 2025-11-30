use crate::state::{Cursor, Mode, State};
use termion::event::Key;

impl State {
    pub fn handle_input_nav_mode(&mut self, input: Key) {
        match input {
            Key::Ctrl('s') => self.enter_save_mode(),
            Key::Backspace => self.input_clear_cell(),

            Key::Ctrl('z') => self.undo(),
            Key::Ctrl('y') => self.redo(),

            // Movement
            Key::Up => self.cursor = self.cursor.move_v(-1),
            Key::Down => self.cursor = self.cursor.move_v(1),
            Key::Left => self.cursor = self.cursor.move_h(-1),
            Key::Right => self.cursor = self.cursor.move_h(1),
            Key::Char('\n') => self.cursor = self.cursor.move_v(1),
            Key::Char('\t') => self.cursor = self.cursor.move_h(1),

            // Jump
            Key::AltUp => self.cursor = self.cursor.jump_v(-1, &self),
            Key::AltDown => self.cursor = self.cursor.jump_v(1, &self),
            // Ghostty treats AltLeft as Alt-f and AltRight as Alt-b
            Key::Alt('f') => self.cursor = self.cursor.jump_h(1, &self),
            Key::Alt('b') => self.cursor = self.cursor.jump_h(-1, &self),

            // Start editing
            Key::Ctrl('e') => self.input_edit_cell(),

            // Or just start typing to overwrite the cell
            Key::Char(l) => self.input_overwrite_cell(l),

            _ => {}
        }
    }

    fn input_clear_cell(&mut self) {
        if let Cursor::Single(addr) = self.cursor {
            self.clear_at(addr);
        }
    }

    fn input_edit_cell(&mut self) {
        if let Cursor::Single(addr) = self.cursor {
            self.mode = Mode::Edit;
            let edit_cell = &self.get_at(addr);
            self.edit_buffer = edit_cell.value.clone();
            self.edit_cursor = self.edit_buffer.len();
        }
    }

    fn input_overwrite_cell(&mut self, ch: char) {
        if let Cursor::Single(addr) = self.cursor {
            self.mode = Mode::Edit;
            self.edit_buffer = ch.to_string();
            self.edit_cursor = 1;
        }
    }
}