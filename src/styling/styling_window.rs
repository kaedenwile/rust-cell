use crate::color::Color;
use crate::state::State;
use crate::window::Window;

pub enum StylingWindow {}

impl StylingWindow {
    pub fn draw(window: &mut dyn Window, _state: &State) {
        let (width, height) = window.size();

        for x in 0..width {
            for y in 0..height {
                window.write_at(x, y, ' ', Color::LightMagenta, Color::Black);
            }
        }
    }
}
