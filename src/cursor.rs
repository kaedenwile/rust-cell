use crate::state::{Address, State};
use std::cmp::{max, min};
use std::fmt;
use std::fmt::Display;

static MAX_ROW: u16 = 1000;
static MAX_COL: u16 = 1000;

#[derive(Clone)]
pub enum Cursor {
    Single(Address),
    /// start, current (may not be ordered)
    Range(Address, Address),
    Row(u16),
    Column(u16),

    // TODO multi column, multi row
}

enum Orientation {
    Horizontal,
    Vertical,
}

impl Display for Cursor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Cursor::Single((r, c)) => write!(f, "{}{}", r + 1, State::col_name(c as u8 + 1)),
            Cursor::Range(start, end) => write!(f, "{}{}:{}{}",
                                                start.0 + 1,
                                                State::col_name(start.1 as u8 + 1),
                                                end.0 + 1,
                                                State::col_name(end.1 as u8 + 1)
            ),
            Cursor::Row(r) => write!(f, "{r}:{r}", r = r + 1),
            Cursor::Column(c) => write!(f, "{c}:{c}", c = State::col_name(c as u8 + 1)),
        }
    }
}

impl Cursor {
    pub fn contains(&self, addr: Address) -> bool {
        match *self {
            Cursor::Single((r, c)) => addr == (r, c),
            Cursor::Range(start, end) => {
                let (l, r, t, b) = Cursor::bounds(start, end);
                addr.0 >= t && addr.0 <= b && addr.1 >= l && addr.1 <= r
            }
            Cursor::Row(r) => addr.0 == r,
            Cursor::Column(c) => addr.1 == c,
        }
    }

    pub fn bounds((r1, c1): Address, (r2, c2): Address) -> (u16, u16, u16, u16) {
        let left = min(c1, c2);
        let right = max(c1, c2);
        let top = min(r1, r2);
        let bottom = max(r1, r2);
        (left, right, top, bottom)
    }

    pub fn move_h(&self, direction: i16) -> Self {
        match *self {
            Cursor::Single((r, c)) if direction < 0 && c == 0 => Cursor::Row(r),
            Cursor::Single((r, c)) => Cursor::Single((r, c.saturating_add_signed(direction))),

            // If range is selected, treat as Single selection from starting point
            Cursor::Range(_, (r, c)) if direction < 0 && c == 0 => Cursor::Row(r),
            Cursor::Range(_, (r, c)) => Cursor::Single((r, c.saturating_add_signed(direction))),

            Cursor::Row(r) if direction < 0 => Cursor::Row(r), // copy of self
            Cursor::Row(r) => Cursor::Single((r, 0)),
            Cursor::Column(c) => Cursor::Column(c.saturating_add_signed(direction)),
        }
    }

    pub fn move_v(&self, direction: i16) -> Self {
        match *self {
            Cursor::Single((r, c)) if direction < 0 && r == 0 => Cursor::Column(c),
            Cursor::Single((r, c)) => Cursor::Single((r.saturating_add_signed(direction), c)),

            // If range is selected, treat as Single selection from starting point
            Cursor::Range(_, (r, c)) if direction < 0 && r == 0 => Cursor::Column(c),
            Cursor::Range(_, (r, c)) => Cursor::Single((r.saturating_add_signed(direction), c)),

            Cursor::Row(r) => Cursor::Row(r.saturating_add_signed(direction)),
            Cursor::Column(c) if direction < 0 => Cursor::Column(c), // copy of self
            Cursor::Column(c) => Cursor::Single((0, c)),
        }
    }

    pub fn jump_h(&self, direction: i16, state: &State) -> Self {
        match *self {
            Cursor::Single(addr) =>
                Cursor::Single(state.jump_addr(addr, Orientation::Horizontal, direction)),
            Cursor::Range(_, end) =>
                Cursor::Single(state.jump_addr(end, Orientation::Horizontal, direction)),
            _ => self.clone()
        }
    }
    pub fn jump_v(&self, direction: i16, state: &State) -> Self {
        match *self {
            Cursor::Single(addr) =>
                Cursor::Single(state.jump_addr(addr, Orientation::Vertical, direction)),
            Cursor::Range(_, end) =>
                Cursor::Single(state.jump_addr(end, Orientation::Vertical, direction)),
            _ => self.clone()
        }
    }

