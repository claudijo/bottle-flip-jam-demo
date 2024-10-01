use crate::aerobat::components::{Aerobat, FlipMeter, Grounded, Resting};
use crate::progression::components::WaypointPlatform;
use crate::progression::resources::RoundId;
use crate::progression::states::{LevelState, RoundState};
use bevy::prelude::*;

pub fn end_round(
    waypoint_platform_query: Query<Entity, With<WaypointPlatform>>,
    mut next_round_state: ResMut<NextState<RoundState>>,
    aerobat_query: Query<&Grounded, (With<Aerobat>, With<FlipMeter>, Added<Resting>)>,
) {
    for grounded in &aerobat_query {
        let on_waypoint_platform = waypoint_platform_query
            .iter()
            .any(|waypoint| grounded.0 == Some(waypoint));
        if on_waypoint_platform {
            next_round_state.set(RoundState::Finished);
        } else {
            next_round_state.set(RoundState::Unfinished);
        }
    }
}

pub fn start_first_round(
    mut next_round_state: ResMut<NextState<RoundState>>,
    mut round_id: ResMut<RoundId>,
) {
    round_id.0 = 0;
    next_round_state.set(RoundState::Start);
}

pub fn start_next_round(
    level_state: Res<State<LevelState>>,
    mut next_level_state: ResMut<NextState<LevelState>>,
    mut next_round_state: ResMut<NextState<RoundState>>,
    mut round_id: ResMut<RoundId>,
) {
    round_id.0 += 1;
    next_level_state.set(level_state.next());
    next_round_state.set(RoundState::Start);
}

pub fn restart_round(
    mut next_round_state: ResMut<NextState<RoundState>>,
    mut round_id: ResMut<RoundId>,
) {
    round_id.0 += 1;
    next_round_state.set(RoundState::Start);
}
