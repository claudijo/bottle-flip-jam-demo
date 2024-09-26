mod components;
mod systems;

use crate::collectables::systems::{
    collect_money, despawn_money, spawn_money_trail_1, spawn_money_trail_2, spawn_money_trail_3,
};
use crate::progression::{GameState, ProgressionWaveState};
use bevy::prelude::*;

pub struct CollectablesPlugin;

impl Plugin for CollectablesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(ProgressionWaveState::First),
            spawn_money_trail_1.run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            OnExit(ProgressionWaveState::First),
            despawn_money.run_if(in_state(GameState::InGame)),
        );

        app.add_systems(
            OnEnter(ProgressionWaveState::Second),
            spawn_money_trail_2.run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            OnExit(ProgressionWaveState::Second),
            despawn_money.run_if(in_state(GameState::InGame)),
        );

        app.add_systems(
            OnEnter(ProgressionWaveState::Third),
            spawn_money_trail_3.run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            OnExit(ProgressionWaveState::Third),
            despawn_money.run_if(in_state(GameState::InGame)),
        );

        app.add_systems(Update, collect_money.run_if(in_state(GameState::InGame)));
    }
}
