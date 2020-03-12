use crossbeam::bounded;
use crossterm::{InputEvent, KeyEvent, TerminalInput};
use rand::distributions::Uniform;
use rand::prelude::Distribution;
use rusty_audio::Audio;
use rusty_sword::coord::{key_to_direction, Coord};
use rusty_sword::monster::Monster;
use rusty_sword::render::render_loop;
use rusty_sword::timer::Timer;
use rusty_sword::world::World;
use std::thread::{sleep, spawn};
use std::time::{Duration, Instant};

fn main() {
    let mut audio = Audio::new();
    audio.add("monster_dies", "clips/monster_dies.wav");
    audio.add("monster_spawns", "clips/monster_spawns.wav");
    audio.add("player_dies", "clips/player_dies.wav");

    let mut world = World::new();

    // We'll use this to let the render thread know we're done.
    let (render_tx, render_rx) = bounded::<World>(0);
    let (main_tx, main_rx) = bounded::<World>(0);

    // Render Thread
    let render_thread = { spawn(move || render_loop(render_rx, main_tx)) };

    // Game Loop
    sleep(Duration::from_millis(100));
    let input = TerminalInput::new();
    let mut reader = input.read_async();
    let mut rng = rand::thread_rng();
    let mut spawn_timer = Timer::from_millis(1000);
    let mut last_instant = Instant::now();
    'gameloop: loop {
        let delta = last_instant.elapsed();
        last_instant = Instant::now();
        let mut player = &mut world.player;

        // Player moves?
        let mut player_moved = false;
        loop {
            match reader.next() {
                Some(event) => match event {
                    InputEvent::Keyboard(KeyEvent::Char('q'))
                    | InputEvent::Keyboard(KeyEvent::Esc) => break 'gameloop,
                    InputEvent::Keyboard(k) => {
                        if let Some(direction) = key_to_direction(k) {
                            player_moved =
                                player.travel(direction, &world.floor, &mut world.dirty_coords);
                        }
                    }
                    _ => {}
                },
                None => break,
            }
        }

        // Update monster timers
        for monster in world.monsters.iter_mut() {
            monster.move_timer.update(delta);
        }

        // Monsters move?
        if !player_moved {
            for monster in world.monsters.iter_mut() {
                monster.try_travel(player.coord, &mut world.dirty_coords);
            }
        }

        // Did a monster die?
        let num_monsters = world.monsters.len();
        world
            .monsters
            .retain(|monster| monster.coord != player.sword_coord);
        let num_killed = num_monsters - world.monsters.len();
        if num_killed > 0 {
            player.score += num_killed as u64;
            audio.play("monster_dies");
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
                world.monsters.push(Monster::new(to_coord, &mut rng));
                audio.play("monster_spawns");
            }
        }

        // Did the player die?
        if world
            .monsters
            .iter()
            .any(|monster| monster.coord == player.coord)
        {
            audio.play("player_dies");
            break 'gameloop;
        }

        // Give the whole world to the renderer - we're just having fun with channels, we're not
        // getting any performance gains since the main and render thread only run when they've got
        // access to the world
        render_tx.send(world).unwrap();
        world = main_rx.recv().unwrap();
        sleep(Duration::from_millis(10));
    }

    // Wait until currently-playing sounds are done
    audio.wait();
    // Close the render_tx channel, which will trigger the render thread to exit
    drop(render_tx);
    // Wait for the render thread to actually exit
    render_thread.join().unwrap();

    println!("Thanks for playing!");
}
