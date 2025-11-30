use crate::event_loop::{events, AppEvent};
use crate::filesystem::load;
use crate::layout::ScreenLayout;
use crate::menu::Menu;
use crate::screen::draw;
use crate::state::{Mode, State};
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
mod save_mode;
mod edit_mode;
mod nav_mode;

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

    for event in events() {
        match event {
            AppEvent::Key(Key::Ctrl('q')) => break, // Exit
            AppEvent::Resize => layout.layout(),
            AppEvent::Key(key) => match state.mode {
                Mode::Nav => state.handle_input_nav_mode(key),
                Mode::Edit => state.handle_input_edit_mode(key),
                Mode::Save => state.handle_input_save_mode(key),
            }
        }

        compute::bake(&mut state);
        draw(&mut layout.window, &state);
        StatusBar::draw(&mut layout.status_bar, &state);
        Menu::draw(&mut layout.menu, &state);
        screen.flush();
    }
}
