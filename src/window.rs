use std::cell::RefCell;
use std::fmt::{Arguments, Debug, Pointer};
use std::io::{stdout, Stdout, Write};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::{AlternateScreen, IntoAlternateScreen};

// A rectangle that can be written to
pub trait Window {
    // get size of the window
    fn size(&self) -> (u16, u16);

    // move cursor within the window
    fn go_to(&self, x: u16, y: u16);

    fn write_fmt(&self, fmt: Arguments<'_>);

    fn flush(&self);
}

// The base screen object
pub struct Screen {
    inner: RefCell<RawTerminal<Stdout>>,
    // inner: RefCell<AlternateScreen<RawTerminal<Stdout>>>,
}

pub fn screen() -> Screen {
    let mut terminal = stdout().into_raw_mode().unwrap();
    // .into_alternate_screen()
    // .unwrap();

    write!(terminal, "{}", termion::cursor::Hide).unwrap();

    Screen {
        inner: RefCell::new(terminal),
    }
}

impl Window for Screen {
    fn size(&self) -> (u16, u16) {
        let Ok((cols, rows)) = termion::terminal_size() else {
            panic!("Could not get terminal size!");
        };

        (cols, rows)
    }

    fn go_to(&self, x: u16, y: u16) {
        write!(self, "{}", termion::cursor::Goto(x, y));
    }

    fn write_fmt(&self, fmt: Arguments<'_>) {
        self.inner.borrow_mut().write_fmt(fmt).unwrap()
    }

    fn flush(&self) {
        self.inner.borrow_mut().flush().unwrap()
    }
}

// A subsection of the screen
pub struct Frame<'a> {
    parent: &'a dyn Window,
    offset: (u16, u16),
    size: (u16, u16),
}

impl Frame<'_> {
    pub fn new(parent: &dyn Window, offset: (u16, u16), size: (u16, u16)) -> Frame {
        Frame {
            parent,
            offset,
            size,
        }
    }
}

impl Window for Frame<'_> {
    fn size(&self) -> (u16, u16) {
        self.size
    }

    fn go_to(&self, x: u16, y: u16) {
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
    }

    fn write_fmt(&self, fmt: Arguments<'_>) {
        self.parent.write_fmt(fmt)
    }

    fn flush(&self) {
        self.parent.flush()
    }
}
