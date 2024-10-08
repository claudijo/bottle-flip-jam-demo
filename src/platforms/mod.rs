use crate::platforms::systems::{
    animate_fan, despawn_game_platforms, remove_round_target_platform_marker, spawn_awning_1,
    spawn_awning_2, spawn_bench, spawn_cardboard_boxes, spawn_fan, spawn_ground_collider,
    spawn_horizontal_neon_sign, spawn_ledge_1, spawn_skateboard, spawn_thin_ledge,
    spawn_top_awning, spawn_top_ledge, spawn_vent_1, spawn_vent_2, spawn_vent_3,
    spawn_vertical_neon_sign, spawn_wall_tiles_1, spawn_wall_tiles_2, spawn_wall_tiles_3,
};

use crate::progression::states::{GameState, LevelState, RoundState};
use bevy::prelude::*;

mod components;
mod systems;

pub struct PlatformTemplate {
    translation: Vec3,
    collider_size: Vec2,
    name: Option<String>,
    asset_path: String,
    sprite_offset: Vec3,
}

impl PlatformTemplate {
    pub fn new(translation: Vec3, collider_size: Vec2, asset_path: String) -> Self {
        Self {
            translation,
            collider_size,
            name: None,
            asset_path,
            sprite_offset: Vec3::ZERO,
        }
    }

    pub fn with_sprite_offset(&mut self, offset: Vec3) -> &mut Self {
        self.sprite_offset = offset;
        self
    }

    pub fn with_name(&mut self, name: String) -> &mut Self {
        self.name = Some(name);
        self
    }
}

pub struct PlatformsPlugin;

impl Plugin for PlatformsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ground_collider);

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
            OnEnter(LevelState::Fourth),
            spawn_ledge_1.run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            OnEnter(LevelState::Fifth),
            spawn_vent_1.run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            OnEnter(LevelState::Sixth),
            spawn_vent_2.run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            OnEnter(LevelState::Seventh),
            spawn_vent_3.run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            OnEnter(LevelState::Eighth),
            spawn_awning_1.run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            OnEnter(LevelState::Ninth),
            spawn_awning_2.run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            OnEnter(LevelState::Tenth),
            spawn_thin_ledge.run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            OnEnter(LevelState::Eleventh),
            spawn_vertical_neon_sign.run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            OnEnter(LevelState::Twelfth),
            spawn_wall_tiles_1.run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            OnEnter(LevelState::Thirteenth),
            spawn_wall_tiles_2.run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            OnEnter(LevelState::Fourteenth),
            spawn_wall_tiles_3.run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            OnEnter(LevelState::Fifteenth),
            spawn_horizontal_neon_sign.run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            OnEnter(LevelState::Sixteenth),
            spawn_top_awning.run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            OnEnter(LevelState::Seventeenth),
            (
                spawn_top_ledge.run_if(in_state(GameState::InGame)),
                spawn_skateboard.run_if(in_state(GameState::InGame)),
            ),
        );

        app.add_systems(
            OnEnter(RoundState::Finished),
            remove_round_target_platform_marker.run_if(in_state(GameState::InGame)),
        );

        app.add_systems(Update, animate_fan.run_if(in_state(GameState::InGame)));

        app.add_systems(OnEnter(GameState::MainMenu), despawn_game_platforms);
        app.add_systems(OnEnter(GameState::Restarting), despawn_game_platforms);
    }
}
