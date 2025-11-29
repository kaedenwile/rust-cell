use crate::state::{Address, Alignment, Cursor, DisplayCell, Mode, State};
use crate::window::Window;
use termion::style;
use crate::color::Color;

enum Position<'a> {
    Pivot,
    ColumnHeader(u16, u16),
    RowHeader(u16, u16),
    InsideCell(Address, &'a DisplayCell, u16),
}

static CELL_WIDTH: u16 = 8;
static ROW_HEADER_WIDTH: u16 = 3;

/// Draw the spreadsheet
pub fn draw(window: &dyn Window, state: &State) {
    let State { cursor, scroll, .. } = state;
    let (width, height) = window.size();

    for y in 0..height {
        window.go_to(1, y + 1);

        for x in 0..width {
            // position on screen
            let row = y + scroll.0;
            let col = if x < ROW_HEADER_WIDTH { 0 } else { 1 + (scroll.1 + x - ROW_HEADER_WIDTH) / CELL_WIDTH };

            let y = if row == 0 { y } else { y + scroll.0 };
            let x = if col == 0 { x } else { x + scroll.1 };
            let text_pos = (x + CELL_WIDTH - ROW_HEADER_WIDTH) % CELL_WIDTH;

            use Position::*;
            let position = if row == 0 && col == 0 {
                Pivot
            } else if y < 1 {
                ColumnHeader(col - 1, text_pos)
            } else if x < 3 {
                RowHeader(row - 1, text_pos)
            } else {
                let cell = state.get_at((row - 1, col - 1));
                InsideCell((row - 1, col - 1), cell, text_pos)
            };

            let bg: Color = match (&position, cursor) {
                // Highlight cell if cell is selected
                (InsideCell(address, _, _), Cursor::Single(cursor))
                    if cursor == address => Color::LightWhite,
                // Highlight cell if row is selected
                (InsideCell((row, _), _, _), Cursor::Row(cursor_row))
                    if cursor_row == row => Color::LightWhite,
                // Highlight cell if column is selected
                (InsideCell((_, col), _, _), Cursor::Column(cursor_col))
                    if cursor_col == col => Color::LightWhite,
                // cell is not selected
                (InsideCell(_, _, _), _) => Color::White,

                // Highlight row header if row is selected
                (RowHeader(row, _), Cursor::Row(cursor_row))
                    if cursor_row == row => Color::LightWhite,
                // Row is not selected
                (RowHeader(_, _), _) => Color::Gray,

                // Highlight col header if col is selected
                (ColumnHeader(col, _), Cursor::Column(cursor_col))
                    if cursor_col == col => Color::LightWhite,
                // Column is not selected
                (ColumnHeader(_, _), _) => Color::Gray,
                (Pivot, _) => Color::Black
            };

            let fg = match position {
                InsideCell(_, cell, _) if cell.computed.error
                    => Color::Red,
                _ => Color::Black
            };

            let val = match position {
                Pivot => " ",

                ColumnHeader(col, text_pos) => &State::col_name(col as u8 + 1)
                    .chars()
                    .nth(text_pos as usize)
                    .unwrap_or(' ')
                    .to_string(),

                RowHeader(row, text_pos) => &(row + 1)
                    .to_string()
                    .chars()
                    .nth_back(7 - text_pos as usize)
                    .unwrap_or(' ')
                    .to_string(),

                InsideCell(cell_addr, cell, text_pos) => {
                    let is_sole_selection = match state.cursor {
                        Cursor::Single(addr) => addr == cell_addr,
                        _ => false,
                    };

                    let mut content = match state.mode {
                        Mode::Edit if is_sole_selection => format!("={}", state.edit_buffer),
                        _ => cell.computed.display.to_string(),
                    };
                    let mut chars = content.chars();

                    let l = match &cell.alignment {
                        Alignment::Left => chars.nth(text_pos as usize),
                        Alignment::Right => chars.nth_back(8 - text_pos as usize),
                    };

                    &l.unwrap_or(' ').to_string()
                }
            };

            write!(window, "{}{}{}{}", bg.bg(), fg.fg(), val, style::Reset);
        }
    }
}
