use crate::event_loop::{events, AppEvent};
use crate::filesystem::load;
use crate::keyboard::Key;
use crate::layout::ScreenLayout;
use crate::state::{Mode, State};
use crate::window::{screen, Window};
use std::env;

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
mod cursor;
mod keyboard;

fn main() {
    let screen = &mut screen();
    let mut layout = ScreenLayout::new(screen);

    let args: Vec<String> = env::args().collect();
    let mut state = State::blank();
    if args.len() > 1 {
        state = load(&args[1]);
    }

    compute::bake(&mut state);
    layout.draw(&state);

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
        layout.draw(&state);
    }
}
