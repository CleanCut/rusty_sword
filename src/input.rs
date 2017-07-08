use std::io::*;
use std::sync::*;
use std::time::*;
use std::thread;

use termion::event::*;
use termion::input::TermRead;

pub fn input_loop(stop : Arc<Mutex<bool>>, input_tx : mpsc::Sender<Key> ) {

    let stdin = stdin();
    let mut stdin_events = stdin.events();
    loop {
        let c = stdin_events.next();
        if let Ok(event) = c.unwrap() {
            match event {
                // Stop the game?
                Event::Key(Key::Esc) => {
                    *stop.lock().unwrap() = true;
                    break;
                },
                Event::Key(k) => {
                    if let Err(_) = input_tx.send(k) {}
                },
                _ => {},
            }
        }
        thread::sleep(Duration::from_millis(10));
    }

}
