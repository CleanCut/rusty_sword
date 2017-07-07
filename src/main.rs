extern crate rusty_sword;
extern crate termion;

use std::sync::*;
use std::thread;

use rusty_sword::game::*;
use rusty_sword::input::*;
use rusty_sword::primitive::*;
use rusty_sword::render::*;
use rusty_sword::world::*;

fn main() {
    let world_arc = Arc::new(Mutex::new(World::new(60, 30)));
    World::new_player(&world_arc);

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
