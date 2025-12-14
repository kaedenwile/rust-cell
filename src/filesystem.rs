use crate::state::State;
use std::fs::File;
use std::io::{Read, Write};
use crate::styling::CellStyles;

// TODO it would be nice to add a pretty error state on the state bar
pub fn save(state: &State, filepath: &str) {
    let mut file = File::create(filepath).unwrap();

    let mut csv = String::new();

    // First row is column widths
    csv.push(','); // first column will be ignored
    for width in state.sheet.column_widths.iter() {
        csv.push_str(&width.to_string());
        csv.push(',');
    }
    csv.push('\n');

    for (y, row) in state.sheet.cells.iter_rows().iter().enumerate() {
        csv.push(','); // first column will be ignored
        for (x, cell) in row.iter().enumerate() {
            csv.push_str(cell);
            if let Some(style) = state.sheet.cell_styling.get_at((y as u16, x as u16)) {
                if *style != CellStyles::default() {
                    // use separator to escape styling
                    csv.push_str("|:");
                    csv.push_str(
                        state.sheet.cell_styling.get_at((y as u16, x as u16))
                            .map(|s| s.serialize())
                            .unwrap_or_default()
                            .as_str());
                }
            }
            csv.push(',');
        }
        csv.push('\n');
    }

    file.write_all(csv.as_bytes()).unwrap();
}

pub fn load(filepath: &str) -> State {
    let mut state = State::blank();

    let mut csv = String::new();

    let Ok(mut file) = File::open(filepath) else {
        panic!("File not found");
    };
    let Ok(_) = file.read_to_string(&mut csv) else {
        panic!("Unabled to read file");
    };

    for (r, row_str) in csv.lines().enumerate() {
        for (c, cell) in row_str.split(',').skip(1).enumerate() {
            // First row is column widths
            if r == 0 {
                state.sheet.column_widths.push(cell.parse::<u16>().unwrap_or(8));
            } else {
                let (cell, styling) = cell.split_once("|:").unwrap_or((cell, ""));
                state.sheet.cells.set_at(((r - 1) as u16, c as u16), cell.to_string());
                if !styling.is_empty() {
                    state.sheet.cell_styling.set_at(
                        ((r - 1) as u16, c as u16),
                        CellStyles::deserialize(styling).unwrap_or_default(),
                    );
                }
            }
        }
    }

    state.filename = filepath.to_string();
    state
}
