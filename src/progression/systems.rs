use crate::bottle::components::Bottle;
use crate::progression::components::WaypointPlatform;
use crate::progression::ProgressionWaveState;
use avian2d::position::Rotation;
use avian2d::prelude::ShapeHits;
use bevy::prelude::*;

pub fn check_waypoint_completed(
    mut commands: Commands,
    waypoint_platform_query: Query<Entity, With<WaypointPlatform>>,
    mut bottle_query: Query<(&ShapeHits, &Rotation), With<Bottle>>,
    state: Res<State<ProgressionWaveState>>,
    mut next_state: ResMut<NextState<ProgressionWaveState>>,
) {
    for (hits, rotation) in &mut bottle_query {
        for waypoint_platform in &waypoint_platform_query {
            let is_grounded_on_waypoint_platform = hits.iter().any(|hit| {
                hit.entity == waypoint_platform
                    && (rotation * -hit.normal2).angle_between(Vec2::Y).abs() <= 0.01
            });

            if is_grounded_on_waypoint_platform {
                commands
                    .entity(waypoint_platform)
                    .remove::<WaypointPlatform>();
                match state.get() {
                    ProgressionWaveState::First => next_state.set(ProgressionWaveState::Second),
                    ProgressionWaveState::Second => next_state.set(ProgressionWaveState::Third),
                    ProgressionWaveState::Third => {}
                }
            }
        }
    }
}
