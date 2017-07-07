use std::sync::*;
use std::time::*;
use std::thread;

use actor::*;
use world::*;


pub fn game_loop(world_mutex : Arc<Mutex<World>>, stop : Arc<Mutex<bool>>) {
    let mut last_instant = Instant::now();
    loop {
        let stop = stop.lock().unwrap();
        if *stop {
            break;
        }

        let mut world = world_mutex.lock().unwrap();
        let current_instant = Instant::now();
        let delta = current_instant - last_instant;

        world.players[0].update(delta);

        last_instant = current_instant;
        thread::sleep(Duration::from_millis(10));
    }

}
