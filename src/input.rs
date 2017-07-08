use std::io::*;
use std::sync::*;
use std::time::*;
use std::thread;

use termion::event::*;
use termion::input::TermRead;

use world::*;


pub fn input_loop(world_mutex : Arc<Mutex<World>>, stop : Arc<Mutex<bool>>, input_tx : mpsc::Sender<Key> ) {

    let stdin = stdin();
    let mut stdin_events = stdin.events();
    loop {
        let c = stdin_events.next();
        if let Ok(event) = c.unwrap() {
            match event {
                // Stop the game?
                Event::Key(Key::Char('q'))|Event::Key(Key::Esc) => {
                    let mut stop_value = stop.lock().unwrap();
                    *stop_value = true;
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
