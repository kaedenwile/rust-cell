use crate::event_loop::{events, AppEvent};
use crate::filesystem::{load, save};
use crate::layout::ScreenLayout;
use crate::menu::Menu;
use crate::screen::draw;
use crate::state::{Cursor, Mode, State};
use crate::status_bar::StatusBar;
use crate::window::{screen, Window};
use std::env;
use termion::event::Key;
use termion::input::TermRead;

mod compute;
mod filesystem;
mod screen;
mod state;
mod status_bar;
mod window;
mod color;
mod event_loop;
mod layout;
mod menu;

fn main() {
    let screen = &screen();
    let mut layout = ScreenLayout::new(screen);

    let args: Vec<String> = env::args().collect();
    let mut state = State::blank();
    if args.len() > 1 {
        state = load(&args[1]);
    }

    compute::bake(&mut state);
    draw(&mut layout.window, &state);
    StatusBar::draw(&mut layout.status_bar, &state);
    Menu::draw(&mut layout.menu, &state);
    screen.flush();

    for evt in events() {
        if let AppEvent::Resize = evt {
            layout.layout();
            draw(&mut layout.window, &state);
            StatusBar::draw(&mut layout.status_bar, &state);
            Menu::draw(&mut layout.menu, &state);
            screen.flush();
            continue;
        }

        match state.mode {
            Mode::Nav => match evt {
                AppEvent::Key(Key::Ctrl('q')) => break, // Exit
                AppEvent::Key(Key::Ctrl('c')) => break, // Exit

                // Save
                AppEvent::Key(Key::Ctrl('s')) => {
                    state.mode = Mode::Save;
                    state.edit_buffer = state.filename.clone();
                    state.edit_cursor = state.edit_buffer.len();
                }

                // Delete cell
                AppEvent::Key(Key::Backspace) => {
                    if let Cursor::Single(addr) = state.cursor {
                        state.clear_at(addr);
                    }
                }

                // Start editing
                AppEvent::Key(Key::Char(l)) => {
                    if let Cursor::Single(addr) = state.cursor {
                        state.mode = Mode::Edit;
                        state.edit_buffer = l.to_string();
                        state.edit_cursor = 1;
                    }
                }

                AppEvent::Key(Key::Ctrl('z')) => state.undo(),
                AppEvent::Key(Key::Ctrl('y')) => state.redo(),

                // AppEvent::Key(Key::Char('w')) if state.scroll.0 > 0 => state.scroll.0 -= 1,
                // AppEvent::Key(Key::Char('a')) if state.scroll.1 > 0 => state.scroll.1 -= 1,
                // AppEvent::Key(Key::Char('s')) => state.scroll.0 += 1,
                // AppEvent::Key(Key::Char('d')) => state.scroll.1 += 1,

                AppEvent::Key(Key::Up) => state.cursor = state.cursor.move_v(-1),
                AppEvent::Key(Key::Down) => state.cursor = state.cursor.move_v(1),
                AppEvent::Key(Key::Left) => state.cursor = state.cursor.move_h(-1),
                AppEvent::Key(Key::Right) => state.cursor = state.cursor.move_h(1),

                AppEvent::Key(Key::AltUp) => state.cursor = state.cursor.jump_v(-1, &state),
                AppEvent::Key(Key::AltDown) => state.cursor = state.cursor.jump_v(1, &state),
                // Ghostty treats AltLeft as Alt-f and AltRight as Alt-b
                AppEvent::Key(Key::Alt('f')) => state.cursor = state.cursor.jump_h(1, &state),
                AppEvent::Key(Key::Alt('b')) => state.cursor = state.cursor.jump_h(-1, &state),

                _ => {}
            },
            Mode::Edit => {
                let Cursor::Single(addr) = state.cursor else {
                    panic!("Non-single cursor in EDIT mode");
                };

                match evt {
                    AppEvent::Key(Key::Char('\n')) => {
                        state.save_edits();
                        state.cursor = state.cursor.move_v(1);
                    }
                    AppEvent::Key(Key::Esc) => state.mode = Mode::Nav,

                    AppEvent::Key(Key::Ctrl('a')) => state.edit_cursor = 0,
                    AppEvent::Key(Key::Ctrl('e')) => state.edit_cursor = state.get_at(addr).value.len(),
                    AppEvent::Key(Key::Alt('f')) => {
                        state.edit_cursor = state.get_at(addr).value[state.edit_cursor..]
                            .find(" ")
                            .and_then(|idx| Some(idx + state.edit_cursor + 1))
                            .unwrap_or(state.get_at(addr).value.len())
                    }
                    AppEvent::Key(Key::Alt('b')) => {
                        state.edit_cursor = state.get_at(addr).value[..state.edit_cursor]
                            .rfind(" ")
                            .and_then(|idx| if idx == 0 { None } else { Some(idx - 1) })
                            .unwrap_or(0)
                    }

                    AppEvent::Key(Key::Char(l)) => {
                        state.edit_buffer.insert(state.edit_cursor, l);
                        state.edit_cursor += 1;
                    }

                    AppEvent::Key(Key::Backspace) => {
                        if state.edit_cursor == 0 {
                            continue;
                        }
                        state.edit_buffer.remove(state.edit_cursor - 1);
                        state.edit_cursor -= 1;
                    }

                    AppEvent::Key(Key::Left) if state.edit_cursor > 0 => state.edit_cursor -= 1,
                    AppEvent::Key(Key::Right) if state.edit_cursor < state.get_at(addr).value.len() => {
                        state.edit_cursor += 1
                    }

                    _ => {}
                }
            }
            Mode::Save => match evt {
                AppEvent::Key(Key::Char('\n')) => {
                    state.filename = state.edit_buffer.clone();
                    save(&state, &state.edit_buffer);
                    state.mode = Mode::Nav;
                }
                AppEvent::Key(Key::Esc) => state.mode = Mode::Nav,

                AppEvent::Key(Key::Char(l)) => {
                    state.edit_buffer.insert(state.edit_cursor, l);
                    state.edit_cursor += 1;
                }

                AppEvent::Key(Key::Backspace) => {
                    if state.edit_cursor == 0 {
                        continue;
                    }
                    state.edit_buffer.remove(state.edit_cursor - 1);
                    state.edit_cursor -= 1;
                }

                _ => {}
            },
        }

        compute::bake(&mut state);
        draw(&mut layout.window, &state);
        StatusBar::draw(&mut layout.status_bar, &state);
        Menu::draw(&mut layout.menu, &state);
        screen.flush();
    }
}

// Shortcuts
//  q - quit
//  = - edit
//
//  wasd - scroll
//  arrow keys - move selection
//
