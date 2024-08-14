use std::io::{stdout, Stdout, Write};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::{AlternateScreen, IntoAlternateScreen};

// A rectangle that can be written to
pub trait Window: Write {
    // get size of the window
    fn size(&self) -> (u16, u16);

    // move cursor within the window
    fn go_to(&mut self, x: u16, y: u16);
}

// The base screen object
pub type Screen = AlternateScreen<RawTerminal<Stdout>>;

pub fn get_screen() -> Screen {
    let mut screen = stdout()
        .into_raw_mode()
        .unwrap()
        .into_alternate_screen()
        .unwrap();

    write!(screen, "{}", termion::cursor::Hide).unwrap();

    screen
}

impl Window for Screen {
    fn size(&self) -> (u16, u16) {
        let Ok((cols, rows)) = termion::terminal_size() else {
            panic!("Could not get terminal size!");
        };

        (cols, rows)
    }

    fn go_to(&mut self, x: u16, y: u16) {
        write!(self, "{}", termion::cursor::Goto(x, y)).unwrap();
    }
}

// A subsection of the screen
pub struct Frame<'a> {
    parent: &'a mut dyn Window,
    offset: (u16, u16),
    size: (u16, u16),
}

impl Frame<'_> {
    pub fn new(parent: &mut dyn Window, offset: (u16, u16), size: (u16, u16)) -> Frame {
        Frame {
            parent,
            offset,
            size,
        }
    }
}

impl Write for Frame<'_> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.parent.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.parent.flush()
    }
}

impl Window for Frame<'_> {
    fn size(&self) -> (u16, u16) {
        self.size
    }

    fn go_to(&mut self, x: u16, y: u16) {
        if x == 0 || y == 0 {
            panic!("go_to is 1-based")
        } else if x > self.size.0 || y > self.size.1 {
            panic!("Writing to frame OOB!")
        }

        write!(
            self,
            "{}",
            termion::cursor::Goto(x + self.offset.0, y + self.offset.1)
        )
            .unwrap();
    }
}
