use crate::menu::Menu;
use crate::screen::draw;
use crate::state::{Mode, State};
use crate::status_bar::StatusBar;
use crate::styling::{StylingWindow, STYLING_WINDOW_WIDTH};
use crate::window::{Frame, Screen, Window};

pub struct ScreenLayout<'a> {
    screen: &'a mut Screen,
    menu: FrameLayout,
    window: FrameLayout,
    status_bar: FrameLayout,
}

struct FrameLayout {
    offset: (u16, u16),
    size: (u16, u16),
}

impl<'a> ScreenLayout<'a> {
    pub fn new(screen: &'a mut Screen) -> Self {
        let mut new_layout = Self {
            screen,
            menu: FrameLayout { offset: (0, 0), size: (0, 0) },
            window: FrameLayout { offset: (0, 0), size: (0, 0) },
            status_bar: FrameLayout { offset: (0, 0), size: (0, 0) },
        };
        new_layout.resize();
        new_layout
    }

    pub fn draw(&mut self, state: &State) {
        let (width, height) = self.screen.size();

        let (primary_w, secondary_w) = match state.mode {
            Mode::Format => (width - STYLING_WINDOW_WIDTH, STYLING_WINDOW_WIDTH),
            _ => (width, 0),
        };

        // Run safely in IntelliJ
        if height <= 2 { return; }

        Menu::draw(&mut Frame::new(self.screen, (0, 0), (width, 1)), &state);
        draw(&mut Frame::new(self.screen, (0, 1), (primary_w, height - 2)), &state);
        StatusBar::draw(&mut Frame::new(self.screen, (0, height - 1), (primary_w, 1)), &state);

        if let Mode::Format = state.mode {
            StylingWindow::draw(&mut Frame::new(self.screen, (primary_w, 1), (secondary_w, height - 1)), &state);
        }

        self.screen.flush();
    }

    pub fn resize(&mut self) {
        self.screen.resize();
    }
}

impl FrameLayout {
    pub fn to_frame<'a>(&self, screen: &'a mut Screen) -> Frame<'a> {
        Frame::new(screen, self.offset, self.size)
    }
}