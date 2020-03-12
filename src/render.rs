use crate::world::World;
use crossbeam::{Receiver, Sender};
use crossterm::{style, AlternateScreen, Color, Crossterm};

pub fn render_loop(world_rx: Receiver<World>, main_tx: Sender<World>) {
    let _alt = AlternateScreen::to_alternate(true).unwrap();
    let crossterm = Crossterm::new();
    let cursor = crossterm.cursor();
    cursor.hide().unwrap();

    // Draw the entire floor - we only have to do this once
    let mut world = world_rx.recv().unwrap();
    cursor.goto(0, 0).unwrap();
    {
        let tiles = &world.floor.tiles;
        for row in tiles {
            for tile in row {
                if let Some(wall) = tile {
                    print!("{}", wall);
                } else {
                    print!(" ");
                }
            }
            print!("\r\n");
        }
    }
    main_tx.send(world).unwrap();

    loop {
        world = match world_rx.recv() {
            Ok(w) => w,
            Err(_) => {
                break;
            }
        };

        // Redraw any dirty coordinates with floor tiles
        let dirty_coords = &mut world.dirty_coords;
        let floor = &world.floor;
        for coord in dirty_coords.drain(..) {
            cursor.goto(coord.col, coord.row).unwrap();
            print!("{}", floor.get_symbol(coord));
        }

        // Render Player
        let player = &mut world.player;
        if player.dirty {
            player.dirty = false;
            // Player's sword
            cursor
                .goto(player.sword_coord.col, player.sword_coord.row)
                .unwrap();
            print!("{}", style(player.sword_symbol()).with(Color::Red));
            // Player himself
            cursor.goto(player.coord.col, player.coord.row).unwrap();
            print!("{}", style(&player.symbol).with(Color::Blue));
        }
        // Player Score
        let score_string = format!("Score: {}", player.score);
        cursor
            .goto((floor.cols - score_string.len()) as u16, floor.rows as u16)
            .unwrap();
        print!("{}", style(score_string).with(Color::Blue));

        // Render Monsters
        let monsters = &mut world.monsters;
        for monster in monsters.iter() {
            cursor.goto(monster.coord.col, monster.coord.row).unwrap();
            print!("{}", style(&monster.symbol).with(Color::Green));
        }

        // Game Title
        cursor.goto(0, floor.rows as u16).unwrap();
        print!(
            "{}",
            style("Rusty Sword - Game of Infamy!").with(Color::White)
        );
        if main_tx.send(world).is_err() {
            break;
        }
    }
    // cleanup
    cursor.show().unwrap();
}
