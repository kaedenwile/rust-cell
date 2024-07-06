pub struct Sheet {
    pub content: Vec<Vec<DisplayCell>>,
    pub scroll: Address,
    pub cursor: Cursor,
}

impl Sheet {
    pub fn blank(size: u8) -> Self {
        let mut sheet = Sheet {
            content: Vec::new(),
            scroll: (0, 0),
            cursor: Cursor::Single((1, 1)),
        };

        // HEADER ROW
        let mut header_row = vec![DisplayCell::blank()];
        for i in 1..size {
            header_row.push(DisplayCell::new(Sheet::col_name(i - 1)))
        }
        sheet.content.push(header_row);

        // BODY ROWS
        for i in 1..size {
            sheet.content.push(vec![
                DisplayCell::new(i.to_string()).with_alignment(Alignment::Right)
            ])
        }

        sheet
    }

    pub fn col_name(i: u8) -> String {
        ((('A' as u8 - 1) + (i % 26)) as char).to_string()
    }

    pub fn edit_at<F>(&mut self, (r, c): Address, f: F)
    where
        F: Fn(&DisplayCell) -> DisplayCell,
    {
        let mut row = &mut self.get_row(r as usize);
        let cell = Sheet::get_col(&mut row, c as usize);

        row[c as usize] = f(cell)
    }

    fn get_row(&mut self, r: usize) -> &mut Vec<DisplayCell> {
        while r >= self.content.len() {
            self.content
                .push(vec![DisplayCell::new(self.content.len().to_string())])
        }

        &mut self.content[r]
    }

    fn get_col(row: &mut Vec<DisplayCell>, c: usize) -> &DisplayCell {
        while c >= row.len() {
            row.push(DisplayCell::blank())
        }

        &row[c]
    }
}

pub type Address = (u16, u16);

pub struct DisplayCell {
    pub value: String,
    pub alignment: Alignment,
}

impl DisplayCell {
    pub fn new(value: String) -> Self {
        DisplayCell {
            value,
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

pub enum Alignment {
    Left,
    Right,
}

pub enum Cursor {
    Single(Address),
    // Range(Address, Address),
}

impl Cursor {
    pub fn move_h(&self, direction: i16) -> Self {
        let Cursor::Single((r, c)) = self;
        Cursor::Single((*r, c.saturating_add_signed(direction)))
    }

    pub fn move_v(&self, direction: i16) -> Self {
        let Cursor::Single((r, c)) = self;
        Cursor::Single((r.saturating_add_signed(direction), *c))
    }
}
