extern crate rusty_sword;
extern crate termion;

use std::sync::*;
use std::thread;

use rusty_sword::floor::Floor;
use rusty_sword::actor::{Monster, Player};
use rusty_sword::primitive::Coord;
use rusty_sword::world::World;
use rusty_sword::render::render_loop;
use rusty_sword::input::input_loop;
use rusty_sword::game::game_loop;

fn main() {
    let world_arc = Arc::new(Mutex::new(
            World {
                floor: Floor::new("Dungeon Level 1", 60, 30),
                actors: vec![
                ],
                dirty_coords: Vec::<Coord>::new(),
                player: Player { _coord: Coord { col: 1, row: 1} },
                messages: Vec::<String>::new(),
            }));
    {
        let world = world_arc.lock().unwrap();
        world.show_message("Welcome to: Rusty Sword – Game of Infamy!".to_string());
        world.add_actor(Box::new(Monster::new(Coord { col: 20, row: 20} })));
    }
    let stop = Arc::new(Mutex::new(false));

    let (dirty_coord_tx, dirty_coord_rx) = mpsc::channel::<Coord>();

    // Render Thread
    let render_thread = {
        let world = world_arc.clone();
        let stop = stop.clone();
        thread::spawn(move || { render_loop(world, stop, dirty_coord_rx) })
    };

    // Input Thread
    let dirty_coord_tx2 = dirty_coord_tx.clone();
    let input_thread = {
        let world = world_arc.clone();
        let stop = stop.clone();
        thread::spawn(move || { input_loop(world, stop, dirty_coord_tx2) })
    };

    // Game Thread
    let game_thread = {
        let world = world_arc.clone();
        let stop = stop.clone();
        thread::spawn(move || { game_loop(world, stop) })
    };

    // Wait for other threads to stop before exiting
    render_thread.join().unwrap();
    input_thread.join().unwrap();
    game_thread.join().unwrap();
    println!("Thanks for playing!");
}
