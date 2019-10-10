use std::sync::{mpsc, Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

pub fn sound_loop(sound_rx: mpsc::Receiver<&str>) {
    let mut audio = rusty_audio::Audio::new();
    audio.add("monster_dies", "monster_dies.wav");
    audio.add("monster_spawns", "monster_spawns.wav");
    audio.add("player_dies", "player_dies.wav");
    while let Ok(clip) = sound_rx.recv() {
        if clip == "stop" {
            break;
        }
        audio.play(clip);
    }
    audio.wait();
}
