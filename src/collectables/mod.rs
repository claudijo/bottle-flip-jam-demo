pub mod components;
mod systems;

use crate::collectables::systems::{
    collect_money, despawn_money, spawn_money_trail_1, spawn_money_trail_10, spawn_money_trail_11,
    spawn_money_trail_12, spawn_money_trail_13, spawn_money_trail_14, spawn_money_trail_15,
    spawn_money_trail_16, spawn_money_trail_17, spawn_money_trail_2, spawn_money_trail_3,
    spawn_money_trail_4, spawn_money_trail_5, spawn_money_trail_6, spawn_money_trail_7,
    spawn_money_trail_8, spawn_money_trail_9,
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

        app.add_systems(
            OnEnter(LevelState::Fourth),
            spawn_money_trail_4.run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            OnExit(LevelState::Fourth),
            despawn_money.run_if(in_state(GameState::InGame)),
        );

        app.add_systems(
            OnEnter(LevelState::Fifth),
            spawn_money_trail_5.run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            OnExit(LevelState::Fifth),
            despawn_money.run_if(in_state(GameState::InGame)),
        );

        app.add_systems(
            OnEnter(LevelState::Sixth),
            spawn_money_trail_6.run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            OnExit(LevelState::Sixth),
            despawn_money.run_if(in_state(GameState::InGame)),
        );

        app.add_systems(
            OnEnter(LevelState::Seventh),
            spawn_money_trail_7.run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            OnExit(LevelState::Seventh),
            despawn_money.run_if(in_state(GameState::InGame)),
        );

        app.add_systems(
            OnEnter(LevelState::Eighth),
            spawn_money_trail_8.run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            OnExit(LevelState::Eighth),
            despawn_money.run_if(in_state(GameState::InGame)),
        );

        app.add_systems(
            OnEnter(LevelState::Ninth),
            spawn_money_trail_9.run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            OnExit(LevelState::Ninth),
            despawn_money.run_if(in_state(GameState::InGame)),
        );

        app.add_systems(
            OnEnter(LevelState::Tenth),
            spawn_money_trail_10.run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            OnExit(LevelState::Tenth),
            despawn_money.run_if(in_state(GameState::InGame)),
        );

        app.add_systems(
            OnEnter(LevelState::Eleventh),
            spawn_money_trail_11.run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            OnExit(LevelState::Eleventh),
            despawn_money.run_if(in_state(GameState::InGame)),
        );

        app.add_systems(
            OnEnter(LevelState::Twelfth),
            spawn_money_trail_12.run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            OnExit(LevelState::Twelfth),
            despawn_money.run_if(in_state(GameState::InGame)),
        );

        app.add_systems(
            OnEnter(LevelState::Thirteenth),
            spawn_money_trail_13.run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            OnExit(LevelState::Thirteenth),
            despawn_money.run_if(in_state(GameState::InGame)),
        );

        app.add_systems(
            OnEnter(LevelState::Fourteenth),
            spawn_money_trail_14.run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            OnExit(LevelState::Fourteenth),
            despawn_money.run_if(in_state(GameState::InGame)),
        );

        app.add_systems(
            OnEnter(LevelState::Fifteenth),
            spawn_money_trail_15.run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            OnExit(LevelState::Fifteenth),
            despawn_money.run_if(in_state(GameState::InGame)),
        );

        app.add_systems(
            OnEnter(LevelState::Sixteenth),
            spawn_money_trail_16.run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            OnExit(LevelState::Sixteenth),
            despawn_money.run_if(in_state(GameState::InGame)),
        );

        app.add_systems(
            OnEnter(LevelState::Seventeenth),
            spawn_money_trail_17.run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            OnExit(LevelState::Seventeenth),
            despawn_money.run_if(in_state(GameState::InGame)),
        );

        app.add_systems(OnEnter(GameState::MainMenu), despawn_money);
        app.add_systems(OnEnter(GameState::Restarting), despawn_money);

        app.add_systems(Update, collect_money.run_if(in_state(GameState::InGame)));
    }
}