    pub fn select_h(&self, direction: i16) -> Self {
        match *self {
            // Cannot expand selection left past column 0
            Cursor::Single((_, c)) if direction < 0 && c == 0 => self.clone(),
            Cursor::Range(_, (_, c)) if direction < 0 && c == 0 => self.clone(),

            // Start a new range selection
            Cursor::Single((r, c)) => Cursor::Range((r, c), (r, c.saturating_add_signed(direction))),

            // Convert to single if reverting to one-cell selection
            Cursor::Range((start_row, start_col), (end_row, end_col))
            if start_row == end_row && start_col == end_col.saturating_add_signed(direction)
            => Cursor::Single((start_row, start_col)),

            // Otherwise, update range
            Cursor::Range(start, (r, c)) => Cursor::Range(start, (r, c.saturating_add_signed(direction))),

            // TODO Ignoring multi-row for now
            _ => self.clone()
        }
    }

    pub fn select_v(&self, direction: i16) -> Self {
        match *self {
            // Cannot expand selection up past row 0
            Cursor::Single((r, _)) if direction < 0 && r == 0 => self.clone(),
            Cursor::Range(_, (r, _)) if direction < 0 && r == 0 => self.clone(),

            // Start a new range selection
            Cursor::Single((r, c)) => Cursor::Range((r, c), (r.saturating_add_signed(direction), c)),

            // Convert to single if reverting to one-cell selection
            Cursor::Range((start_row, start_col), (end_row, end_col))
            if start_row == end_row.saturating_add_signed(direction) && start_col == end_col
            => Cursor::Single((start_row, start_col)),

            // Otherwise, update range
            Cursor::Range(start, (r, c)) => Cursor::Range(start, (r.saturating_add_signed(direction), c)),

            // TODO Ignoring multi-column for now
            _ => self.clone()
        }
    }

    pub fn select_jump_h(&self, direction: i16, state: &State) -> Self {
        self.select_jump(Orientation::Horizontal, direction, state)
    }

    pub fn select_jump_v(&self, direction: i16, state: &State) -> Self {
        self.select_jump(Orientation::Vertical, direction, state)
    }

    pub fn select_jump(&self, orientation: Orientation, direction: i16, state: &State) -> Self {
        match *self {
            Cursor::Single(addr) => {
                let jump_addr = state.jump_addr(addr, orientation, direction);
                if addr == jump_addr {
                    self.clone()
                } else {
                    Cursor::Range(addr, jump_addr)
                }
            }
            Cursor::Range(start, end) => {
                let jump_addr = state.jump_addr(end, orientation, direction);
                if start == jump_addr {
                    Cursor::Single(jump_addr)
                } else {
                    Cursor::Range(start, jump_addr)
                }
            }
            _ => self.clone()
        }
    }
}

impl State {
    fn jump_addr(&self, start: Address, orientation: Orientation, direction: i16) -> Address {
        let take_step = |(r, c): Address| -> Address {
            match orientation {
                Orientation::Horizontal => (r, min(c.saturating_add_signed(direction), MAX_COL - 1)),
                Orientation::Vertical => (min(r.saturating_add_signed(direction), MAX_ROW - 1), c),
            }
        };

        let start_is_empty = self.sheet.is_empty_at(start);
        let mut prev = start;
        let mut prev_is_empty = start_is_empty;

        loop {
            let next = take_step(prev);
            let next_is_empty = self.sheet.is_empty_at(next);

            if next == prev {
                return prev; // no movement possible
            } else if start_is_empty && prev_is_empty && !next_is_empty {
                return next;
            } else if !start_is_empty && prev_is_empty && !next_is_empty {
                return next
            } else if prev != start && !start_is_empty && !prev_is_empty && next_is_empty {
                return prev
            }

            (prev, prev_is_empty) = (next, next_is_empty);
        }
    }
}
