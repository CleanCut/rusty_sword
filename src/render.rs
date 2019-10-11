
use std::sync::{Arc, Mutex};
use crate::floor::Floor;
use crate::player::{Player, sword_symbol};
use crate::coord::Coord;
use crate::monster::Monster;
use std::io::{stdout, Write, Stdout};
use std::time::Duration;
use std::thread::sleep;
use crossterm::{style, AlternateScreen, ClearType, Color, Crossterm, Result, RawScreen, TerminalCursor};

pub fn render_loop(
    stop: Arc<Mutex<bool>>,
    floor: Arc<Mutex<Floor>>,
    player: Arc<Mutex<Player>>,
    dirty_coords: Arc<Mutex<Vec<Coord>>>,
    monsters: Arc<Mutex<Vec<Monster>>>,
) {
    let _raw = RawScreen::into_raw_mode().unwrap();
    let crossterm = Crossterm::new();
    let terminal = crossterm.terminal();
    let cursor = crossterm.cursor();

    terminal.clear(ClearType::All).unwrap();

    cursor.hide().unwrap();
    // Draw the entire floor
    cursor.goto(0, 0);
    {
        let floor = floor.lock().unwrap();
        for row in &floor.tiles {
            for tile in row {
                if let Some(ref wall) = tile.wall {
                    print!("{}", wall);
                } else {
                    print!(" ");
                }
            }
            print!("\r\n");
        }
    }

    loop {
        sleep(Duration::from_millis(10));
        if *stop.lock().unwrap() {
            break;
        }
        // Lock floor first...
        let floor = floor.lock().unwrap();

        // Redraw any dirty coordinates with floor tiles
        let mut dirty_coords = dirty_coords.lock().unwrap();
        for coord in dirty_coords.drain(..) {
            cursor.goto(coord.col, coord.row);
            print!("{}", floor.get_symbol(coord));
        }

        // Render Player
        let mut player = player.lock().unwrap();
        if player.dirty {
            player.dirty = false;
            // Player's sword
            cursor.goto(player.sword_coord.col, player.sword_coord.row);
            print!("{}", style(sword_symbol(player.facing)).with(Color::Red));
            // Player himself
            cursor.goto(player.coord.col, player.coord.row);
            print!("{}", style(&player.symbol).with(Color::Blue));
        }
        // Player Score
        let score_string = format!("Score: {}", player.score);
        cursor.goto((floor.cols - score_string.len()) as u16, floor.rows as u16);
        print!("{}", style(score_string).with(Color::Blue));

        // Render Monsters
        let monsters = monsters.lock().unwrap();
        for monster in monsters.iter() {
            cursor.goto(monster.coord.col, monster.coord.row);
            print!("{}", style(&monster.symbol).with(Color::Green));
        }

        // Game Title
        cursor.goto(0, floor.rows as u16);
        print!("{}", style("Rusty Sword - Game of Infamy!").with(Color::White));
    }

    // Nice cleanup: Move cursor below the floor, so we can see how we finished
    {
        let floor = floor.lock().unwrap();
        cursor.goto(0, (floor.rows + 2) as u16);
        RawScreen::disable_raw_mode().unwrap();
    }
    cursor.show().unwrap();
}

//fn out<S: ToString>(screen: &mut RawTerminal<Stdout>, output: S) {
//    //write!(*screen, "{}", output.to_string()).unwrap();
//}