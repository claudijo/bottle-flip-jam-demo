use bevy::prelude::*;

//  This resource will hold the track list for your soundtrack
#[derive(Resource)]
pub struct SoundtrackPlayer {
    pub track_list: Vec<Handle<AudioSource>>,
}

impl SoundtrackPlayer {
    pub fn new(track_list: Vec<Handle<AudioSource>>) -> Self {
        Self { track_list }
    }
}