use crate::cursor::Cursor;
use crate::keyboard::Key;
use crate::state::{Mode, State};

impl State {
    pub fn handle_input_nav_mode(&mut self, input: Key) {
        match input {
            // Save
            Key::Ctrl('s') => self.enter_save_mode(),

            // Undo/redo
            Key::Ctrl('z') => self.undo(),
            Key::Ctrl('y') => self.redo(),

            // Column width
            Key::Ctrl('=') => self.column_width(1),
            Key::Ctrl('+') => self.column_width(1),
            Key::Ctrl('-') => self.column_width(-1),

            // Start editing existing content
            Key::Ctrl('e') => self.input_edit_cell(),

            // Clear
            Key::Backspace => self.clear_at_cursor(),

            // Movement
            Key::Up => self.cursor = self.cursor.move_v(-1),
            Key::Down => self.cursor = self.cursor.move_v(1),
            Key::Left => self.cursor = self.cursor.move_h(-1),
            Key::Right => self.cursor = self.cursor.move_h(1),

            // Jump
            Key::AltUp => self.cursor = self.cursor.jump_v(-1, &self),
            Key::AltDown => self.cursor = self.cursor.jump_v(1, &self),
            Key::AltLeft => self.cursor = self.cursor.jump_h(1, &self),
            Key::AltRight => self.cursor = self.cursor.jump_h(-1, &self),

            // Select
            Key::ShiftUp => self.cursor = self.cursor.select_v(-1),
            Key::ShiftDown => self.cursor = self.cursor.select_v(1),
            Key::ShiftLeft => self.cursor = self.cursor.select_h(-1),
            Key::ShiftRight => self.cursor = self.cursor.select_h(1),

            // Select + Jump
            Key::AltShiftUp => self.cursor = self.cursor.select_jump_v(-1, &self),
            Key::AltShiftDown => self.cursor = self.cursor.select_jump_v(1, &self),
            Key::AltShiftLeft => self.cursor = self.cursor.select_jump_h(-1, &self),
            Key::AltShiftRight => self.cursor = self.cursor.select_jump_h(1, &self),

            // Handle return and tab
            Key::Enter => self.cursor = self.cursor.move_v(1),
            Key::Tab => self.cursor = self.cursor.move_h(1),

            // Or just start typing to overwrite the cell
            Key::Char(l) => self.input_overwrite_cell(l),

            // _ => { println!("Unhandled nav input: {:?}", input) }
            _ => {}
        }
    }

    fn input_edit_cell(&mut self) {
        if let Cursor::Single(addr) = self.cursor {
            self.mode = Mode::Edit;
            let edit_cell = self.sheet.cells.get_at(addr);
            self.edit_buffer = edit_cell.cloned().unwrap_or_default();
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

    fn column_width(&mut self, direction: i16) {
        if let Cursor::Single((_, c)) = self.cursor {
            let current_width = self.sheet.column_widths.get(c as usize).unwrap_or(&10);
            let new_width = current_width.saturating_add_signed(direction).max(1);
            self.sheet.column_widths[c as usize] = new_width;
        }
    }
}