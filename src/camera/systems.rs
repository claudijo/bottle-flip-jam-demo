use bevy::prelude::*;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(-280., -340., 0.,),
        ..default()
    });
}
