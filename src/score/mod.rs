mod components;
pub mod events;
mod resources;
mod systems;

use crate::progression::states::GameState;
use crate::score::events::ScoreEvent;
use crate::score::resources::Score;
use crate::score::systems::{
    collect_score, spawn_score_display, spawn_score_icon, update_score_display,
};
use bevy::prelude::*;
use std::collections::HashMap;

pub const COLLECT_MONEY_SCORE: u32 = 50;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ScoreEvent>();
        app.insert_resource(Score(HashMap::<u64, u32>::new()));
        app.add_systems(OnEnter(GameState::InGame), spawn_score_display);
        app.add_systems(OnEnter(GameState::InGame), spawn_score_icon);
        app.add_systems(
            Update,
            (collect_score, update_score_display).run_if(in_state(GameState::InGame)),
        );
    }
}
