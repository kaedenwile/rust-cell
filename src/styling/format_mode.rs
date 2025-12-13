use std::cmp::min;
use crate::color::Color;
use crate::keyboard::Key;
use crate::state::{Mode, State};
use crate::styling::{CellStyles, StylingState};

impl State {
    pub fn enter_format_mode(&mut self) {
        self.mode = Mode::Format;

        self.styling_state = StylingState::from_cell_styles(
            self.sheet.cell_styling.get_at(self.cursor.anchor()).cloned().unwrap_or_default()
        );
    }

    pub fn handle_input_format_mode(&mut self, input: Key) {
        let state = self;
        match input {
            Key::Enter => {
                state.save_styling_to_cells();
                state.mode = Mode::Nav
            }
            Key::Esc => state.mode = Mode::Nav,

            Key::Up => state.styling_state.cursor = state.styling_state.cursor.saturating_sub(1),
            Key::Down => state.styling_state.cursor = min(4, state.styling_state.cursor + 1),

            Key::Char(' ') => state.toggle_selected_styling_option(),

            _ => {}
        }
    }

    pub fn toggle_selected_styling_option(&mut self) {
        match self.styling_state.cursor {
            0 => self.styling_state.bold = !self.styling_state.bold,
            1 => self.styling_state.italic = !self.styling_state.italic,
            2 => self.styling_state.underline = !self.styling_state.underline,
            3 => self.styling_state.fg = Color::Blue,
            4 => self.styling_state.bg = Color::LightGreen,
            _ => {}
        }
    }

    pub fn save_styling_to_cells(&mut self) {
        for addr in self.iter_cursor() {
            let styles = CellStyles {
                bold: self.styling_state.bold,
                italic: self.styling_state.italic,
                underline: self.styling_state.underline,
                fg: self.styling_state.fg,
                bg: self.styling_state.bg,
            };
            self.sheet.cell_styling.set_at(addr, styles);
        }
    }
}