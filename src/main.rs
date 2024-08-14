use crate::screen::draw;
use crate::sheet::{Cursor, DisplayCell, Sheet};
use crate::status_bar::StatusBar;
use crate::window::{get_screen, Frame, Window};
use std::io::{self, Write};
use termion::event::*;
use termion::input::TermRead;

mod screen;
mod sheet;
mod status_bar;
mod window;

enum Mode {
    Nav,
    Edit,
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = get_screen();

    let screen_size = stdout.size();
    let mut window = Frame::new(&mut stdout, (0, 0), (screen_size.0, screen_size.1 - 1));

    // TODO
    let mut stdout2 = get_screen();
    let mut status_bar = Frame::new(&mut stdout2, (0, screen_size.1 - 1), (screen_size.0, 1));

    let mut sheet = Sheet::blank();
    let mut mode = Mode::Nav;

    draw(&mut window, &sheet);
    StatusBar::draw(&mut status_bar, &sheet);
    window.flush().unwrap();

    for c in stdin.keys() {
        let evt = c.unwrap();

        match mode {
            Mode::Nav => match evt {
                Key::Char('q') => break,
                Key::Char('=') => {
                    if let Cursor::Single(_) = sheet.cursor {
                        mode = Mode::Edit
                    }
                }

                Key::Char('w') if sheet.scroll.0 > 0 => sheet.scroll.0 -= 1,
                Key::Char('a') if sheet.scroll.1 > 0 => sheet.scroll.1 -= 1,
                Key::Char('s') => sheet.scroll.0 += 1,
                Key::Char('d') => sheet.scroll.1 += 1,

                Key::Up => sheet.cursor = sheet.cursor.move_v(-1),
                Key::Down => sheet.cursor = sheet.cursor.move_v(1),
                Key::Left => sheet.cursor = sheet.cursor.move_h(-1),
                Key::Right => sheet.cursor = sheet.cursor.move_h(1),

                _ => {}
            },
            Mode::Edit => {
                let Cursor::Single(addr) = sheet.cursor else {
                    panic!("Non-single cursor in EDIT mode");
                };

                match evt {
                    Key::Char('\n') => mode = Mode::Nav,
                    Key::Char(l) => sheet.edit_at(addr, |cell| {
                        DisplayCell::new(cell.value.clone() + &l.to_string())
                    }),
                    Key::Backspace => sheet.edit_at(addr, |cell| {
                        let s = cell.value.clone();

                        if s.len() <= 1 {
                            return DisplayCell::blank();
                        }

                        let mut iter = s.char_indices();
                        let (end, _) = iter.nth_back(1).unwrap();
                        DisplayCell::new(s[..=end].to_string())
                    }),
                    _ => {}
                }
            }
        }

        draw(&mut window, &sheet);
        StatusBar::draw(&mut status_bar, &sheet);
        window.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}

// Shortcuts
//  q - quit
//
