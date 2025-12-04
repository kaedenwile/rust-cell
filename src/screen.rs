use crate::color::Color;
use crate::compute::CellComputation;
use crate::cursor::Cursor;
use crate::state::{Address, Mode, State};
use crate::window::Window;

enum Position {
    Pivot,
    ColumnHeader(u16, u16),
    RowHeader(u16, u16),
    InsideCell(Address, CellComputation, u16),
}

static CELL_WIDTH: u16 = 8;
static ROW_HEADER_WIDTH: u16 = 3;
static ELLIPSIS: char = '…';

/// Draw the spreadsheet
pub fn draw(window: &mut dyn Window, state: &State) {
    let State { cursor, scroll, .. } = state;
    let (width, height) = window.size();

    for y in 0..height {
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
                let cell = state.sheet.rendered_cells.get_at((row - 1, col - 1))
                    .cloned().unwrap_or_default();
                InsideCell((row - 1, col - 1), cell, text_pos)
            };

            let bg: Color = match (&position, cursor) {
                // Highlight cell if cell is selected
                (InsideCell(address, _, _), cursor)
                if cursor.contains(*address) => Color::LightWhite,
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

            let fg = match &position {
                InsideCell(_, cell, _) if cell.error
                => Color::Red,
                _ => Color::Black
            };

            let val = match position {
                Pivot => ' ',

                ColumnHeader(col, text_pos) => State::col_name(col as u8 + 1)
                    .chars()
                    .nth(text_pos as usize)
                    .unwrap_or(' '),

                RowHeader(row, text_pos) => (row + 1)
                    .to_string()
                    .chars()
                    .nth_back(7 - text_pos as usize)
                    .unwrap_or(' '),

                InsideCell(cell_addr, cell, text_pos) => {
                    let is_sole_selection = match state.cursor {
                        Cursor::Single(addr) => addr == cell_addr,
                        _ => false,
                    };

                    let content = match state.mode {
                        Mode::Edit if is_sole_selection => &state.edit_buffer,
                        _ => &cell.display,
                    };
                    let mut chars = content.chars();

                    // match &cell.alignment {
                    //     Alignment::Left => {
                    if content.len() > 8 && text_pos == 7 {
                        ELLIPSIS
                    } else {
                        chars.nth(text_pos as usize).unwrap_or(' ')
                    }
                    //     }
                    //     Alignment::Right => {
                    //         if content.len() > 8 && text_pos == 0 {
                    //             Some(ELLIPSIS)
                    //         } else {
                    //             chars.nth_back(8 - text_pos as usize)
                    //         }
                    //     }
                    // }.unwrap_or(' ')
                }
            };

            window.write_at(x, y, val, bg, fg);
        }
    }
}
