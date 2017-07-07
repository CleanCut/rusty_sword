use actor::*;
use floor::*;
use primitive::*;

use std::sync::*;
use std::time::Duration;

pub struct World {
    pub floor : Floor,
    pub dirty_coords : Vec<Coord>,
    pub messages : Vec<String>,
    pub players : Vec<Player>,
    pub monsters : Vec<Monster>,
}

impl World {
    pub fn new(cols : usize, rows : usize) -> Self {
        Self {
            floor: Floor::new("Dungeon Level 1", cols, rows),
            dirty_coords: Vec::<Coord>::new(),
            messages: vec!["Welcome to: Rusty Sword – Game of Infamy!".to_string()],
            players : Vec::<Player>::new(),
            monsters : Vec::<Monster>::new(),
        }
    }
    pub fn new_player(world_arc: &Arc<Mutex<World>>) {
        let world_weak = Arc::downgrade(world_arc);
        {
            let mut world = world_arc.lock().unwrap();
            world.players.push(Player::new(Coord {col: 1, row: 1}, world_weak));
        }
    }
    pub fn show_message(&mut self, msg : String) {
        self.messages.insert(0, msg);
        if self.messages.len() > 4 {
            self.messages.pop();
        }
    }

    pub fn update(&mut self, delta : Duration) {
       self.players[0].update(delta);
    }
}

