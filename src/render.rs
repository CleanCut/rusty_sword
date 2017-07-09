use ::*;

fn cursor_coord(coord : Coord) -> termion::cursor::Goto {
    // Coordinate translation naively assumes floor is being rendered at 1,1
    termion::cursor::Goto(coord.col+1, coord.row+1)
}

pub fn render_loop(floor        : Arc<Mutex<Floor>>,
                   dirty_coords : Arc<Mutex<Vec<Coord>>>,
                   messages     : Arc<Mutex<Vec<String>>>,
                   player       : Arc<Mutex<Player>>,
                   monsters     : Arc<Mutex<Vec<Monster>>>,
                   stop         : Arc<Mutex<bool>>) {

    let mut screen = stdout().into_raw_mode().unwrap();
    // Hide the cursor, clear the screen
    write!(screen, "{}{}", termion::cursor::Hide, termion::clear::All).unwrap();

    // Draw the entire floor
    write!(screen, "{}", cursor_coord(Coord::new(0,0))).unwrap();
    {
        let floor = floor.lock().unwrap();
        for row in &floor.tiles {
            for tile in row {
                if let Some(ref wall) = tile.wall {
                    write!(screen, "{}", wall).unwrap();
                } else {
                    write!(screen, " ").unwrap();
                }
            }
            write!(screen, "\r\n").unwrap();
        }
    }

    // Render Loop
    loop {
        // Don't render too hot.
        sleep(Duration::from_millis(10));

        // Time to stop?
        {
            if *stop.lock().unwrap() {
                break;
            }
        }

        // Once we can lock floor, we can lock anything else we want in this thread.
        let floor = floor.lock().unwrap();

        // Redraw any dirty coordinates
        let mut dirty_coords = dirty_coords.lock().unwrap();
        for coord in dirty_coords.drain(..) {
            write!(screen, "{}{}", cursor_coord(coord), floor.get_symbol(&coord)).unwrap();
        }

        // Render Player
        {
            let mut player = player.lock().unwrap();
            if player.dirty {
                player.dirty = false;
                // Player's sword
                write!(screen, "{}", cursor_coord(player.sword_coord)).unwrap();
                write!(screen, "{}", color::Fg(color::Red)).unwrap();
                write!(screen, "{}", &sword_symbol(&player.facing)).unwrap();
                // Player himself
                write!(screen, "{}", cursor_coord(player.coord)).unwrap();
                write!(screen, "{}", color::Fg(color::Blue)).unwrap();
                write!(screen, "{}", &player.symbol).unwrap();
                write!(screen, "{}", color::Fg(color::Reset)).unwrap();
            }
            // Player Score
            let score_string = format!("Score: {}", player.score);
            write!(screen, "{}", termion::cursor::Goto(
                    (floor.cols + 1 - score_string.len()) as u16, (floor.rows+1) as u16)).unwrap();
            write!(screen, "{}", score_string).unwrap();
        }

        // Render Monsters
        {
            let monsters = monsters.lock().unwrap();
            for monster in monsters.iter() {
                write!(screen, "{}", cursor_coord(monster.coord)).unwrap();
                write!(screen, "{}", color::Fg(color::Green)).unwrap();
                write!(screen, "{}", &monster.symbol).unwrap();
                write!(screen, "{}", color::Fg(color::Reset)).unwrap();
            }
        }

        // Dungeon Name
        write!(screen, "{}", termion::cursor::Goto(1, (floor.rows+1) as u16)).unwrap();
        write!(screen, "{}\n\r\n\r", floor.name).unwrap(); // Dungeon Name
        // Messages
        let mut messages = messages.lock().unwrap();
        if messages.len() > 4 {
            messages.remove(0);
        }
        for msg in messages.iter() {
            write!(screen, "{}{}", color::Fg(color::LightWhite), msg).unwrap();
            write!(screen, "{}{}\n\r", color::Fg(color::Reset), clear::UntilNewline).unwrap();
        }

        screen.flush().unwrap();
    }

    // Nice cleanup: Move cursor below the floor, so we can see how we finished
    {
        let floor = floor.lock().unwrap();
        write!(screen, "{}", cursor_coord(Coord::new(0, (floor.rows+7) as u16))).unwrap();
    }
    print!("{}", termion::cursor::Show);
    screen.flush().unwrap();
}

