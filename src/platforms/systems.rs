use crate::physics::CustomCollisionLayer;
use crate::platforms::components::{FanAnimationTimer, GamePlatform};
use crate::platforms::PlatformTemplate;
use crate::progression::components::{TargetPlatform, SpawnPlatform, PermanentPlatform};
use avian2d::prelude::*;
use bevy::prelude::*;
use std::time::Duration;

pub fn clear_target_platform(
    mut commands: Commands,
    prev_target_platform_query: Query<Entity, With<TargetPlatform>>,
    prev_spawn_platform_query: Query<Entity, With<SpawnPlatform>>,

) {
    for entity in &prev_spawn_platform_query {
        commands.entity(entity).remove::<SpawnPlatform>();
    }

    for entity in &prev_target_platform_query {
        commands.entity(entity).remove::<TargetPlatform>();
        commands.entity(entity).insert(SpawnPlatform);
    }
}

pub fn disable_colliders_for_cleared_platforms(
    mut platform_query: Query<&mut CollisionLayers, (With<GamePlatform>, Without<TargetPlatform>, Without<SpawnPlatform>, Without<PermanentPlatform>)>,
) {
    for mut collision_layers in &mut platform_query {
        collision_layers.memberships = LayerMask::NONE;
        collision_layers.filters = LayerMask::NONE;
    }

}

pub fn spawn_ground_collider(mut commands: Commands) {
    commands.spawn((
        TransformBundle::from_transform(Transform::from_xyz(0., -768., 2.)),
        RigidBody::Static,
        Collider::half_space(Vec2::Y),
        CollisionLayers::new(
            CustomCollisionLayer::Platform,
            [CustomCollisionLayer::Bottle, CustomCollisionLayer::Platform],
        ),
        PermanentPlatform,
        GamePlatform,
        Name::new("Ground collider"),
    ));
}

pub fn spawn_fan(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle = asset_server.load("images/fan_sprite.png");
    let texture_atlas = TextureAtlasLayout::from_grid(UVec2::splat(32), 4, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn((
            VisibilityBundle::default(),
            TransformBundle::from_transform(Transform::from_xyz(336., -624., 2.)),
            RigidBody::Static,
            Collider::rectangle(32. * 3., 32. * 3.),
            CollisionLayers::new(
                CustomCollisionLayer::Platform,
                [CustomCollisionLayer::Bottle, CustomCollisionLayer::Platform],
            ),
            GamePlatform,
            TargetPlatform,
        ))
        .with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(0., 0., 0.).with_scale(Vec3::ONE * 3.),
                    texture: texture_handle,
                    ..default()
                },
                TextureAtlas::from(texture_atlas_handle),
                FanAnimationTimer(Timer::new(Duration::from_millis(80), TimerMode::Repeating)),
            ));
        });
}

pub fn animate_fan(
    mut fan_atlas_query: Query<(&mut TextureAtlas, &mut FanAnimationTimer)>,
    time: Res<Time>,
) {
    for (mut fan_atlas, mut fan_animation_timer) in &mut fan_atlas_query {
        fan_animation_timer.0.tick(time.delta());
        if fan_animation_timer.0.just_finished() {
            fan_atlas.index = (fan_atlas.index + 1) % 4;
        }
    }
}

fn spawn_platform(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    platform: &PlatformTemplate,
) -> Entity {
    let entity = commands
        .spawn((
            VisibilityBundle::default(),
            TransformBundle::from_transform(Transform::from_translation(platform.translation)),
            RigidBody::Static,
            Collider::rectangle(platform.collider_size.x, platform.collider_size.y),
            CollisionLayers::new(
                CustomCollisionLayer::Platform,
                [CustomCollisionLayer::Bottle, CustomCollisionLayer::Platform],
            ),
            GamePlatform,
            TargetPlatform,
        ))
        .with_children(|parent| {
            parent.spawn(SpriteBundle {
                transform: Transform::from_translation(platform.sprite_offset)
                    .with_scale(Vec3::ONE * 3.),
                texture: asset_server.load(platform.asset_path.clone()),
                ..default()
            });
        })
        .id();

    if let Some(name) = platform.name.clone() {
        commands.entity(entity).insert(Name::from(name));
    }

    entity
}

pub fn spawn_bench(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_platform(
        &mut commands,
        &asset_server,
        PlatformTemplate::new(
            Vec3::new(-66., -754.5, 2.),
            Vec2::new(93., 33.),
            "images/bench.png".to_string(),
        )
        .with_sprite_offset(Vec3::new(1.5, 13.5, 0.))
        .with_name("Bench".to_string()),
    );
}

pub fn spawn_cardboard_boxes(mut commands: Commands, asset_server: Res<AssetServer>) {
    for (i, translation) in [
        Vec3::new(108., -738., 2.),
        Vec3::new(189., -738., 2.),
        Vec3::new(174., -675., 2.),
    ]
    .into_iter()
    .enumerate()
    {
        spawn_platform(
            &mut commands,
            &asset_server,
            PlatformTemplate::new(
                translation,
                Vec2::new(84., 66.),
                "images/cardboard_box.png".to_string(),
            )
            .with_name(format!("Cardboard box {}", i)),
        );
    }
}

pub fn spawn_ledge_1(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_platform(
        &mut commands,
        &asset_server,
        PlatformTemplate::new(
            Vec3::new(288., -504., 2.),
            Vec2::new(384., 45.),
            "images/thick_ledge.png".to_string(),
        )
        .with_name("Thick Ledge 1".to_string()),
    );
}

pub fn spawn_vent_1(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_platform(
        &mut commands,
        &asset_server,
        PlatformTemplate::new(
            Vec3::new(-48., -432., 2.),
            Vec2::new(96., 96.),
            "images/vent.png".to_string(),
        )
        .with_name("Vent 1".to_string()),
    );
}

