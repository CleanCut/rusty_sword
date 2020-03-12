use crate::coord::Coord;
use crate::world::World;
use crossbeam::{Receiver, Sender};
use crossterm::{style, AlternateScreen, Color, Crossterm, TerminalCursor};

trait GotoCoord {
    fn goto_coord(&self, coord: Coord);
}

impl GotoCoord for TerminalCursor {
    fn goto_coord(&self, coord: Coord) {
        self.goto(coord.col as u16, coord.row as u16).unwrap();
    }
}

pub fn render_loop(world_rx: Receiver<World>, main_tx: Sender<World>) {
    let _alt = AlternateScreen::to_alternate(true).unwrap();
    let crossterm = Crossterm::new();
    let cursor = crossterm.cursor();
    cursor.hide().unwrap();

    // Draw the entire floor - we only have to do this once
    let mut world = world_rx.recv().unwrap();
    let game_title_coord = Coord::new(world.floor.rows, 0);
    cursor.goto_coord(Coord::new(0, 0));
    {
        let tiles = &world.floor.tiles;
        for row in tiles {
            for tile in row {
                print!("{}", tile);
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
        for coord in world.dirty_coords.drain(..) {
            cursor.goto_coord(coord);
            print!("{}", world.floor.get_symbol(coord));
        }

        // Render Player
        let player = &mut world.player;
        if player.dirty {
            player.dirty = false;
            // Player's sword
            cursor.goto_coord(player.sword_coord);
            print!("{}", style(player.sword_symbol()).with(Color::Red));
            // Player himself
            cursor.goto_coord(player.coord);
            print!("{}", style(&player.symbol).with(Color::Blue));
        }
        // Player Score
        let score_string = format!("Score: {}", player.score);
        cursor.goto_coord(Coord::new(
            world.floor.rows,
            world.floor.cols - score_string.len(),
        ));
        print!("{}", style(score_string).with(Color::Blue));

        // Render Monsters
        let monsters = &mut world.monsters;
        for monster in monsters.iter() {
            cursor.goto_coord(monster.coord);
            print!("{}", style(&monster.symbol).with(Color::Green));
        }

        // Game Title
        cursor.goto_coord(game_title_coord);
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
