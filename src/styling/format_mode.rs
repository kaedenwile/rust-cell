use std::cmp::min;
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

            Key::Right => match state.styling_state.cursor {
                3 => state.fg_next_color(),
                4 => state.bg_next_color(),
                _ => {}
            },
            Key::Left => match state.styling_state.cursor {
                3 => state.fg_prev_color(),
                4 => state.bg_prev_color(),
                _ => {}
            },

            Key::Char(' ') => state.toggle_selected_styling_option(),
            Key::Char('b') => state.toggle_bold(),
            Key::Char('i') => state.toggle_italic(),
            Key::Char('u') => state.toggle_underline(),

            _ => {}
        }
    }

    fn toggle_bold(&mut self) {
        self.styling_state.bold = !self.styling_state.bold;
    }

    fn toggle_italic(&mut self) {
        self.styling_state.italic = !self.styling_state.italic;
    }

    fn toggle_underline(&mut self) {
        self.styling_state.underline = !self.styling_state.underline;
    }

    fn fg_next_color(&mut self) {
        self.styling_state.fg = self.styling_state.fg.next_color()
    }

    fn fg_prev_color(&mut self) {
        self.styling_state.fg = self.styling_state.fg.prev_color()
    }

    fn bg_next_color(&mut self) {
        self.styling_state.bg = self.styling_state.bg.next_color()
    }

    fn bg_prev_color(&mut self) {
        self.styling_state.bg = self.styling_state.bg.prev_color()
    }

    fn toggle_selected_styling_option(&mut self) {
        match self.styling_state.cursor {
            0 => self.toggle_bold(),
            1 => self.toggle_italic(),
            2 => self.toggle_underline(),
            3 => self.fg_next_color(),
            4 => self.bg_next_color(),
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