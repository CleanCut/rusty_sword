extern crate termion;

use termion::raw::*;
use termion::clear;
use termion::color;
use termion::input::MouseTerminal;

use actor::*;
use floor::*;
use world::*;
use primitive::*;

use std::io::{Write, stdout};
use std::sync::*;
use std::thread;
use std::time::Duration;

fn render_floor<W : Write>(screen : &mut W, floor: &Floor) {
    for row in &floor.tiles {
        let mut line = String::new();
        for tile in row {
            if let Some(ref wall) = tile.wall {
                line.push_str(wall);
            } else {
                line.push_str(" "); // U-2027
                //line.push_str("‧"); // U-2027
            }
        }
        write!(screen, "{}\n\r", line).unwrap(); // U-2502
    }
}

fn render_actor<W : Write>(screen : &mut W, actor : &Box<Actor + Send>) {
    write!(screen, "{}", goto_cursor_coord(actor.coord())).unwrap();
    write!(screen, "{}", actor.symbol()).unwrap();
}

fn goto_cursor_coord(coord : &Coord) -> termion::cursor::Goto {
    // Coordinate translation naively assumes floor is being rendered at 1,1
    termion::cursor::Goto(coord.col+1, coord.row+1)
}


pub fn render_loop(world_mutex : Arc<Mutex<World>>, stop : Arc<Mutex<bool>>) {
    let mut screen = MouseTerminal::from(stdout().into_raw_mode().unwrap());
    // Hide the cursor, clear the screen
    write!(screen, "{}{}", termion::cursor::Hide, termion::clear::All).unwrap();

    {
        write!(screen, "{}", termion::cursor::Goto(1, 1)).unwrap();
        let world = world_mutex.lock().unwrap();
        render_floor(&mut screen, &world.floors[0]);
    }

    // Render Loop
    loop {
        // Time to stop?
        let stop = stop.lock().unwrap();
        if *stop {
            break;
        }

        // Get access to the world
        let world = world_mutex.lock().unwrap();

        // Render Actors
        render_actor(&mut screen, &world.player); // Player
        for actor in &world.actors {
            render_actor(&mut screen, &actor); // Others
        }

        // Bottom text
        write!(screen, "{}", termion::cursor::Goto(1, (world.floors[0].rows+1) as u16)).unwrap();
        write!(screen, "{}\n\r\n\r", world.floors[0].name).unwrap(); // Dungeon Name
        for msg in &world.messages {
            write!(screen, "{}{}", color::Fg(color::LightWhite), msg).unwrap();
            write!(screen, "{}{}\n\r", color::Fg(color::Reset), clear::UntilNewline).unwrap();
        }

        screen.flush().unwrap();

        // Don't render too hot.
        thread::sleep(Duration::from_millis(50));
    }

    // Nice cleanup: Move cursor below the world, so we can see how we finished
    let world = world_mutex.lock().unwrap();
    write!(screen, "{}", goto_cursor_coord(&Coord::at(0, (world.floors[0].rows+7) as u16))).unwrap();
    print!("{}", termion::cursor::Show);
    screen.flush().unwrap();
}

