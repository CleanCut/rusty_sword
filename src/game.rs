use std::sync::*;
use std::time::*;
use std::thread;

use world::*;


pub fn game_loop(world_mutex : Arc<Mutex<World>>, stop : Arc<Mutex<bool>>) {
    loop {
        //let world = world_mutex.lock().unwrap();
        let stop = stop.lock().unwrap();

        if *stop {
            break;
        }
        thread::sleep(Duration::from_millis(10));
    }

}
