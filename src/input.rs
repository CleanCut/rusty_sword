use std::io::*;
use std::sync::*;
use std::time::*;
use std::thread;

use termion::event::*;
use termion::input::TermRead;

use world::*;


pub fn input_loop(world_mutex : Arc<Mutex<World>>, stop : Arc<Mutex<bool>>) {

    let stdin = stdin();
    let mut stdin_events = stdin.events();
    loop {
        // Handle input
        let c = stdin_events.next();
        if let Ok(event) = c.unwrap() {
            match event {
                Event::Key(k_event) => {
                    match k_event {
                        Key::Char('q')|Key::Esc => break,
                        Key::Char(ch) => {
                        },
                        _ => {},
                    }
                }
                Event::Mouse(m_event) => {
                    match m_event {
                        MouseEvent::Press(_, col, row) => {
                            let mut world = world_mutex.lock().unwrap();
                            world.show_message(format!("Mouse at {}, {}", col, row));
                        },
                        _ => {},
                    }
                }
                _ => {},
            }
        }
        thread::sleep(Duration::from_millis(10));
    }

    // Signal other threads to stop
    {
        let mut stop_value = stop.lock().unwrap();
        *stop_value = true;
    }
}
