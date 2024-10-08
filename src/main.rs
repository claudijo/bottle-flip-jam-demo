// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod aerobat;
mod bottle;
mod camera;
mod collectables;
pub mod config;
pub mod free_hand_controller;
mod game_options_menu;
mod level_editor;
mod levels;
mod main_menu;
mod physics;
mod platforms;
mod progression;
mod score;
mod soundtrack;

use crate::aerobat::AerobatPlugin;
use crate::bottle::BottlePlugin;
use crate::camera::CameraPlugin;
use crate::collectables::CollectablesPlugin;
use crate::free_hand_controller::FreeHandControllerPlugin;
use crate::game_options_menu::InGameMenuPlugin;
use crate::levels::LevelsPlugin;
use crate::main_menu::MainMenuPlugin;
use crate::platforms::PlatformsPlugin;
use crate::progression::ProgressionPlugin;
use crate::score::ScorePlugin;
use avian2d::prelude::*;
use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::window::WindowResolution;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics in web builds on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bottle Flip Jam ᗪ乇爪ㄖ".into(),
                        resolution: WindowResolution::new(640., 360.),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(PhysicsPlugins::default().with_length_unit(100.0))
        .insert_resource(Gravity(Vec2::NEG_Y * 1500.0))
        // .add_plugins(PhysicsDebugPlugin::default())
        //.add_plugins(LevelEditorPlugin)
        .add_plugins((
            CameraPlugin,
            MainMenuPlugin,
            InGameMenuPlugin,
            LevelsPlugin,
            PlatformsPlugin,
            BottlePlugin,
            FreeHandControllerPlugin,
            CollectablesPlugin,
            AerobatPlugin,
            ProgressionPlugin,
            ScorePlugin,
            // SoundTrackPlugin,
        ))
        .insert_resource(ClearColor(Color::srgb(56. / 255., 0. / 255., 44. / 255.)))
        .run();
}
