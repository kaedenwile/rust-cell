use crate::keyboard::Key;
use crate::state::{Mode, State};
use crate::styling::CellStyles;

impl State {
    pub fn enter_format_mode(&mut self) {
        self.mode = Mode::Format;

        for addr in self.iter_cursor() {
            self.sheet.cell_styling.set_at(addr, CellStyles::colorful())
        }
    }

    pub fn handle_input_format_mode(&mut self, input: Key) {
        let state = self;
        match input {
            Key::Enter => state.mode = Mode::Nav,
            Key::Esc => state.mode = Mode::Nav,
            _ => {}
        }
    }
}