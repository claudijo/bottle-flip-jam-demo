mod components;
mod systems;

use crate::collectables::systems::{
    collect_money, spawn_money_trail_1, spawn_money_trail_2, spawn_money_trail_3,
};
use bevy::prelude::*;
use crate::level::LevelState;

pub struct CollectablesPlugin;

impl Plugin for CollectablesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(LevelState::FirstWave), spawn_money_trail_1);
        app.add_systems(OnEnter(LevelState::SecondWave), spawn_money_trail_2);
        app.add_systems(OnEnter(LevelState::ThirdWave), spawn_money_trail_3);
        app.add_systems(Update, collect_money);
    }
}
