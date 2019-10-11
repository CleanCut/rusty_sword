use crossbeam::Receiver;

pub fn sound_loop(sound_rx: Receiver<&str>) {
    let mut audio = rusty_audio::Audio::new();
    audio.add("monster_dies", "monster_dies.wav");
    audio.add("monster_spawns", "monster_spawns.wav");
    audio.add("player_dies", "player_dies.wav");
    while let Ok(clip) = sound_rx.recv() { // recv() blocks
        if clip == "stop" {
            break;
        }
        audio.play(clip); // play() does not block (playback occurs in its own thread)
    }
    audio.wait();
}
