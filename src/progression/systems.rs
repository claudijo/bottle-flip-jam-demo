use avian2d::position::Rotation;
use avian2d::prelude::ShapeHits;
use bevy::prelude::*;
use crate::bottle::components::Bottle;
use crate::progression::components::WaypointPlatform;
use crate::progression::ProgressionWaveState;

pub fn check_waypoint_completed(
    mut commands: Commands,
    waypoint_platform_query: Query<Entity, With<WaypointPlatform>>,
    mut bottle_query: Query<(&ShapeHits, &Rotation), With<Bottle>>,
    state: Res<State<ProgressionWaveState>>,
    mut next_state: ResMut<NextState<ProgressionWaveState>>,
    time: Res<Time>,
) {
    for (hits, rotation) in &mut bottle_query {
        for waypoint_platform in &waypoint_platform_query {
            let is_grounded_on_waypoint_platform = hits
                .iter()
                .any(|hit| {
                    hit.entity == waypoint_platform && (rotation * -hit.normal2).angle_between(Vec2::Y).abs() <= 0.0001
                });

            if is_grounded_on_waypoint_platform {
                println!("Is grounded {:?}", time.elapsed().as_millis());
                // commands.entity(waypoint_platform).remove::<WaypointPlatform>();
                // match state.get() {
                //     ProgressionWaveState::FirstWave => next_state.set(ProgressionWaveState::SecondWave),
                //     ProgressionWaveState::SecondWave => next_state.set(ProgressionWaveState::ThirdWave),
                //     ProgressionWaveState::ThirdWave => {}
                // }
            } else {
                // println!("Not grounded")
            }
        }
    }
}