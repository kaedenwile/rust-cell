use crate::state::State;
use crate::window::Window;
use termion::{color, style};

pub enum Menu {}

impl Menu {
    pub fn draw(window: &mut dyn Window, state: &State) {
        // let (width, _) = window.size();
        window.go_to(1, 1);

        write!(
            window,
            "{}{}",
            color::Bg(color::Black),
            color::Fg(color::White)
        );

        write!(
            window,
            "{}{}",
            color::Bg(color::Black),
            color::Fg(color::White)
        );

        write!(window, "{}F{}ile  {}E{}dit  {}S{}ave  {}Q{}uit",
               style::Underline,
               style::NoUnderline,
               style::Underline,
               style::NoUnderline,
               style::Underline,
               style::NoUnderline,
               style::Underline,
               style::NoUnderline,
        );
    }
}
