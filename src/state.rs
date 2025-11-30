use crate::cursor::Cursor;

pub struct State {
    pub mode: Mode,
    pub content: Vec<Vec<DisplayCell>>,
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
            content: vec![],
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
        let Cursor::Single((r, c)) = self.cursor else {
            panic!("Non-single cursor in EDIT mode");
        };

        let mut row = &mut self.get_row(r as usize);
        let prev_cell = State::get_col(&mut row, c as usize);
        let previous_value = prev_cell.value.clone();

        self.mode = Mode::Nav;

        self.undo_stack.push(vec![Action {
            addr: (r, c),
            previous_value,
            new_value: self.edit_buffer.clone(),
        }]);
        self.redo_stack.clear();

        self.set_at((r, c), DisplayCell::new(self.edit_buffer.clone()));
    }

    fn get_row(&mut self, r: usize) -> &mut Vec<DisplayCell> {
        while r >= self.content.len() {
            self.content.push(vec![])
        }

        &mut self.content[r]
    }

    fn get_col(row: &mut Vec<DisplayCell>, c: usize) -> &DisplayCell {
        while c >= row.len() {
            row.push(DisplayCell::blank())
        }

        &row[c]
    }

    pub fn get_at(&self, (row, col): Address) -> &DisplayCell {
        &self
            .content
            .get(row as usize)
            .and_then(|x| x.get(col as usize))
            .unwrap_or(&BLANK_CELL)
    }

    pub fn set_at(&mut self, (r, c): Address, cell: DisplayCell) {
        let row = &mut self.get_row(r as usize);
        let _ = State::get_col(row, c as usize);
        row[c as usize] = cell;
    }

    pub fn clear_at_cursor(&mut self) {
        // Track cleared values for undo
        let mut cleared_values: Vec<(Address, String)> = vec![];
        let mut clear_at = |state: &mut State, addr: Address| {
            let cell = state.get_at(addr);
            if !cell.is_empty() {
                cleared_values.push((addr, cell.value.clone()));
                state.set_at(addr, DisplayCell::blank())
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
                let columns = self.content[r as usize].len();
                for c in 0..columns {
                    clear_at(self, (r, c as u16))
                }
            }
            Cursor::Column(c) => {
                for r in 0..self.content.len() {
                    clear_at(self, (r as u16, c));
                }
            }
        }

        if !cleared_values.is_empty() {
            let actions: Vec<Action> = cleared_values
                .into_iter()
                .map(|(addr, previous_value)| Action {
                    addr,
                    previous_value,
                    new_value: String::new(),
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
            let (r, c) = action.addr;
            let row = &mut self.get_row(r as usize);
            row[c as usize].value = action.previous_value.clone();
        }

        self.redo_stack.push(actions);
    }

    pub fn redo(&mut self) {
        let Some(actions) = self.redo_stack.pop() else {
            return;
        };

        for action in &actions {
            let (r, c) = action.addr;
            let row = &mut self.get_row(r as usize);
            row[c as usize].value = action.new_value.clone();
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
    pub new_value: String,
}

static BLANK_CELL: DisplayCell = DisplayCell::blank();

#[derive(Clone)]
pub struct CellComputation {
    /// Has this value been computed yet or is it still pending?
    /// Can be true even when value is None
    pub is_computed: bool,
    pub error: bool,
    pub display: String,
    pub value: Option<f32>,
}

impl CellComputation {
    pub const fn new() -> Self {
        CellComputation {
            is_computed: false,
            error: false,
            display: String::new(),
            value: None,
        }
    }

    pub fn clear(&mut self) {
        self.is_computed = false;
        self.display.clear();
    }

    pub fn set_string(&mut self, value: String) {
        self.is_computed = true;
        self.error = false;
        // Convert to float with best effort
        self.value = (&value).parse::<f32>().ok();
        self.display = value;
    }

    pub fn set_error(&mut self, err: String) {
        self.is_computed = true;
        self.error = true;
        self.display = err;
        self.value = None;
    }

    pub fn set_computed(&mut self, value: f32) {
        self.is_computed = true;
        self.error = false;
        self.display = format!("{}", value);
        self.value = Some(value);
    }
}

#[derive(Clone)]
pub struct DisplayCell {
    pub alignment: Alignment,
    pub value: String,
    pub computed: CellComputation,
}

impl DisplayCell {
    pub const fn new(value: String) -> Self {
        DisplayCell {
            value,
            computed: CellComputation::new(),
            alignment: Alignment::Left,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    pub const fn blank() -> Self {
        DisplayCell::new(String::new())
    }

    pub fn with_alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }
}

#[derive(Clone)]
pub enum Alignment {
    Left,
    Right,
}
