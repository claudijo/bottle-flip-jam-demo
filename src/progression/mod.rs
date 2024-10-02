use crate::progression::resources::RoundId;
use crate::progression::states::{GameState, LevelState, RoundState};
use crate::progression::systems::{
    end_round, reset_level_state, reset_round_id, reset_round_state, restart_game, restart_round,
    start_first_level, start_first_round, start_next_round,
};
use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod states;
mod systems;

pub struct ProgressionPlugin;

impl Plugin for ProgressionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(RoundState::default());
        app.insert_state(LevelState::default());
        app.insert_state(GameState::default());
        app.insert_resource(RoundId(0));
        app.add_systems(OnEnter(GameState::InGame), start_first_round);
        app.add_systems(OnEnter(GameState::InGame), start_first_level);
        app.add_systems(OnEnter(RoundState::Finished), start_next_round);
        app.add_systems(OnEnter(RoundState::Unfinished), restart_round);

        app.add_systems(
            OnEnter(GameState::MainMenu),
            (reset_round_state, reset_level_state, reset_round_id),
        );

        app.add_systems(
            OnEnter(GameState::Restarting),
            (
                reset_round_state,
                reset_level_state,
                reset_round_id,
                restart_game,
            ),
        );

        app.add_systems(Update, end_round.run_if(in_state(GameState::InGame)));
    }
}
