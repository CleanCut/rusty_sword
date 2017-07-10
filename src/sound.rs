use ::*;

pub fn sound_loop(sound_rx : mpsc::Receiver<&str>, stop : Arc<Mutex<bool>>) {
    let mut monster_dies = Sound::new("monster_dies.wav").unwrap();
    let mut monster_spawns = Sound::new("monster_spawns.wav").unwrap();
    let mut player_dies = Sound::new("player_dies.wav").unwrap();
    loop {
        sleep(Duration::from_millis(10));
        {
            if *stop.lock().unwrap() {
                break;
            }
        }
        while let Ok(choice) = sound_rx.try_recv() {
            match choice {
                "monster_dies" => monster_dies.play(),
                "monster_spawns" => monster_spawns.play(),
                "player_dies" => player_dies.play(),
                _ => {},
            }
        }
    }
    while player_dies.is_playing() {}
}
