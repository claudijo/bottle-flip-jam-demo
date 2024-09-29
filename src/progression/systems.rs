use crate::aerobat::components::{Aerobat, Grounded, Resting};
use crate::progression::components::WaypointPlatform;
use crate::progression::ProgressionWaveState;
use bevy::prelude::*;

pub fn check_waypoint_completed(
    mut commands: Commands,
    waypoint_platform_query: Query<Entity, With<WaypointPlatform>>,
    state: Res<State<ProgressionWaveState>>,
    mut next_state: ResMut<NextState<ProgressionWaveState>>,
    aerobat_query: Query<&Grounded, (With<Aerobat>, Added<Resting>)>,
) {
    for grounded in &aerobat_query {
        for waypoint in &waypoint_platform_query {
            if grounded.0 == Some(waypoint) {
                commands.entity(waypoint).remove::<WaypointPlatform>();
                match state.get() {
                    ProgressionWaveState::Zero => {}
                    ProgressionWaveState::First => next_state.set(ProgressionWaveState::Second),
                    ProgressionWaveState::Second => next_state.set(ProgressionWaveState::Third),
                    ProgressionWaveState::Third => {}
                }
            }
        }
    }
}
