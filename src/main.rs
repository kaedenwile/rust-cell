use crate::screen::{draw, get_screen};
use crate::sheet::{Cursor, DisplayCell, Sheet};
use std::io::{self, Write};
use termion::event::*;
use termion::input::TermRead;

mod screen;
mod sheet;

enum Mode {
    Nav,
    Edit,
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = get_screen();

    let mut sheet = Sheet::blank(25);
    let mut mode = Mode::Nav;

    draw(&mut stdout, &sheet);

    for c in stdin.keys() {
        let evt = c.unwrap();

        match mode {
            Mode::Nav => match evt {
                Key::Char('q') => break,
                Key::Char('=') => mode = Mode::Edit,

                Key::Up => sheet.cursor = sheet.cursor.move_v(-1),
                Key::Down => sheet.cursor = sheet.cursor.move_v(1),
                Key::Left => sheet.cursor = sheet.cursor.move_h(-1),
                Key::Right => sheet.cursor = sheet.cursor.move_h(1),

                _ => {}
            },
            Mode::Edit => match evt {
                Key::Char('\n') => mode = Mode::Nav,
                Key::Char(l) => {
                    let Cursor::Single(addr) = sheet.cursor;
                    sheet.edit_at(addr, |cell| {
                        DisplayCell::new(cell.value.clone() + &l.to_string())
                    })
                }
                Key::Backspace => {
                    let Cursor::Single(addr) = sheet.cursor;
                    sheet.edit_at(addr, |cell| {
                        let s = cell.value.clone();

                        if s.len() <= 1 {
                            return DisplayCell::blank();
                        }

                        let mut iter = s.char_indices();
                        let (end, _) = iter.nth_back(1).unwrap();
                        DisplayCell::new(s[..=end].to_string())
                    })
                }
                _ => {}
            },
        }

        draw(&mut stdout, &sheet);
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}

// Shortcuts
//  q - quit
//
