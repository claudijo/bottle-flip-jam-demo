mod components;
pub mod events;
mod resources;
mod systems;

use crate::progression::states::GameState;
use crate::score::events::{BonusEvent, ScoreEvent};
use crate::score::resources::Score;
use crate::score::systems::{
    collect_bonus, collect_score, despawn_score_display, despawn_score_icon, reset_score,
    spawn_score_display, spawn_score_icon, update_score_display,
};
use bevy::prelude::*;
use std::collections::HashMap;

pub const COLLECT_MONEY_SCORE: u32 = 50;
pub const FLIP_LAND_BONUS: u32 = 4;
pub const CAP_FLIP_LAND_BONUS: u32 = 10;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ScoreEvent>();
        app.add_event::<BonusEvent>();
        app.insert_resource(Score(HashMap::<u64, u32>::new()));
        app.add_systems(
            OnEnter(GameState::InGame),
            (spawn_score_display, spawn_score_icon),
        );
        app.add_systems(
            OnEnter(GameState::MainMenu),
            (despawn_score_display, despawn_score_icon),
        );

        app.add_systems(OnEnter(GameState::Restarting), reset_score);

        app.add_systems(
            Update,
            (collect_score, collect_bonus, update_score_display)
                .run_if(in_state(GameState::InGame)),
        );
    }
}
