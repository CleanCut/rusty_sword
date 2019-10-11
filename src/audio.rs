use crossbeam::Receiver;

pub fn audio_loop(clip_rx: Receiver<&str>) {
    let mut audio = rusty_audio::Audio::new();
    audio.add("monster_dies", "clips/monster_dies.wav");
    audio.add("monster_spawns", "clips/monster_spawns.wav");
    audio.add("player_dies", "clips/player_dies.wav");
    while let Ok(clip) = clip_rx.recv() { // recv() blocks
        if clip == "stop" {
            break;
        }
        audio.play(clip); // play() does not block (playback occurs in its own thread)
    }
    audio.wait();
}
