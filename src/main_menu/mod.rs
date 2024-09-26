mod components;
mod systems;

use crate::main_menu::systems::{
    despawn_main_menu, handle_play_button_interactions, spawn_main_menu,
};
use crate::progression::GameState;
use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), spawn_main_menu);
        app.add_systems(OnExit(GameState::MainMenu), despawn_main_menu);
        app.add_systems(
            Update,
            handle_play_button_interactions.run_if(in_state(GameState::MainMenu)),
        );
    }
}
