use crate::bottle::components::BottlePart;
use crate::collectables::components::Money;
use crate::config::ASSETS_SCALE_FACTOR;
use crate::grabber::resources::Grabbing;
use avian2d::prelude::{Collider, Collision, RigidBody, Sensor};
use bevy::prelude::*;

pub fn spawn_money_trail_1(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle = asset_server.load("money_sprite.png");
    let texture_atlas = TextureAtlasLayout::from_grid(UVec2::splat(24), 6, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    for (translation, atlas_index) in [
        (
            Vec3::new(-80. * ASSETS_SCALE_FACTOR, 5. * ASSETS_SCALE_FACTOR, 3.),
            1,
        ),
        (
            Vec3::new(-75. * ASSETS_SCALE_FACTOR, 22.5 * ASSETS_SCALE_FACTOR, 3.),
            0,
        ),
        (
            Vec3::new(-60. * ASSETS_SCALE_FACTOR, 35. * ASSETS_SCALE_FACTOR, 3.),
            5,
        ),
        (
            Vec3::new(-45. * ASSETS_SCALE_FACTOR, 17.5 * ASSETS_SCALE_FACTOR, 3.),
            3,
        ),
        (
            Vec3::new(-40. * ASSETS_SCALE_FACTOR, -5. * ASSETS_SCALE_FACTOR, 3.),
            2,
        ),
    ] {
        commands
            .spawn((
                VisibilityBundle::default(),
                TransformBundle::from_transform(Transform::from_translation(translation)),
                RigidBody::Static,
                Collider::circle(16.),
                Sensor,
                Money,
            ))
            .with_children(|child_builder| {
                child_builder.spawn((
                    SpriteBundle {
                        transform: Transform::from_xyz(0., 0., 0.).with_scale(Vec3::ONE * 2.),
                        texture: texture_handle.clone(),
                        ..default()
                    },
                    TextureAtlas {
                        layout: texture_atlas_handle.clone(),
                        index: atlas_index,
                    },
                ));
            });
    }
}

pub fn collect_money(
    mut commands: Commands,
    mut collision_event_reader: EventReader<Collision>,
    bottle_part_query: Query<Entity, With<BottlePart>>,
    money_query: Query<Entity, With<Money>>,
    grabbing: Res<Grabbing>,
) {
    if grabbing.0 {
        return;
    }

    let mut despawned = vec![];

    for Collision(contacts) in collision_event_reader.read() {
        for bottle_part in &bottle_part_query {
            for money in &money_query {
                // Avoiding warnings about could not despawn entity because it doesn't exist in this World.
                if despawned.contains(&contacts.entity1) || despawned.contains(&contacts.entity2) {
                    continue;
                }

                if contacts.entity1 == bottle_part && contacts.entity2 == money {
                    despawned.push(contacts.entity2);
                    commands.entity(contacts.entity2).despawn_recursive();
                } else if contacts.entity1 == money && contacts.entity2 == bottle_part {
                    despawned.push(contacts.entity1);
                    commands.entity(contacts.entity1).despawn_recursive();
                }
            }
        }
    }
}
