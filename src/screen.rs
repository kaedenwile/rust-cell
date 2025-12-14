use crate::color::Color;
use crate::compute::CellComputation;
use crate::cursor::Cursor;
use crate::state::{Address, Mode, State};
use crate::styling::CellStyles;
use crate::window::Window;

enum Position {
    Pivot,
    ColumnHeader(u16),
    RowHeader(u16),
    InsideCell(Address, CellComputation, CellStyles),
}

// Default cell width
pub static CELL_WIDTH: u16 = 8;
static ROW_HEADER_WIDTH: u16 = 3;
static ELLIPSIS: char = '…';

/// Draw the spreadsheet
pub fn draw(window: &mut dyn Window, state: &State) {
    let State { cursor, .. } = state;
    let (width, height) = window.size();

    for y in 0..height {
        let row = y;
        let mut col: u16 = 0;
        let mut col_width = ROW_HEADER_WIDTH;
        let mut col_width_remaining = ROW_HEADER_WIDTH + 1;

        for x in 0..width {
            // position on screen
            col_width_remaining = col_width_remaining - 1;
            if col_width_remaining == 0 {
                col += 1;
                col_width = state.sheet.column_widths.get((col - 1) as usize).cloned().unwrap_or(CELL_WIDTH);
                col_width_remaining = col_width;
            }
            let text_pos = col_width_remaining;

            use Position::*;
            let position = if row == 0 && col == 0 {
                Pivot
            } else if y < 1 {
                ColumnHeader(col - 1)
            } else if x < 3 {
                RowHeader(row - 1)
            } else {
                let addr = (row - 1, col - 1);
                let cell = state.sheet.rendered_cells.get_at(addr).cloned().unwrap_or_default();
                let styles = state.sheet.cell_styling.get_at(addr).cloned().unwrap_or_default();
                InsideCell(addr, cell, styles)
            };

            let bg: Color = match (&position, cursor) {
                // Apply formatting background if in format mode and cell is selected
                (InsideCell(address, _, _), cursor)
                if cursor.contains(*address) && matches!(state.mode, Mode::Format) => state.styling_state.bg,
                // Highlight cell if cell is selected
                (InsideCell(address, _, _), cursor)
                if cursor.contains(*address) => Color::CursorWhite,
                // Normal cell background
                (InsideCell(_, _, CellStyles { bg, .. }), _) => *bg,

                // Highlight row header if row is selected
                (RowHeader(row), Cursor::Row(cursor_row))
                if cursor_row == row => Color::CursorWhite,
                // Row is not selected
                (RowHeader(_), _) => Color::Gray,

                // Highlight col header if col is selected
                (ColumnHeader(col), Cursor::Column(cursor_col))
                if cursor_col == col => Color::CursorWhite,
                // Column is not selected
                (ColumnHeader(_), _) => Color::Gray,

                // Pivot cell background
                (Pivot, _) => Color::Black
            };

            let fg = match &position {
                // Apply formatting colors if in format mode and cell is selected
                InsideCell(address, _, _)
                if cursor.contains(*address) && matches!(state.mode, Mode::Format) => state.styling_state.fg,
                // Error cells are red
                InsideCell(_, cell, _) if cell.error => Color::Red,
                // Normal cell foreground
                InsideCell(_, _, CellStyles { fg, .. }) => *fg,
                // Headers foreground
                _ => Color::Black,
            };

            let CellStyles { bold, italic, underline, .. } = match &position {
                // Apply formatting styles if in format mode and cell is selected
                InsideCell(address, _, _)
                if cursor.contains(*address) && matches!(state.mode, Mode::Format) =>
                    state.styling_state.to_cell_styles(),
                // Normal cell styles
                InsideCell(_, _, cell_styles) => cell_styles.clone(),
                // Headers are default style
                _ => CellStyles::default(),
            };

            let val = match position {
                Pivot => ' ',

                ColumnHeader(col) => State::col_name(col as u8 + 1)
                    .chars()
                    .nth((col_width - text_pos) as usize)
                    .unwrap_or(' '),

                RowHeader(row) => (row + 1)
                    .to_string()
                    .chars()
                    .nth_back((col_width_remaining - 1) as usize)
                    .unwrap_or(' '),

                InsideCell(cell_addr, cell, _) => {
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
                    if content.len() > col_width as usize && text_pos == 1 {
                        ELLIPSIS
                    } else {
                        chars.nth((col_width - col_width_remaining) as usize).unwrap_or(' ')
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

            window.write_at(x, y, val, bg, fg, bold, italic, underline);
        }
    }
}
