use bevy::prelude::*;
use rand::seq::IteratorRandom;

pub fn play_sound(sound: &Handle<AudioSource>) -> impl Bundle {
    (AudioPlayer::new(sound.clone()), PlaybackSettings::DESPAWN)
}

pub fn play_random_sound(sounds: &Vec<Handle<AudioSource>>) -> impl Bundle {
    let mut rng = rand::rng();
    let sound = sounds.iter().choose(&mut rng).expect("Asset sound");
    play_sound(sound)
}
