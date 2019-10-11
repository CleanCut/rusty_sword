use crossterm::{InputEvent, KeyEvent, TerminalInput};
use rand::distributions::Uniform;
use rand::prelude::Distribution;
use rusty_sword::coord::{key_to_direction, Coord};
use rusty_sword::floor::Floor;
use rusty_sword::monster::Monster;
use rusty_sword::player::Player;
use rusty_sword::render::render_loop;
use rusty_sword::audio::audio_loop;
use rusty_sword::timer::Timer;
use std::sync::{Arc, Mutex};
use std::thread::{sleep, spawn};
use std::time::{Duration, Instant};
use crossbeam::unbounded;

fn main() {
    // To avoid lock contention for this group of objects, we'll follow the rule:
    // - You must have a lock on floor before trying to lock anything else
    // - You must unlock all other locks before (or when) floor gets unlocked
    let floor = Arc::new(Mutex::new(Floor::new(60, 30)));
    let player = Arc::new(Mutex::new(Player::new(Coord::new(30, 15))));
    let dirty_coords = Arc::new(Mutex::new(Vec::<Coord>::new()));
    let monsters = Arc::new(Mutex::new(Vec::<Monster>::new()));

    // We'll use this to let the render thread know we're done.
    let (stop_tx, stop_rx) = unbounded::<()>();

    // Render Thread
    let render_thread = {
        let floor = floor.clone();
        let player = player.clone();
        let dirty_coords = dirty_coords.clone();
        let monsters = monsters.clone();
        spawn(move || render_loop(stop_rx, floor, player, dirty_coords, monsters))
    };

    // Audio Thread
    let (clip_tx, clip_rx) = unbounded::<&str>();
    let audio_thread = { spawn(move || audio_loop(clip_rx)) };

    // Game Loop
    sleep(Duration::from_millis(100));
    let input = TerminalInput::new();
    let mut reader = input.read_async();
    let mut rng = rand::thread_rng();
    let mut spawn_timer = Timer::from_millis(1000);
    let mut last_instant = Instant::now();
    'gameloop: loop {
        sleep(Duration::from_millis(10));
        // Lock floor first!
        let floor = floor.lock().unwrap();
        let mut player = player.lock().unwrap();
        let mut dirty_coords = dirty_coords.lock().unwrap();
        let mut monsters = monsters.lock().unwrap();

        let current_instant = Instant::now();
        let delta = current_instant - last_instant;

        // Player moves?
        let mut player_moved = false;
        loop {
            if let Some(event) = reader.next() {
                match event {
                    InputEvent::Keyboard(KeyEvent::Char('q'))
                    | InputEvent::Keyboard(KeyEvent::Esc) => break 'gameloop,
                    InputEvent::Keyboard(k) => {
                        if let Some(direction) = key_to_direction(k) {
                            player_moved = player.travel(direction, &floor, &mut dirty_coords);
                        }
                    }
                    _ => {}
                }
            } else {
                break;
            }
        }

        // Update monster timers
        for monster in monsters.iter_mut() {
            monster.move_timer.update(delta);
        }

        // Monsters move?
        if !player_moved {
            for monster in monsters.iter_mut() {
                monster.try_travel(player.coord, &mut dirty_coords);
            }
        }

        // Did a monster die?
        let num_monsters = monsters.len();
        monsters.retain(|monster| monster.coord != player.sword_coord);
        let num_killed = num_monsters - monsters.len();
        if num_killed > 0 {
            player.score += num_killed as u64;
            clip_tx.send("monster_dies").unwrap();
        }

        // Spawn a new monster!
        spawn_timer.update(delta);
        if spawn_timer.ready {
            spawn_timer = Timer::from_millis(Uniform::new(1000, 5000).sample(&mut rng));
            let to_coord = Coord::new(
                Uniform::new(1, 59).sample(&mut rng),
                Uniform::new(1, 29).sample(&mut rng),
            );
            if to_coord != player.coord {
                monsters.push(Monster::new(to_coord, &mut rng));
                clip_tx.send("monster_spawns").unwrap();
            }
        }

        // Did the player die?
        if monsters.iter().any(|monster| monster.coord == player.coord) {
            clip_tx.send("player_dies").unwrap();
            break 'gameloop;
        }

        last_instant = current_instant;
    }

    // Game ended
    //sleep(Duration::from_millis(50));
    stop_tx.send(()).unwrap();
    clip_tx.send("stop").unwrap();

    // Wait for other threads to gracefully exit before exiting the main thread
    render_thread.join().unwrap();
    println!("Thanks for playing!");
    audio_thread.join().unwrap();
}
