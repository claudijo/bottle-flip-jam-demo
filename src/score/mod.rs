mod components;
mod resources;
mod systems;

use crate::progression::states::GameState;
use crate::score::resources::Score;
use crate::score::systems::{spawn_score_display, spawn_score_icon};
use bevy::prelude::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score(0));
        app.add_systems(OnEnter(GameState::InGame), spawn_score_display);
        app.add_systems(OnEnter(GameState::InGame), spawn_score_icon);
    }
}
