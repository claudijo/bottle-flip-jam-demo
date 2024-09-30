mod components;
mod systems;

use crate::collectables::systems::{
    collect_money, despawn_money, spawn_money_trail_1, spawn_money_trail_2, spawn_money_trail_3,
};
use crate::progression::states::{GameState, LevelState};
use bevy::prelude::*;

pub struct CollectablesPlugin;

impl Plugin for CollectablesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(LevelState::First),
            spawn_money_trail_1.run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            OnExit(LevelState::First),
            despawn_money.run_if(in_state(GameState::InGame)),
        );

        app.add_systems(
            OnEnter(LevelState::Second),
            spawn_money_trail_2.run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            OnExit(LevelState::Second),
            despawn_money.run_if(in_state(GameState::InGame)),
        );

        app.add_systems(
            OnEnter(LevelState::Third),
            spawn_money_trail_3.run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            OnExit(LevelState::Third),
            despawn_money.run_if(in_state(GameState::InGame)),
        );

        app.add_systems(Update, collect_money.run_if(in_state(GameState::InGame)));
    }
}
