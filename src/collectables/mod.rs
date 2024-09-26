mod components;
mod systems;

use crate::collectables::systems::{collect_money, despawn_money, spawn_money_trail_1, spawn_money_trail_2, spawn_money_trail_3};
use bevy::prelude::*;
use crate::progression::ProgressionWaveState;

pub struct CollectablesPlugin;

impl Plugin for CollectablesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ProgressionWaveState::FirstWave), spawn_money_trail_1);
        app.add_systems(OnExit(ProgressionWaveState::FirstWave), despawn_money);

        app.add_systems(OnEnter(ProgressionWaveState::SecondWave), spawn_money_trail_2);
        app.add_systems(OnExit(ProgressionWaveState::SecondWave), despawn_money);

        app.add_systems(OnEnter(ProgressionWaveState::ThirdWave), spawn_money_trail_3);
        app.add_systems(OnExit(ProgressionWaveState::ThirdWave), despawn_money);

        app.add_systems(Update, collect_money);
    }
}
