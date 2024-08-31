use crate::screen::draw;
use crate::state::{Cursor, DisplayCell, Mode, State};
use crate::status_bar::StatusBar;
use crate::window::{screen, Frame, Window};
use std::io;
use termion::event::*;
use termion::input::TermRead;

mod compute;
mod screen;
mod state;
mod status_bar;
mod window;

fn main() {
    let stdin = io::stdin();
    let screen = &screen();

    let screen_size = screen.size();
    let mut window = Frame::new(screen, (0, 0), (screen_size.0, screen_size.1 - 1));
    let mut status_bar = Frame::new(screen, (0, screen_size.1), (screen_size.0, 1));

    let mut state = State::blank();
    state.edit_at((2, 2), |_| DisplayCell::new("4 * ( 2 + 3 )".to_string()));
    compute::compute(&mut state);

    draw(&mut window, &state);
    StatusBar::draw(&mut status_bar, &state);
    window.flush();

    for c in stdin.keys() {
        let evt = c.unwrap();

        match state.mode {
            Mode::Nav => match evt {
                Key::Char('q') => break,
                Key::Char('=') => {
                    if let Cursor::Single(_) = state.cursor {
                        state.mode = Mode::Edit
                    }
                }

                Key::Char('w') if state.scroll.0 > 0 => state.scroll.0 -= 1,
                Key::Char('a') if state.scroll.1 > 0 => state.scroll.1 -= 1,
                Key::Char('s') => state.scroll.0 += 1,
                Key::Char('d') => state.scroll.1 += 1,

                Key::Up => state.cursor = state.cursor.move_v(-1),
                Key::Down => state.cursor = state.cursor.move_v(1),
                Key::Left => state.cursor = state.cursor.move_h(-1),
                Key::Right => state.cursor = state.cursor.move_h(1),

                _ => {}
            },
            Mode::Edit => {
                let Cursor::Single(addr) = state.cursor else {
                    panic!("Non-single cursor in EDIT mode");
                };

                match evt {
                    Key::Char('\n') => state.mode = Mode::Nav,
                    Key::Char(l) => state.edit_at(addr, |cell| {
                        DisplayCell::new(cell.value.clone() + &l.to_string())
                    }),
                    Key::Backspace => state.edit_at(addr, |cell| {
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

        compute::compute(&mut state);
        draw(&mut window, &state);
        StatusBar::draw(&mut status_bar, &state);
        window.flush();
    }

    write!(screen, "{}", termion::cursor::Show)
}

// Shortcuts
//  q - quit
//  = - edit
//
//  wasd - scroll
//  arrow keys - move selection
//
