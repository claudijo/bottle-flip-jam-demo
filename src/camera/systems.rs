use crate::camera::components::FocusTarget;
use crate::camera::resources::FollowTarget;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(-280., -340., 0.),
        projection: OrthographicProjection {
            far: 1000.,
            near: -1000.,
            scaling_mode: ScalingMode::AutoMin {
                min_width: 640.,
                min_height: 340.,
            },
            ..default()
        },
        ..default()
    });
}

pub fn aim_camera(
    target_query: Query<&GlobalTransform, With<FocusTarget>>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    follow_target: Res<FollowTarget>,
    time: Res<Time>,
) {
    if !follow_target.0 {
        return;
    }

    for target_transform in &target_query {
        let target_translation = target_transform.translation();
        for mut camera_transform in &mut camera_query {
            camera_transform.translation = camera_transform.translation.lerp(
                target_translation + Vec3::Y * 115.,
                5. * time.delta_seconds(),
            );
        }
    }
}
