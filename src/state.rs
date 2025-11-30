pub struct State {
    pub mode: Mode,
    pub content: Vec<Vec<DisplayCell>>,
    pub scroll: Address,
    pub cursor: Cursor,

    pub edit_buffer: String,
    pub edit_cursor: usize,

    pub filename: String,

    pub undo_stack: Vec<Action>,
    pub redo_stack: Vec<Action>,
}

impl State {
    pub fn blank() -> Self {
        State {
            mode: Mode::Nav,
            content: Vec::new(),
            scroll: (0, 0),
            cursor: Cursor::Single((1, 1)),

            edit_buffer: String::new(),
            edit_cursor: 0,

            filename: String::new(),

            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
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

        &self.undo_stack.push(Action {
            addr: (r, c),
            previous_value,
            new_value: self.edit_buffer.clone(),
        });
        &self.redo_stack.clear();

        self.set_at((r, c), DisplayCell::new(self.edit_buffer.clone()));
    }

    // Helper function that calls set_at under the hood.
    // Also updates undo/redo stack
    pub fn edit_at<F>(&mut self, (r, c): Address, f: F)
    where
        F: Fn(&DisplayCell) -> DisplayCell,
    {
        let mut row = &mut self.get_row(r as usize);

        let prev_cell = State::get_col(&mut row, c as usize);
        let previous_value = prev_cell.value.clone();

        let new_cell = f(prev_cell);
        let new_value = new_cell.value.clone();

        &self.undo_stack.push(Action {
            addr: (r, c),
            previous_value,
            new_value,
        });
        &self.redo_stack.clear();

        self.set_at((r, c), new_cell);
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

    pub fn clear_at(&mut self, addr: Address) {
        self.set_at(addr, DisplayCell::blank())
    }

    pub fn undo(&mut self) {
        let Some(action) = self.undo_stack.pop() else {
            return;
        };

        let (r, c) = action.addr;
        let row = &mut self.get_row(r as usize);
        row[c as usize].value = action.previous_value.clone();

        self.redo_stack.push(action);
    }

    pub fn redo(&mut self) {
        let Some(action) = self.redo_stack.pop() else {
            return;
        };

        let (r, c) = action.addr;
        let row = &mut self.get_row(r as usize);
        row[c as usize].value = action.new_value.clone();

        self.undo_stack.push(action);
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

static MAX_ROW: u16 = 1000;
static MAX_COL: u16 = 1000;

#[derive(Clone)]
pub enum Alignment {
    Left,
    Right,
}

#[derive(Clone)]
pub enum Cursor {
    Single(Address),
    // Range(Address, Address),
    Row(u16),
    Column(u16),
}

impl Cursor {
    pub fn move_h(&self, direction: i16) -> Self {
        match self {
            Cursor::Single((r, c)) if direction < 0 && *c == 0 => Cursor::Row(*r),
            Cursor::Single((r, c)) => Cursor::Single((*r, c.saturating_add_signed(direction))),
            Cursor::Row(r) if direction < 0 => Cursor::Row(*r), // copy of self
            Cursor::Row(r) => Cursor::Single((*r, 0)),
            Cursor::Column(c) => Cursor::Column(c.saturating_add_signed(direction)),
        }
    }

    pub fn move_v(&self, direction: i16) -> Self {
        match self {
            Cursor::Single((r, c)) if direction < 0 && *r == 0 => Cursor::Column(*c),
            Cursor::Single((r, c)) => Cursor::Single((r.saturating_add_signed(direction), *c)),
            Cursor::Row(r) => Cursor::Row(r.saturating_add_signed(direction)),
            Cursor::Column(c) if direction < 0 => Cursor::Column(*c), // copy of self
            Cursor::Column(c) => Cursor::Single((0, *c)),
        }
    }

    pub fn jump_h(&self, direction: i16, state: &State) -> Self {
        let Cursor::Single((r, c)) = *self else { return self.clone(); };

        // Short circuit if we're already in the first col
        if direction < 0 && c == 0 { return self.clone(); }

        let start_is_empty = state.get_at((r, c)).is_empty();
        let mut prev_col = c;
        let mut prev_is_empty = start_is_empty;

        loop {
            let next_col = prev_col.saturating_add_signed(direction);
            let next_is_empty = state.get_at((r, next_col)).is_empty();

            if start_is_empty && prev_is_empty && !next_is_empty {
                return Cursor::Single((r, next_col));
            } else if !start_is_empty && prev_is_empty && !next_is_empty {
                return Cursor::Single((r, next_col));
            } else if prev_col != c && !start_is_empty && !prev_is_empty && next_is_empty {
                return Cursor::Single((r, prev_col))
            }

            if direction < 0 && prev_col == 1 {
                return Cursor::Single((r, 0));
            } else if direction > 0 && next_col == MAX_COL {
                return Cursor::Single((r, MAX_COL - 1));
            }

            prev_col = next_col;
            prev_is_empty = next_is_empty;
        }
    }
    pub fn jump_v(&self, direction: i16, state: &State) -> Self {
        let Cursor::Single((r, c)) = *self else { return self.clone(); };

        // Short circuit if we're already in the first row
        if direction < 0 && r == 0 { return self.clone(); }

        let start_is_empty = state.get_at((r, c)).is_empty();
        let mut prev_row = r;
        let mut prev_is_empty = start_is_empty;

        loop {
            let next_row = prev_row.saturating_add_signed(direction);
            let next_is_empty = state.get_at((next_row, c)).is_empty();

            if start_is_empty && prev_is_empty && !next_is_empty {
                return Cursor::Single((next_row, c));
            } else if !start_is_empty && prev_is_empty && !next_is_empty {
                return Cursor::Single((next_row, c));
            } else if prev_row != r && !start_is_empty && !prev_is_empty && next_is_empty {
                return Cursor::Single((prev_row, c))
            }

            if direction < 0 && prev_row == 1 {
                return Cursor::Single((0, c));
            } else if direction > 0 && next_row == MAX_ROW {
                return Cursor::Single((MAX_ROW - 1, c));
            }

            prev_row = next_row;
            prev_is_empty = next_is_empty;
        }
    }
}
