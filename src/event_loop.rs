use crate::keyboard::{map_raw_code, map_termion_key, Key};
use signal_hook::consts::signal::SIGWINCH;
use signal_hook::iterator::Signals;
use std::io::stdin;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
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
        for event in stdin().events() {
            match event.unwrap() {
                termion::event::Event::Key(termion_key) => {
                    let key = map_termion_key(termion_key);
                    tx_keys.send(AppEvent::Key(key)).unwrap();
                }
                termion::event::Event::Unsupported(raw_code) => {
                    let key = map_raw_code(raw_code);
                    tx_keys.send(AppEvent::Key(key)).unwrap();
                }
                _ => {}
            }
        }
    });

    rx
}