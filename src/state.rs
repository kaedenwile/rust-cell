use std::sync::OnceLock;

pub struct State {
    pub mode: Mode,
    pub content: Vec<Vec<DisplayCell>>,
    pub scroll: Address,
    pub cursor: Cursor,
    pub edit_cursor: usize,
}

impl State {
    pub fn blank() -> Self {
        State {
            mode: Mode::Nav,
            content: Vec::new(),
            scroll: (0, 0),
            cursor: Cursor::Single((1, 1)),
            edit_cursor: 0,
        }
    }

    pub fn col_name(i: u8) -> String {
        ((('A' as u8 - 1) + (i % 26)) as char).to_string()
    }

    pub fn edit_at<F>(&mut self, (r, c): Address, f: F)
    where
        F: Fn(&DisplayCell) -> DisplayCell,
    {
        let mut row = &mut self.get_row(r as usize);
        let cell = State::get_col(&mut row, c as usize);

        row[c as usize] = f(cell)
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
            .unwrap_or(BLANK_CELL.get_or_init(|| DisplayCell::blank()))
    }

    pub fn set_at(&mut self, (r, c): Address, cell: DisplayCell) {
        let row = &mut self.get_row(r as usize);
        while c as usize >= row.len() {
            row.push(DisplayCell::blank())
        }

        row[c as usize] = cell;
    }
}

pub enum Mode {
    Nav,
    Edit,
}

pub type Address = (u16, u16);

static BLANK_CELL: OnceLock<DisplayCell> = OnceLock::new();

#[derive(Clone)]
pub struct CellComputation {
    pub is_computed: bool,
    pub error: bool,
    pub display: String,
    pub value: Option<f32>,
}

impl CellComputation {
    pub fn new() -> Self {
        CellComputation {
            is_computed: false,
            error: false,
            display: "".to_string(),
            value: None,
        }
    }

    pub fn clear(&mut self) {
        self.is_computed = false;
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
    pub fn new(value: String) -> Self {
        DisplayCell {
            value,
            computed: CellComputation::new(),
            alignment: Alignment::Left,
        }
    }

    pub fn blank() -> Self {
        DisplayCell::new("".to_string())
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
}
