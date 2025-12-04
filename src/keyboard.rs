use termion::event::Key as TermionKey;

#[derive(Debug)]
pub enum Key {
    Char(char),
    Enter,
    Tab,
    Backspace,
    Esc,
    Ctrl(char),
    Alt(char),

    Up,
    Left,
    Right,
    Down,

    AltUp,
    AltLeft,
    AltRight,
    AltDown,

    ShiftUp,
    ShiftLeft,
    ShiftRight,
    ShiftDown,

    AltShiftUp,
    AltShiftLeft,
    AltShiftRight,
    AltShiftDown,

    Unsupported,
}

pub fn map_termion_key(input: TermionKey) -> Key {
    match input {
        TermionKey::Up => Key::Up,
        TermionKey::Left => Key::Left,
        TermionKey::Right => Key::Right,
        TermionKey::Down => Key::Down,

        TermionKey::AltUp => Key::AltUp,
        TermionKey::Alt('f') => Key::AltLeft,
        TermionKey::Alt('b') => Key::AltRight,
        TermionKey::AltDown => Key::AltDown,

        TermionKey::ShiftUp => Key::ShiftUp,
        TermionKey::ShiftLeft => Key::ShiftLeft,
        TermionKey::ShiftRight => Key::ShiftRight,
        TermionKey::ShiftDown => Key::ShiftDown,

        TermionKey::Backspace => Key::Backspace,
        TermionKey::Esc => Key::Esc,

        TermionKey::Char('\n') => Key::Enter,
        TermionKey::Char('\t') => Key::Tab,

        TermionKey::Char(c) => Key::Char(c),
        TermionKey::Ctrl(c) => Key::Ctrl(c),
        TermionKey::Alt(c) => Key::Alt(c),

        _ => Key::Unsupported,
    }
}

pub fn map_raw_code(input: Vec<u8>) -> Key {
    match input.as_slice() {
        [27, 91, 49, 59, 52, 65] => Key::AltShiftUp,
        [27, 91, 49, 59, 52, 66] => Key::AltShiftDown,
        [27, 91, 49, 59, 52, 67] => Key::AltShiftRight,
        [27, 91, 49, 59, 52, 68] => Key::AltShiftLeft,
        [27, 91, 52, 53, 59, 53, 117] => Key::Ctrl('-'),
        [27, 91, 54, 49, 59, 53, 117] => Key::Ctrl('='),
        [27, 91, 54, 49, 59, 54, 117] => Key::Ctrl('+'),
        _ => {
            // println!("{:?}", input);
            Key::Unsupported
        }
    }
}