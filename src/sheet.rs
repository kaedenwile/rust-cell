use crate::compute::CellComputation;

/// A spreadsheet. Data is stored as sparsely as possible.
pub struct Sheet {
    pub cells: Grid<String>,
    pub rendered_cells: Grid<CellComputation>,
    pub dirty_cells: Vec<(u16, u16)>,

    // pub cell_styling: Grid<CellStyles>,
    pub column_widths: Vec<u16>,
}

impl Sheet {
    pub fn new() -> Sheet {
        Sheet {
            cells: Grid::new(),
            rendered_cells: Grid::new(),
            column_widths: vec![],
            // cell_styling: Grid::new(),
            dirty_cells: vec![],
        }
    }

    pub fn bake(&mut self) {
        // self.dirty_cells.clear();
        crate::compute::bake(&self.cells, &mut self.rendered_cells);
    }

    pub fn is_empty_at(&self, addr: (u16, u16)) -> bool {
        self.cells.get_at(addr)
            .is_none_or(|value| value.is_empty())
    }
}

pub struct Grid<T: Default> {
    data: Vec<Vec<T>>,
}

impl<T: Default> Grid<T> {
    pub fn new() -> Grid<T> {
        Grid { data: vec![] }
    }

    pub fn iter_rows(&self) -> &Vec<Vec<T>> {
        &self.data
    }

    pub fn num_rows(&self) -> u16 {
        self.data.len() as u16
    }
    pub fn num_columns_in_row(&self, r: u16) -> u16 {
        self.data.get(r as usize)
            .map(|row| row.len() as u16)
            .unwrap_or(0)
    }

    pub fn iter_addresses(&self) -> Vec<(u16, u16)> {
        let mut addresses = vec![];
        for (row_idx, row) in self.data.iter().enumerate() {
            for col_idx in 0..row.len() {
                addresses.push((row_idx as u16, col_idx as u16));
            }
        }
        addresses
    }

    pub fn get_at(&self, (r, c): (u16, u16)) -> Option<&T> {
        self.data.get(r as usize)
            .and_then(|row| row.get(c as usize))
    }

    pub fn set_at(&mut self, (r, c): (u16, u16), value: T) {
        // TODO clean up when value is default

        while self.data.len() <= r as usize {
            self.data.push(vec![]);
        }
        while self.data[r as usize].len() <= c as usize {
            self.data[r as usize].push(T::default());
        }
        self.data[r as usize][c as usize] = value;
    }
}