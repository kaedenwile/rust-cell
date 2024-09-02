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
    state.set_at((2, 2), DisplayCell::new("4 * ( 2 + 3 )".to_string()));
    compute::bake(&mut state);

    draw(&mut window, &state);
    StatusBar::draw(&mut status_bar, &state);
    window.flush();

    for c in stdin.keys() {
        let evt = c.unwrap();

        match state.mode {
            Mode::Nav => match evt {
                Key::Char('q') => break,
                Key::Char('=') => {
                    if let Cursor::Single(addr) = state.cursor {
                        state.mode = Mode::Edit;
                        state.edit_cursor = state.get_at(addr).value.len();
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
                    Key::Char('\n') | Key::Esc => state.mode = Mode::Nav,
                    Key::Ctrl('a') => state.edit_cursor = 0,
                    Key::Ctrl('e') => state.edit_cursor = state.get_at(addr).value.len(),
                    Key::Alt('f') => {
                        state.edit_cursor = state.get_at(addr).value[state.edit_cursor..]
                            .find(" ")
                            .and_then(|idx| Some(idx + state.edit_cursor + 1))
                            .unwrap_or(state.get_at(addr).value.len())
                    }
                    Key::Alt('b') => {
                        state.edit_cursor = state.get_at(addr).value[..state.edit_cursor]
                            .rfind(" ")
                            .and_then(|idx| if idx == 0 { None } else { Some(idx - 1) })
                            .unwrap_or(0)
                    }

                    Key::Char(l) => {
                        let edit_cursor = state.edit_cursor;
                        state.edit_at(addr, |cell| {
                            let mut new_val = cell.value.clone();
                            new_val.insert(edit_cursor, l);
                            DisplayCell::new(new_val)
                        });
                        state.edit_cursor += 1;
                    }

                    Key::Backspace => {
                        let edit_cursor = state.edit_cursor;
                        state.edit_at(addr, |cell| {
                            let mut new_val = cell.value.clone();

                            if edit_cursor == 0 {
                                return DisplayCell::new(new_val);
                            }

                            new_val.remove(edit_cursor - 1);
                            DisplayCell::new(new_val)
                        });

                        if state.edit_cursor > 0 {
                            state.edit_cursor -= 1;
                        }
                    }

                    Key::Left if state.edit_cursor > 0 => state.edit_cursor -= 1,
                    Key::Right if state.edit_cursor < state.get_at(addr).value.len() => {
                        state.edit_cursor += 1
                    }

                    _ => {}
                }
            }
        }

        compute::bake(&mut state);
        draw(&mut window, &state);
        StatusBar::draw(&mut status_bar, &state);
        window.flush();
    }
}

// Shortcuts
//  q - quit
//  = - edit
//
//  wasd - scroll
//  arrow keys - move selection
//
