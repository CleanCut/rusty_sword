pub extern crate rusty_sword;
use rusty_sword::*;

fn main() {
    // To avoid lock contention for this group of objects, we'll follow the rule:
    // - You must have a lock on floor before trying to lock anything else
    // - You must unlock all other locks before (or when) floor gets unlocked
    let floor        = Arc::new(Mutex::new(Floor::new(60, 30)));
    let player       = Arc::new(Mutex::new(Player::new(Coord::new(30, 15))));
    let dirty_coords = Arc::new(Mutex::new(Vec::<Coord>::new()));
    let monsters     = Arc::new(Mutex::new(Vec::<Monster>::new()));

    // To avoid lock contention, we'll follow the rule:
    // - stop should be locked only when no other objects are locked
    let stop = Arc::new(Mutex::new(false));

    // Render Thread
    let render_thread = {
        let stop = stop.clone();
        let floor = floor.clone();
        let player = player.clone();
        let dirty_coords = dirty_coords.clone();
        let monsters = monsters.clone();
        spawn(move || { render_loop(stop, floor, player, dirty_coords, monsters) } )
    };

    // Sound Thread
    let (sound_tx, sound_rx) = mpsc::channel::<&str>();
    let sound_thread = {
        let stop = stop.clone();
        spawn(move || { sound_loop(stop, sound_rx) } )
    };

    // Game Loop
    let mut quit = false;
    let mut astdin = async_stdin();
    let mut rng = rand::thread_rng();
    let mut spawn_timer = Timer::from_millis(1000);
    let mut last_instant = Instant::now();
    loop {
        sleep(Duration::from_millis(10));
        if quit {
            sleep(Duration::from_millis(50));
            *stop.lock().unwrap() = true;
        }
        {
            if *stop.lock().unwrap() {
                break;
            }
        }
        // Lock floor first!
        let floor = floor.lock().unwrap();
        let mut player = player.lock().unwrap();
        let mut dirty_coords = dirty_coords.lock().unwrap();
        let mut monsters = monsters.lock().unwrap();

        let current_instant = Instant::now();
        let delta = current_instant - last_instant;

        // Player moves?
        let mut player_moved = false;
        let mut bytebuf : [u8; 1] = [0];
        while let Ok(amount) = astdin.read(&mut bytebuf) {
            if amount == 1 {
                match bytebuf[0] {
                    27|b'q' => {
                        quit = true;
                    },
                    _ => {
                        if let Some(direction) = byte_to_direction(bytebuf[0]) {
                            player_moved = player.travel(direction, &floor, &mut dirty_coords);
                        }
                    },
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
            sound_tx.send("monster_dies").unwrap();
        }

        // Spawn a new monster!
        spawn_timer.update(delta);
        if spawn_timer.ready {
            spawn_timer = Timer::from_millis(sample(&mut rng, 1000..5000, 1)[0]);
            let to_coord = Coord::new(
                sample(&mut rng, 1..59, 1)[0],
                sample(&mut rng, 1..29, 1)[0],
            );
            if to_coord != player.coord {
                monsters.push(Monster::new(to_coord, &mut rng));
                sound_tx.send("monster_spawns").unwrap();
            }
        }

        // Did the player die?
        if monsters.iter().any(|monster| monster.coord == player.coord) {
            quit = true;
            sound_tx.send("player_dies").unwrap();
        }

        last_instant = current_instant;
    }

    // Wait for other threads to stop before exiting
    render_thread.join().unwrap();
    println!("Thanks for playing!");
    sound_thread.join().unwrap();
}

