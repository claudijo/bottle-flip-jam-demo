use crate::game_options_menu::systems::{
    despawn_game_options_button, despawn_game_options_menu,
    handle_cancel_or_resume_button_interactions, handle_game_options_button_interactions,
    handle_quit_button_interaction, handle_restart_button_interaction, spawn_game_options_button,
    spawn_game_options_menu,
};
use crate::progression::states::GameState;
use bevy::prelude::*;

mod components;
mod systems;

pub struct InGameMenuPlugin;

impl Plugin for InGameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_game_options_button);
        app.add_systems(OnExit(GameState::InGame), despawn_game_options_button);
        app.add_systems(
            Update,
            handle_game_options_button_interactions.run_if(in_state(GameState::InGame)),
        );
        app.add_systems(OnEnter(GameState::GameOptionsMenu), spawn_game_options_menu);
        app.add_systems(
            OnExit(GameState::GameOptionsMenu),
            despawn_game_options_menu,
        );

        app.add_systems(
            Update,
            (
                handle_cancel_or_resume_button_interactions,
                handle_restart_button_interaction,
                handle_quit_button_interaction,
            )
                .run_if(in_state(GameState::GameOptionsMenu)),
        );
    }
}
