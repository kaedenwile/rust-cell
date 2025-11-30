use crate::window::{Frame, Screen, Window};

pub struct ScreenLayout<'a> {
    pub screen: &'a Screen,

    pub menu: Frame<'a>,
    pub window: Frame<'a>,
    pub status_bar: Frame<'a>,
}

impl<'a> ScreenLayout<'a> {
    pub fn new(screen: &'a Screen) -> Self {
        let mut new_layout = Self {
            screen,
            menu: Frame::new(screen),
            window: Frame::new(screen),
            status_bar: Frame::new(screen),
        };
        new_layout.layout();
        new_layout
    }

    pub fn layout(&mut self) {
        let (width, height) = self.screen.size();

        self.menu.position((0, 0), (width, 1));
        self.window.position((0, 1), (width, height - 2));
        self.status_bar.position((0, height), (width, 1));
    }
}