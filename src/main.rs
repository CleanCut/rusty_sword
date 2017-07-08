extern crate rusty_sword;
extern crate termion;

use std::sync::*;
use std::thread;
use std::time::*;

use termion::event::*;

use rusty_sword::input::*;
use rusty_sword::primitive::*;
use rusty_sword::render::*;
use rusty_sword::world::*;

fn main() {
    let world = Arc::new(Mutex::new(World::new()));

    let stop = Arc::new(Mutex::new(false));

    let (dirty_coord_tx, dirty_coord_rx) = mpsc::channel::<Coord>();
    let (input_tx, input_rx) = mpsc::channel::<Key>();

    // Render Thread
    let render_thread = {
        let world = world.clone();
        let stop = stop.clone();
        thread::spawn(move || { render_loop(world, stop, dirty_coord_rx) })
    };

    // Input Thread
    let input_thread = {
        let world = world.clone();
        let stop = stop.clone();
        let input_tx = input_tx.clone();
        thread::spawn(move || { input_loop(world, stop, input_tx) })
    };

    //-------------------------------------------------------------------------
    // Game Loop
    let mut last_instant = Instant::now();
    'game: loop {
        {
            let stop_value = stop.lock().unwrap();
            if *stop_value {
                break;
            }
        }
        let current_instant = Instant::now();
        let delta = current_instant - last_instant;

        // Handle input sent by the Input Thread
        while let Ok(key) = input_rx.try_recv() {
            match key {
                Key::Char('q') => break 'game,
                Key::Char(ch) => {
                    //match ch {
                    //    'a' =>
                    //}
                    let mut world = world.lock().unwrap();
                    world.show_message(ch.to_string());
                },
                _ => {},
            }
        }


        last_instant = current_instant;
        thread::sleep(Duration::from_millis(10));
    }
    // End game loop

    // Wait for other threads to stop before exiting
    render_thread.join().unwrap();
    input_thread.join().unwrap();
    println!("Thanks for playing!");
}
