mod components;
mod systems;

use crate::collectables::systems::{collect_money, spawn_money_trail_1};
use bevy::prelude::*;

pub struct CollectablesPlugin;

impl Plugin for CollectablesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_money_trail_1);
        app.add_systems(Update, collect_money);
    }
}
