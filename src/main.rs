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
    let world = Arc::new(Mutex::new(
            World {
                floors: vec![Floor::new("Dungeon Level 1", 30, 60)],
                actors: vec![
                    Box::new(Monster { _coord: Coord { col: 20, row: 20} }),
                ],
                dirty_coords: Vec::<Coord>::new(),
                player: Box::new(Player::new(1, 1)),
                messages: Vec::<String>::new(),
            }));
    let stop = Arc::new(Mutex::new(false));

    // Render Thread
    let render_thread = {
        let world = world.clone();
        let stop = stop.clone();
        thread::spawn(move || { render_loop(world, stop) })
    };

    // Input Thread
    let input_thread = {
        let world = world.clone();
        let stop = stop.clone();
        thread::spawn(move || { input_loop(world, stop) })
    };

    // Game Thread
    let game_thread = {
        let world = world.clone();
        let stop = stop.clone();
        thread::spawn(move || { game_loop(world, stop) })
    };

    {
        let mut world = world.lock().unwrap();
        world.show_message("Welcome to: Rusty Sword – Game of Infamy!".to_string())
    }

    // Wait for other threads to stop before exiting
    render_thread.join().unwrap();
    input_thread.join().unwrap();
    game_thread.join().unwrap();
    println!("Thanks for playing!");
}
