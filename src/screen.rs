use crate::state::{Address, Alignment, Cursor, DisplayCell, State};
use crate::window::Window;
use termion::color;
use termion::style;

enum Position<'a> {
    BetweenRows(Address, Address),
    BetweenCols(Address, Address),
    Corner {
        top_left: Address,
        bottom_right: Address,
    },

    ColumnHeader(u16, u16),
    RowHeader(u16, u16),
    InsideCell(Address, &'a DisplayCell, u16),
}

pub fn draw(
    window: &dyn Window,
    State {
        content,
        cursor,
        scroll,
        ..
    }: &State,
) {
    let (width, height) = window.size();

    let screen_sel = match cursor {
        Cursor::Single((r, c)) => (r + 1, c + 1),
        Cursor::Row(r) => (*r + 1, 0),
        Cursor::Column(c) => (0, *c + 1),
    };

    let blank_cell = DisplayCell::blank();

    for y in 0..height {
        window.go_to(1, y + 1);

        for x in 0..width {
            // position on screen
            let row = if y < 2 { 0 } else { 1 + (scroll.0 + y - 2) / 2 };
            let col = if x < 4 { 0 } else { 1 + (scroll.1 + x - 4) / 8 };

            let y = if row == 0 { y } else { y + scroll.0 };
            let x = if col == 0 { x } else { x + scroll.1 };
            let text_pos = (x + 4) % 8;

            // APPLY STYLING TO HEADER
            if row == 0 || col == 0 {
                write!(
                    window,
                    "{}{}",
                    color::Bg(color::LightBlack),
                    color::Fg(color::Black),
                )
            } else {
                write!(
                    window,
                    "{}{}",
                    color::Bg(color::White),
                    color::Fg(color::Black),
                )
            };

            use Position::*;
            let position = if x % 8 == 3 && y % 2 == 1 {
                Corner {
                    top_left: (row, col),
                    bottom_right: (row + 1, col + 1),
                }
            } else if x % 8 == 3 {
                BetweenCols((row, col), (row, col + 1))
            } else if y % 2 == 1 {
                BetweenRows((row, col), (row + 1, col))
            } else if y < 2 {
                ColumnHeader(col, text_pos)
            } else if x < 3 {
                RowHeader(row, text_pos)
            } else {
                let cell = content
                    .get((row - 1) as usize)
                    .and_then(|x| x.get((col - 1) as usize))
                    .unwrap_or_else(|| &blank_cell);

                InsideCell((row, col), cell, text_pos)
            };

            let val = match position {
                Corner { top_left: addr, .. } if addr == screen_sel => "╃",
                Corner {
                    top_left: (r, _),
                    bottom_right: (_, c),
                } if (r, c) == screen_sel => "╄",
                Corner {
                    top_left: (_, c),
                    bottom_right: (r, _),
                } if (r, c) == screen_sel => "╅",
                Corner {
                    bottom_right: addr, ..
                } if addr == screen_sel => "╆",
                Corner { .. } => "┼",

                BetweenCols(addr, _) | BetweenCols(_, addr) if addr == screen_sel => "┃",
                BetweenCols(..) => "│",
                BetweenRows(addr, _) | BetweenRows(_, addr) if addr == screen_sel => "━",
                BetweenRows(..) => "─",

                ColumnHeader(col, text_pos) => &State::col_name(col as u8)
                    .chars()
                    .nth(text_pos as usize)
                    .unwrap_or(' ')
                    .to_string(),

                RowHeader(row, text_pos) => &row
                    .to_string()
                    .chars()
                    .nth_back(6 - text_pos as usize)
                    .unwrap_or(' ')
                    .to_string(),

                InsideCell(_, cell, text_pos) => {
                    let mut chars = cell.value.chars();
                    let l = match &cell.alignment {
                        Alignment::Left => chars.nth(text_pos as usize),
                        Alignment::Right => chars.nth_back(6 - text_pos as usize),
                    };

                    &l.unwrap_or(' ').to_string()
                }
            };

            write!(window, "{}{}", val, style::Reset);
        }
    }
}
