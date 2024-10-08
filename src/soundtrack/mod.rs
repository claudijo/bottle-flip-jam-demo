mod components;
mod resources;
mod systems;

use crate::soundtrack::systems::{change_tracks, fade_in, fade_out, setup_tracks};
use bevy::prelude::*;

pub struct SoundTrackPlugin;

// Fade effect duration
pub const FADE_TIME: f32 = 2.0;

impl Plugin for SoundTrackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_tracks);
        app.add_systems(Update, (change_tracks, fade_in, fade_out));
    }
}
