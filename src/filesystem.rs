use crate::state::{DisplayCell, State};
use std::fs::File;
use std::io::{Read, Write};

// TODO it would be nice to add a pretty error state on the state bar
pub fn save(state: &State, filepath: &str) {
    let mut file = File::create(filepath).unwrap();

    let mut csv = String::new();

    for row in state.content.iter() {
        csv.push(','); // first column will be ignored
        for cell in row {
            csv.push_str(cell.value.as_str());
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
            state.set_at((r as u16, c as u16), DisplayCell::new(cell.to_string()));
        }
    }

    state
}
