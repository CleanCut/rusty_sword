use ::*;

fn out<S: ToString>(screen : &mut RawTerminal<Stdout>, output : S) {
    write!(*screen, "{}", output.to_string()).unwrap();
}

fn curs(screen : &mut RawTerminal<Stdout>, coord : Coord) {
    out(screen, termion::cursor::Goto(coord.col+1, coord.row+1));
}

fn color<C: Color>(screen : &mut RawTerminal<Stdout>, clr : C) {
    out(screen, Fg(clr));
}

pub fn render_loop(
    stop         : Arc<Mutex<bool>>,
    floor        : Arc<Mutex<Floor>>,
    player       : Arc<Mutex<Player>>,
    dirty_coords : Arc<Mutex<Vec<Coord>>>,
    messages     : Arc<Mutex<Vec<String>>>,
    monsters     : Arc<Mutex<Vec<Monster>>>,
) {
    let mut screen = &mut stdout().into_raw_mode().unwrap();
    out(screen, termion::cursor::Hide); // Hide the cursor
    out(screen, termion::clear::All);   // Clear the screen

    // Draw the entire floor
    curs(screen, Coord::new(0,0));
    {
        let floor = floor.lock().unwrap();
        for row in &floor.tiles {
            for tile in row {
                if let Some(ref wall) = tile.wall {
                    out(screen, wall);
                } else {
                    out(screen, " ");
                }
            }
            out(screen, "\r\n");
        }
    }

    // Render Loop
    loop {
        sleep(Duration::from_millis(10));
        {
            if *stop.lock().unwrap() {
                break;
            }
        }

        // Lock floor first...
        let floor = floor.lock().unwrap();

        // Redraw any dirty coordinates with floor tiles
        let mut dirty_coords = dirty_coords.lock().unwrap();
        for coord in dirty_coords.drain(..) {
            curs(screen, coord);
            out(screen, floor.get_symbol(&coord));
        }

        // Render Player
        let mut player = player.lock().unwrap();
        if player.dirty {
            player.dirty = false;
            // Player's sword
            curs(screen, player.sword_coord);
            color(screen, Red);
            out(screen, sword_symbol(player.facing));
            // Player himself
            curs(screen, player.coord);
            color(screen, Blue);
            out(screen, &player.symbol);
            color(screen, Reset);
        }
        // Player Score
        let score_string = format!("Score: {}", player.score);
        curs(screen, Coord::new((floor.cols - score_string.len()) as u16,
                                floor.rows as u16));
        out(screen, score_string);

        // Render Monsters
        {
            let monsters = monsters.lock().unwrap();
            for monster in monsters.iter() {
                curs(screen, monster.coord);
                color(screen, Green);
                out(screen, &monster.symbol);
                color(screen, Reset);
            }
        }

        // Game Title
        curs(screen, Coord::new(0, floor.rows as u16));
        out(screen, "Rusty Sword - Game of Infamy!");

        // Messages
        curs(screen, Coord::new(0, (floor.rows + 2) as u16));
        let mut messages = messages.lock().unwrap();
        if messages.len() > 4 {
            messages.remove(0);
        }
        for msg in messages.iter() {
            if msg.contains("spawned") {
                color(screen, Green);
            } else if msg.contains("You killed") {
                color(screen, Blue);
            } else if msg.contains("were eaten") {
                color(screen, Red);
            } else {
                color(screen, LightWhite);
            }
            out(screen, msg);
            out(screen, format!("{}\n\r", clear::UntilNewline));
            color(screen, Reset);
        }

        screen.flush().unwrap();
    }

    // Nice cleanup: Move cursor below the floor, so we can see how we finished
    {
        let floor = floor.lock().unwrap();
        curs(screen, Coord::new(0, (floor.rows+7) as u16));
    }
    out(screen, termion::cursor::Show); // Show the cursor again
    screen.flush().unwrap();
}

