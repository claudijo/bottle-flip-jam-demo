use crate::platforms::systems::{
    animate_fan, despawn_game_platforms, remove_round_target_platform_marker, spawn_bench,
    spawn_cardboard_boxes, spawn_fan, spawn_ground,
};

use crate::progression::states::{GameState, LevelState, RoundState};
use bevy::prelude::*;

mod components;
mod systems;

pub struct PlatformsPlugin;

impl Plugin for PlatformsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(LevelState::First),
            spawn_bench.run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            OnEnter(LevelState::Second),
            spawn_cardboard_boxes.run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            OnEnter(LevelState::Third),
            spawn_fan.run_if(in_state(GameState::InGame)),
        );

        app.add_systems(
            OnEnter(RoundState::Finished),
            remove_round_target_platform_marker.run_if(in_state(GameState::InGame)),
        );

        app.add_systems(Startup, spawn_ground);
        app.add_systems(Update, animate_fan.run_if(in_state(GameState::InGame)));

        app.add_systems(OnEnter(GameState::MainMenu), despawn_game_platforms);
        app.add_systems(OnEnter(GameState::Restarting), despawn_game_platforms);
    }
}
