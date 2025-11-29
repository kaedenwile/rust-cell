use signal_hook::consts::signal::SIGWINCH;
use signal_hook::iterator::Signals;
use std::io::stdin;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use termion::event::Key;
use termion::input::TermRead;

pub enum AppEvent {
    Key(Key),
    Resize,
}

pub fn events() -> Receiver<AppEvent> {
    let (tx, rx): (Sender<AppEvent>, Receiver<AppEvent>) = mpsc::channel();

    // Listen to RESIZE signals
    let tx_resize = tx.clone();
    thread::spawn(move || {
        let mut signals = Signals::new(&[SIGWINCH]).unwrap();
        for _ in &mut signals {
            // Whenever resize happens, fetch new dimensions
            tx_resize.send(AppEvent::Resize).unwrap();
        }
    });

    // Listen to keystroke events
    let tx_keys = tx.clone();
    thread::spawn(move || {
        for c in stdin().keys() {
            if let Ok(event) = c {
                tx_keys.send(AppEvent::Key(event)).unwrap();
            }
        }
    });

    rx
}