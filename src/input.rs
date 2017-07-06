use std::io::*;
use std::sync::*;
use std::time::*;
use std::thread;

use termion::event::*;
use termion::input::TermRead;

use world::*;
use primitive::*;


pub fn input_loop(world_mutex : Arc<Mutex<World>>, stop : Arc<Mutex<bool>>, mut dirty_coord_tx : mpsc::Sender<Coord>) {

    let stdin = stdin();
    let mut stdin_events = stdin.events();
    'outer: loop {
        // Handle input
        loop {
            let c = stdin_events.next();
            if let Ok(event) = c.unwrap() {
                match event {
                    Event::Key(k_event) => {
                        match k_event {
                            Key::Char('q')|Key::Esc => break 'outer,
                            Key::Char(ch) => {
                                let mut world = world_mutex.lock().unwrap();
                                world.player.handle_input(&ch, &mut dirty_coord_tx);
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
            } else {
                break;
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
