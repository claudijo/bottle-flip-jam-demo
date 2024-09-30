use crate::aerobat::resources::{RestingActivationTime, RestingThreshold};
use crate::aerobat::systems::{
    insert_flip_meter_on_release, update_grounded, update_resting, update_resting_time,
};

use crate::progression::states::GameState;
use bevy::prelude::*;

pub mod components;
mod resources;
mod systems;

pub struct AerobatPlugin;

impl Plugin for AerobatPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RestingThreshold {
            linear: 30.,
            angular: 0.15,
        });
        app.insert_resource(RestingActivationTime(0.4));
        app.add_systems(
            Update,
            (update_grounded, update_resting_time, update_resting)
                .run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            Update,
            insert_flip_meter_on_release.run_if(in_state(GameState::InGame)),
        );
    }
}
