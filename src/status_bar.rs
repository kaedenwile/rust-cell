use crate::sheet::{Cursor, Sheet};
use crate::window::Window;
use termion::color;

pub enum StatusBar {}

impl StatusBar {
    pub fn draw(window: &mut dyn Window, Sheet { cursor, .. }: &Sheet) {
        let (width, height) = window.size();

        write!(
            window,
            "{}{}",
            color::Bg(color::LightBlue),
            color::Fg(color::Black)
        )
            .unwrap();

        let cursor_string = format!(
            "Cursor: {}",
            match cursor {
                Cursor::Single((r, c)) => format!("{}{}", r + 1, Sheet::col_name(*c as u8 + 1)),
                Cursor::Row(r) => format!("{r}:{r}", r = r + 1),
                Cursor::Column(c) => format!("{c}:{c}", c = Sheet::col_name(*c as u8 + 1)),
            }
        );

        for y in 0..height {
            window.go_to(1, y + 1);

            for x in 0..width {
                let mut chars = cursor_string.chars();
                write!(
                    window,
                    "{}",
                    chars.nth(x as usize).unwrap_or(' ').to_string()
                )
                    .unwrap();
            }
        }
    }
}
