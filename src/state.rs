use crate::cursor::Cursor;
use crate::sheet::Sheet;

/// The current active state of the program
pub struct State {
    pub mode: Mode,
    pub sheet: Sheet,

    pub scroll: Address,
    pub cursor: Cursor,

    pub edit_buffer: String,
    pub edit_cursor: usize,

    pub filename: String,

    pub undo_stack: Vec<Vec<Action>>,
    pub redo_stack: Vec<Vec<Action>>,
}

impl State {
    pub fn blank() -> Self {
        State {
            mode: Mode::Nav,
            sheet: Sheet::new(),
            scroll: (0, 0),
            cursor: Cursor::Single((0, 0)),

            edit_buffer: String::new(),
            edit_cursor: 0,

            filename: String::new(),

            undo_stack: vec![],
            redo_stack: vec![],
        }
    }

    pub fn col_name(i: u8) -> String {
        ((('A' as u8 - 1) + (i % 26)) as char).to_string()
    }

    pub fn save_edits(&mut self) {
        let Mode::Edit = self.mode else {
            panic!("Called finish_edit() on a non-edit Mode ");
        };
        let Cursor::Single(addr) = self.cursor else {
            panic!("Non-single cursor in EDIT mode");
        };

        self.mode = Mode::Nav;

        let previous_value = self.sheet.cells.get_at(addr).cloned().unwrap_or(String::new());
        self.undo_stack.push(vec![Action {
            addr,
            previous_value,
        }]);
        self.redo_stack.clear();

        self.sheet.cells.set_at(addr, self.edit_buffer.clone());
    }

    pub fn clear_at_cursor(&mut self) {
        // Track cleared values for undo
        let mut cleared_values: Vec<(Address, String)> = vec![];
        let mut clear_at = |state: &mut State, addr: Address| {
            if let Some(value) = state.sheet.cells.get_at(addr) {
                if !value.is_empty() {
                    cleared_values.push((addr, value.clone()));
                    state.sheet.cells.set_at(addr, String::new())
                }
            }
        };

        match self.cursor {
            Cursor::Single(addr) => clear_at(self, addr),
            Cursor::Range(start, end) => {
                let (l, r, t, b) = Cursor::bounds(start, end);
                for y in t..=b {
                    for x in l..=r {
                        clear_at(self, (y, x))
                    }
                }
            }
            Cursor::Row(r) => {
                for c in 0..self.sheet.cells.num_columns_in_row(r) {
                    clear_at(self, (r, c))
                }
            }
            Cursor::Column(c) => {
                for r in 0..self.sheet.cells.num_rows() {
                    clear_at(self, (r, c));
                }
            }
        }

        if !cleared_values.is_empty() {
            let actions: Vec<Action> = cleared_values
                .into_iter()
                .map(|(addr, previous_value)| Action {
                    addr,
                    previous_value,
                })
                .collect();
            self.undo_stack.push(actions);
            self.redo_stack.clear();
        }
    }

    pub fn undo(&mut self) {
        let Some(actions) = self.undo_stack.pop() else {
            return;
        };

        for action in &actions {
            self.sheet.cells.set_at(action.addr, action.previous_value.clone());
        }

        self.redo_stack.push(actions);
    }

    pub fn redo(&mut self) {
        let Some(actions) = self.redo_stack.pop() else {
            return;
        };

        for action in &actions {
            self.sheet.cells.set_at(action.addr, action.previous_value.clone());
        }

        self.undo_stack.push(actions);
    }
}

pub enum Mode {
    Nav,
    Edit,
    Save,
}

/// A cell location in the spreadsheet (row, column)
pub type Address = (u16, u16);

#[derive(Debug)]
pub struct Action {
    pub addr: Address,
    pub previous_value: String,
}
