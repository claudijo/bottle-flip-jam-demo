use crate::aerobat::resources::{RestingActivationTime, RestingThreshold};
use crate::aerobat::systems::{
    add_hit_detector, adjust_gravity, insert_flip_meter_on_release, track_flip_rotation,
    update_grounded, update_resting, update_resting_time,
};
use avian2d::prelude::Gravity;

use crate::bottle::systems::spawn_bottle_content;
use crate::progression::states::{GameState, RoundState};
use bevy::prelude::*;

pub mod components;
mod resources;
mod systems;

#[derive(Debug)]
pub enum LandingType {
    Upright,
    Cap,
    Side,
}

pub const INCREASED_GRAVITY: f32 = 2400.;
pub const NORMAL_GRAVITY: f32 = 1600.;

pub struct AerobatPlugin;

impl Plugin for AerobatPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Gravity(Vec2::NEG_Y * NORMAL_GRAVITY));

        app.add_systems(Update, adjust_gravity.run_if(in_state(GameState::InGame)));

        app.insert_resource(RestingThreshold {
            linear: 30.,
            angular: 0.2,
        });
        app.insert_resource(RestingActivationTime(0.4));
        app.add_systems(
            Update,
            (update_grounded, update_resting_time, update_resting)
                .run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            OnEnter(RoundState::Start),
            add_hit_detector.after(spawn_bottle_content),
        );

        app.add_systems(
            Update,
            (insert_flip_meter_on_release, track_flip_rotation).run_if(in_state(GameState::InGame)),
        );
    }
}

pub fn evaluate_landing_type(rotation: &Quat) -> LandingType {
    let angle = rotation.angle_between(Quat::from_rotation_z(0.));

    if (angle.to_degrees() - 180.).abs() < 2. {
        return LandingType::Cap;
    }

    if (angle.to_degrees()).abs() < 2. {
        return LandingType::Upright;
    }

    LandingType::Side
}

pub fn count_landing_revolutions(landing_type: &LandingType, rotation_angle: f32) -> u32 {
    match landing_type {
        LandingType::Upright => ((rotation_angle.to_degrees() + 180.) / 360.) as u32,
        LandingType::Cap => ((rotation_angle.to_degrees() + 270.) / 360.) as u32,
        LandingType::Side => (rotation_angle.to_degrees() / 360.) as u32,
    }
}