pub fn spawn_vent_2(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_platform(
        &mut commands,
        &asset_server,
        PlatformTemplate::new(
            Vec3::new(-240., -432., 2.),
            Vec2::new(96., 96.),
            "images/vent.png".to_string(),
        )
        .with_name("Vent 2".to_string()),
    );
}

pub fn spawn_vent_3(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_platform(
        &mut commands,
        &asset_server,
        PlatformTemplate::new(
            Vec3::new(-432., -432., 2.),
            Vec2::new(96., 96.),
            "images/vent.png".to_string(),
        )
        .with_name("Vent 3".to_string()),
    );
}

pub fn spawn_awning_2(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_platform(
        &mut commands,
        &asset_server,
        PlatformTemplate::new(
            Vec3::new(-144., -312., 2.),
            Vec2::new(96., 48.),
            "images/awning.png".to_string(),
        )
        .with_name("Awning 1".to_string()),
    );
}

pub fn spawn_awning_1(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_platform(
        &mut commands,
        &asset_server,
        PlatformTemplate::new(
            Vec3::new(-336., -312., 2.),
            Vec2::new(96., 48.),
            "images/awning.png".to_string(),
        )
        .with_name("Awning 2".to_string()),
    );
}

pub fn spawn_thin_ledge(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_platform(
        &mut commands,
        &asset_server,
        PlatformTemplate::new(
            Vec3::new(-240., -207., 2.),
            Vec2::new(160. * 3., 8. * 3.),
            "images/thin_ledge.png".to_string(),
        )
        .with_name("Thin ledge".to_string()),
    );
}

pub fn spawn_vertical_neon_sign(mut commands: Commands, asset_server: Res<AssetServer>) {
    let entity = spawn_platform(
        &mut commands,
        &asset_server,
        PlatformTemplate::new(
            Vec3::new(-624., -288., 2.),
            Vec2::new(32. * 3., 128. * 3.),
            "images/vertical_neon_sign.png".to_string(),
        )
        .with_name("Vertical neon sign".to_string()),
    );

    let neon_symbols = commands
        .spawn(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0., 0., 3.))
                .with_scale(Vec3::ONE * 3.),
            texture: asset_server.load("images/vertical_neon_symbols.png"),
            sprite: Sprite {
                color: Color::srgb(5., 5., 5.),
                ..default()
            },
            ..default()
        })
        .id();

    commands.entity(entity).push_children(&[neon_symbols]);
}

pub fn spawn_wall_tiles_1(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_platform(
        &mut commands,
        &asset_server,
        PlatformTemplate::new(
            Vec3::new(-432., -48., 2.),
            Vec2::new(32. * 3., 32. * 3.),
            "images/wall_tiles.png".to_string(),
        )
        .with_name("Wall tiles 1".to_string()),
    );
}

pub fn spawn_wall_tiles_2(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_platform(
        &mut commands,
        &asset_server,
        PlatformTemplate::new(
            Vec3::new(-240., -48., 2.),
            Vec2::new(32. * 3., 32. * 3.),
            "images/wall_tiles.png".to_string(),
        )
        .with_name("Wall tiles 2".to_string()),
    );
}

pub fn spawn_wall_tiles_3(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_platform(
        &mut commands,
        &asset_server,
        PlatformTemplate::new(
            Vec3::new(-48., -48., 2.),
            Vec2::new(32. * 3., 32. * 3.),
            "images/wall_tiles.png".to_string(),
        )
        .with_name("Wall tiles 3".to_string()),
    );
}

pub fn spawn_horizontal_neon_sign(mut commands: Commands, asset_server: Res<AssetServer>) {
    let entity = spawn_platform(
        &mut commands,
        &asset_server,
        PlatformTemplate::new(
            Vec3::new(288., -48., 2.),
            Vec2::new(128. * 3., 32. * 3.),
            "images/horizontal_neon_sign.png".to_string(),
        )
        .with_name("Horizontal neon sign".to_string()),
    );

    let neon_symbols = commands
        .spawn(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0., 0., 3.))
                .with_scale(Vec3::ONE * 3.),
            texture: asset_server.load("images/horizontal_neon_symbols.png"),
            sprite: Sprite {
                color: Color::srgb(0., 7.5, 7.5),
                ..default()
            },
            ..default()
        })
        .id();

    commands.entity(entity).push_children(&[neon_symbols]);
}

pub fn spawn_top_awning(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_platform(
        &mut commands,
        &asset_server,
        PlatformTemplate::new(
            Vec3::new(432., 72., 2.),
            Vec2::new(32. * 3., 16. * 3.),
            "images/awning.png".to_string(),
        )
        .with_name("Awning Top".to_string()),
    );
}

pub fn spawn_top_ledge(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_platform(
        &mut commands,
        &asset_server,
        PlatformTemplate::new(
            Vec3::new(720., 73.5, 2.),
            Vec2::new(96. * 3., 15. * 3.),
            "images/left_ledge.png".to_string(),
        )
        .with_name("Ledge Top".to_string()),
    );
}

pub fn spawn_skateboard(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_platform(
        &mut commands,
        &asset_server,
        PlatformTemplate::new(
            Vec3::new(816., 102., 2.),
            Vec2::new(26. * 3., 6. * 3.),
            "images/skateboard.png".to_string(),
        )
        .with_name("Skateboard".to_string()),
    );
}

pub fn despawn_game_platforms(
    mut commands: Commands,
    platforms_query: Query<Entity, With<GamePlatform>>,
) {
    for entity in &platforms_query {
        commands.entity(entity).despawn_recursive();
    }
}
