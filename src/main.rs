extern crate rand;
extern crate rusty_sword;
extern crate termion;

use std::sync::*;
use std::thread;
use std::time::*;

use termion::event::*;

use rusty_sword::actor::*;
use rusty_sword::floor::*;
use rusty_sword::input::*;
use rusty_sword::primitive::*;
use rusty_sword::render::*;

fn main() {
    let mut rng = rand::thread_rng();

    // To avoid lock contention for this group of objects, we'll follow the rule:
    // - You must have a lock on floor before trying to lock anything else
    // - You must not keep any locks when floor gets unlocked
    let floor        = Arc::new(Mutex::new(Floor::new("Dungeon Level 1", 60, 30)));
    let dirty_coords = Arc::new(Mutex::new(Vec::<Coord>::new()));
    let messages     = Arc::new(Mutex::new(vec!["Welcome to: Rusty Sword – Game of Infamy!".to_string()]));
    let player       = Arc::new(Mutex::new(Player::new(Coord {col: 5, row: 1})));
    let monsters     = Arc::new(Mutex::new(vec![Monster::new(Coord {col: 1, row: 1}, &mut rng)]));

    // `stop` is not related to the objects above. To avoid lock contention, we'll follow the rule:
    // - stop should be locked and released when no other objects are locked
    let stop = Arc::new(Mutex::new(false));


    // Render Thread
    let render_thread = {
        let floor = floor.clone();
        let dirty_coords = dirty_coords.clone();
        let messages = messages.clone();
        let player = player.clone();
        let monsters = monsters.clone();
        let stop = stop.clone();
        thread::spawn(move || { render_loop(floor, dirty_coords, messages, player, monsters, stop) })
    };

    // Input Thread
    let (input_tx, input_rx) = mpsc::channel::<Key>();
    let input_thread = {
        let stop = stop.clone();
        let input_tx = input_tx.clone();
        thread::spawn(move || { input_loop(stop, input_tx) })
    };

    //-------------------------------------------------------------------------
    // Game Loop
    let mut last_instant = Instant::now();
    'game: loop {
        thread::sleep(Duration::from_millis(10));
        // Time to stop?
        {
            if *stop.lock().unwrap() {
                break;
            }
        }
        // Once we can lock floor, we can lock anything else we want in this thread.
        let floor = floor.lock().unwrap();
        let mut dirty_coords = dirty_coords.lock().unwrap();
        let mut player = player.lock().unwrap();

        let current_instant = Instant::now();
        let delta = current_instant - last_instant;

        // Handle input sent by the Input Thread
        while let Ok(key) = input_rx.try_recv() {
            match key {
                Key::Char(ch) => {
                    if let Some(direction) = char_to_direction(ch) {
                        player.travel(direction, &floor, &mut dirty_coords);
                    }
                },
                _ => {},
            }
        }

        last_instant = current_instant;
    }
    // End game loop

    // Wait for other threads to stop before exiting
    render_thread.join().unwrap();
    input_thread.join().unwrap();
    println!("Thanks for playing!");
}
