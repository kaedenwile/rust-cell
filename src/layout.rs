use crate::menu::Menu;
use crate::screen::draw;
use crate::state::State;
use crate::status_bar::StatusBar;
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
        new_layout.layout();
        new_layout
    }

    pub fn draw(&mut self, state: &State) {
        draw(&mut self.window.to_frame(self.screen), &state);
        StatusBar::draw(&mut self.status_bar.to_frame(self.screen), &state);
        Menu::draw(&mut self.menu.to_frame(self.screen), &state);
        self.screen.flush();
    }

    pub fn layout(&mut self) {
        self.screen.resize();
        let (width, height) = self.screen.size();

        self.menu = FrameLayout { offset: (0, 0), size: (width, 1) };
        self.window = FrameLayout { offset: (0, 1), size: (width, height - 2) };
        self.status_bar = FrameLayout { offset: (0, height - 1), size: (width, 1) };
    }
}

impl FrameLayout {
    pub fn to_frame<'a>(&self, screen: &'a mut Screen) -> Frame<'a> {
        Frame::new(screen, self.offset, self.size)
    }
}