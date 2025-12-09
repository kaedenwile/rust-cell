use crate::color::Color;
use crate::state::State;
use crate::window::Window;

pub enum Menu {}

impl Menu {
    pub fn draw(window: &mut dyn Window, _state: &State) {
        let (width, _) = window.size();
        let message = "Format  Edit  Save  Quit";

        for x in 0..width {
            window.write_at(x, 0, message.chars().nth(x as usize).unwrap_or(' '), Color::Black, Color::White);
        }
    }
}
