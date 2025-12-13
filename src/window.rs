use crate::color::Color;
use std::cell::RefCell;
use std::cmp::PartialEq;
use std::io::{stdout, Write};
use termion::cursor::HideCursor;
use termion::raw::IntoRawMode;

// A rectangle that can be written to
pub trait Window {
    // get size of the window
    fn size(&self) -> (u16, u16);

    fn write_at(&mut self, x: u16, y: u16, char: char, bg: Color, fg: Color, bold: bool, italic: bool, underline: bool);
}

// The base screen object
pub struct Screen {
    inner: RefCell<Box<dyn Write>>,

    /// (rows, columns)
    size: (u16, u16),
    prev_buffer: Vec<Vec<Pixel>>,
    live_buffer: Vec<Vec<Pixel>>,
}

#[derive(Clone, PartialEq)]
struct Pixel {
    char: char,
    fg: Color,
    bg: Color,
    bold: bool,
    italic: bool,
    underline: bool,
}

impl Pixel {
    fn new(char: char, fg: Color, bg: Color, bold: bool, italic: bool, underline: bool) -> Pixel {
        Pixel { char, fg, bg, bold, italic, underline }
    }
    fn blank() -> Pixel {
        Pixel::new(' ', Color::Magenta, Color::Magenta, false, false, false)
    }
}

pub fn screen() -> Screen {
    let terminal = HideCursor::from(
        stdout()
            .into_raw_mode()
            .unwrap()
        // .into_alternate_screen()
        // .unwrap(),
    );

    let mut screen = Screen {
        inner: RefCell::new(Box::new(terminal)),
        size: (0, 0),
        prev_buffer: vec![],
        live_buffer: vec![],
    };
    screen.resize(); // Resize to initialize buffers
    screen
}

impl Window for Screen {
    fn size(&self) -> (u16, u16) {
        self.size
    }

    fn write_at(&mut self, x: u16, y: u16, char: char, bg: Color, fg: Color, bold: bool, italic: bool, underline: bool) {
        self.live_buffer[y as usize][x as usize] = Pixel::new(char, fg, bg, bold, italic, underline);
    }
}

impl Screen {
    pub fn resize(&mut self) {
        let new_size = terminal_size();
        if new_size != self.size {
            self.size = new_size;
            let rows = new_size.1 as usize;
            let cols = new_size.0 as usize;

            self.prev_buffer = vec![vec![Pixel::blank(); cols]; rows];
            self.live_buffer = vec![vec![Pixel::blank(); cols]; rows];

            // Clear the terminal
            write!(self.inner.borrow_mut(), "{}", termion::clear::All).unwrap();
        }
    }

    /// Efficiently write the live buffer's changes to the terminal
    /// After testing, this doesn't seem to be a bottleneck, but it's good to have anyway
    pub fn flush(&mut self) {
        // what are current terminal styles? (Bg, Fg)
        let mut brush_bg: Option<Color> = None;
        let mut brush_fg: Option<Color> = None;
        let mut brush_bold: bool = false;
        let mut brush_italic: bool = false;
        let mut brush_underline: bool = false;
        let mut brush_pos = self.size.0 * self.size.1; // out of bounds

        let mut operations: Vec<String> = vec![];

        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                let pos = x + y * self.size.0;

                let live_pixel = &self.live_buffer[y as usize][x as usize];
                let prev_pixel = &self.prev_buffer[y as usize][x as usize];

                if *live_pixel != *prev_pixel {
                    if brush_pos != pos {
                        // Move cursor to position (x+1, y+1) because termion is 1-indexed
                        operations.push(termion::cursor::Goto(x + 1, y + 1).to_string())
                    }
                    if brush_bg != Some(live_pixel.bg) {
                        operations.push(live_pixel.bg.bg());
                        brush_bg = Some(live_pixel.bg);
                    }
                    if brush_fg != Some(live_pixel.fg) {
                        operations.push(live_pixel.fg.fg());
                        brush_fg = Some(live_pixel.fg);
                    }
                    if brush_bold != live_pixel.bold {
                        if live_pixel.bold {
                            operations.push(termion::style::Bold.to_string());
                        } else {
                            operations.push(termion::style::NoFaint.to_string());
                        }
                        brush_bold = live_pixel.bold;
                    }
                    if brush_italic != live_pixel.italic {
                        if live_pixel.italic {
                            operations.push(termion::style::Italic.to_string());
                        } else {
                            operations.push(termion::style::NoItalic.to_string());
                        }
                        brush_italic = live_pixel.italic;
                    }
                    if brush_underline != live_pixel.underline {
                        if live_pixel.underline {
                            operations.push(termion::style::Underline.to_string());
                        } else {
                            operations.push(termion::style::NoUnderline.to_string());
                        }
                        brush_underline = live_pixel.underline;
                    }

                    operations.push(live_pixel.char.to_string());
                    brush_pos = pos + 1;

                    // Update previous buffer
                    self.prev_buffer[y as usize][x as usize] = live_pixel.clone();
                }
            }
        }

        write!(self.inner.borrow_mut(), "{}", operations.join("")).unwrap();
        self.inner.borrow_mut().flush().unwrap();
    }
}

// A subsection of the screen
pub struct Frame<'a> {
    parent: &'a mut dyn Window,
    offset: (u16, u16),
    size: (u16, u16),
}

impl Frame<'_> {
    pub fn new(parent: &'_ mut dyn Window, offset: (u16, u16), size: (u16, u16)) -> Frame {
        Frame { parent, offset, size }
    }
}

impl Window for Frame<'_> {
    fn size(&self) -> (u16, u16) {
        self.size
    }

    fn write_at(&mut self, x: u16, y: u16, char: char, bg: Color, fg: Color, bold: bool, italic: bool, underline: bool) {
        self.parent.write_at(x + self.offset.0, y + self.offset.1, char, bg, fg, bold, italic, underline);
    }
}

fn terminal_size() -> (u16, u16) {
    let Ok((cols, rows)) = termion::terminal_size() else {
        panic!("Could not get terminal size!");
    };

    (cols, rows)
}