# Cell

A terminal based spreadsheet program built in Rust.

<img width="724" alt="Screenshot 2024-12-14 at 9 01 29 PM" src="https://github.com/user-attachments/assets/fed94e68-c7c2-4ee8-80cd-70bb8955aa07" />

## Background

This is my first time using Rust, and I thought a terminal spreadsheet program would be a great project to get started
with. This was only intended to be a toy spreadsheet, but it should be able to read/write arbitrary CSV files and do
simple math.

The [Rust by Example](https://doc.rust-lang.org/rust-by-example/index.html) book was a very helpful resource along the way.

## Controls

The app is split into **Nav** mode and **Edit** mode.

When in **Nav** mode:
* Press `q` to exit the program
* Press `ctrl+s` to save the file (then type in a file name and press `enter`)
* Use the `arrow keys` to move around
* Press `=` to start editing a cell (see **Edit** mode controls below)
* Press `Backspace` to clear a cell
* Use `wasd` to scroll the viewport
* Press `ctrl+z` to undo
* Press `ctrl+y` to redo

When in **Edit** mode:
* Press `enter` to finish editing the cell
* Press `escape` to abort your changes
* Use `left-arrow` and `right-arrow` to move the cursor

## Feature List

* [x] Status bar
* [x] Compute values
* [x] Edit cursor
* [x] Compute with cell references (A1, B2)
* [x] Support for string values
* [x] Undo / redo
* [x] Save / load
* [x] Pretty error messages
