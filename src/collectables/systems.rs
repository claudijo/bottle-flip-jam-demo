use crate::bottle::components::BottlePart;
use crate::collectables::components::Money;
use crate::free_hand_controller::resources::Grabbing;
use crate::progression::resources::RoundId;
use crate::score::events::ScoreEvent;
use crate::score::COLLECT_MONEY_SCORE;
use avian2d::prelude::{Collider, Collision, RigidBody, Sensor};
use bevy::prelude::*;

pub fn spawn_money_trail(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    trail_configs: Vec<(Vec3, usize)>,
) {
    let texture_handle = asset_server.load("images/money_sprite.png");
    let texture_atlas = TextureAtlasLayout::from_grid(UVec2::splat(24), 6, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    for (translation, atlas_index) in trail_configs {
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

pub fn spawn_money_trail_1(
    commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let configs = vec![
        (Vec3::new(-222., -686.5, 3.), 1),
        (Vec3::new(-196.5, -634.5, 3.), 0),
        (Vec3::new(-145., -604.5, 3.), 5),
        (Vec3::new(-94.5, -643.5, 3.), 3),
        (Vec3::new(-67.5, -693.0, 3.), 2),
    ];

    spawn_money_trail(commands, asset_server, texture_atlases, configs);
}

pub fn spawn_money_trail_2(
    commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let configs = vec![
        (Vec3::new(-25.5, -642., 3.), 1),
        (Vec3::new(3., -576., 3.), 0),
        (Vec3::new(61.5, -535.5, 3.), 5),
        (Vec3::new(126., -555., 3.), 3),
        (Vec3::new(166.5, -604.5, 3.), 2),
    ];

    spawn_money_trail(commands, asset_server, texture_atlases, configs);
}

pub fn spawn_money_trail_3(
    commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let configs = vec![
        (Vec3::new(198., -559.5, 3.), 1),
        (Vec3::new(222., -508.5, 3.), 0),
        (Vec3::new(264., -474., 3.), 5),
        (Vec3::new(309., -495., 3.), 3),
        (Vec3::new(336., -534., 3.), 2),
    ];

    spawn_money_trail(commands, asset_server, texture_atlases, configs);
}

pub fn spawn_money_trail_4(
    commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let configs = vec![
        (Vec3::new(174., -439.5, 3.), 1),
        (Vec3::new(201., -399.0, 3.), 0),
        (Vec3::new(247.5, -379.5, 3.), 5),
        (Vec3::new(286.5, -432., 3.), 3),
        (Vec3::new(309., -489., 3.), 2),
    ];

    spawn_money_trail(commands, asset_server, texture_atlases, configs);
}

pub fn spawn_money_trail_5(
    commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let configs = vec![
        (Vec3::new(-27., -345., 3.), 1),
        (Vec3::new(9., -304.5, 3.), 0),
        (Vec3::new(61.5, -285.0, 3.), 5),
        (Vec3::new(117., -324., 3.), 3),
        (Vec3::new(150., -388.5, 3.), 2),
    ];

    spawn_money_trail(commands, asset_server, texture_atlases, configs);
}

pub fn spawn_money_trail_6(
    commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let configs = vec![
        (Vec3::new(-235.5, -345., 3.), 1),
        (Vec3::new(-210., -294., 3.), 0),
        (Vec3::new(-156., -258.0, 3.), 5),
        (Vec3::new(-99., -277.5, 3.), 3),
        (Vec3::new(-64.5, -324., 3.), 2),
    ];

    spawn_money_trail(commands, asset_server, texture_atlases, configs);
}

pub fn spawn_money_trail_7(
    commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let configs = vec![
        (Vec3::new(-425.5, -345., 3.), 1),
        (Vec3::new(-400., -294., 3.), 0),
        (Vec3::new(-346., -258.0, 3.), 5),
        (Vec3::new(-289., -277.5, 3.), 3),
        (Vec3::new(-254.5, -324., 3.), 2),
    ];

    spawn_money_trail(commands, asset_server, texture_atlases, configs);
}

pub fn spawn_money_trail_8(
    commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let configs = vec![
        (Vec3::new(-427., -316.5, 3.), 1),
        (Vec3::new(-418., -252., 3.), 1),
        (Vec3::new(-395.5, -193.5, 3.), 0),
        (Vec3::new(-346., -201., 3.), 3),
        (Vec3::new(-329.5, -250.5, 3.), 2),
    ];

    spawn_money_trail(commands, asset_server, texture_atlases, configs);
}

pub fn spawn_money_trail_9(
    commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let configs = vec![
        (Vec3::new(-326.5, -231., 3.), 1),
        (Vec3::new(-301., -181.5, 3.), 0),
        (Vec3::new(-241., -157.5, 3.), 5),
        (Vec3::new(-190., -192., 3.), 3),
        (Vec3::new(-154.5, -238.5, 3.), 2),
    ];

    spawn_money_trail(commands, asset_server, texture_atlases, configs);
}

pub fn spawn_money_trail_10(
    commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let configs = vec![
        (Vec3::new(-350.5, -150., 3.), 1),
        (Vec3::new(-320.5, -100.5, 3.), 0),
        (Vec3::new(-253., -75., 3.), 5),
        (Vec3::new(-190., -127.5, 3.), 3),
        (Vec3::new(-157.5, -196.5, 3.), 2),
    ];

    spawn_money_trail(commands, asset_server, texture_atlases, configs);
}

pub fn spawn_money_trail_11(
    commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let configs = vec![
        (Vec3::new(-608.5, -40.5, 3.), 1),
        (Vec3::new(-566.5, 3., 3.), 0),
        (Vec3::new(-500.5, 21., 3.), 5),
        (Vec3::new(-431.5, -33., 3.), 3),
        (Vec3::new(-394.5, -108.5, 3.), 2),
    ];

    spawn_money_trail(commands, asset_server, texture_atlases, configs);
}

pub fn spawn_money_trail_12(
    commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let configs = vec![
        (Vec3::new(-602.5, -19.5, 3.), 1),
        (Vec3::new(-575.5, 45., 3.), 0),
        (Vec3::new(-526.5, 93., 3.), 5),
        (Vec3::new(-469.0, 78., 3.), 3),
        (Vec3::new(-433.5, 35.5, 3.), 2),
    ];

    spawn_money_trail(commands, asset_server, texture_atlases, configs);
}

pub fn spawn_money_trail_13(
    commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let configs = vec![
        (Vec3::new(-410.5, 66., 3.), 1),
        (Vec3::new(-380.5, 114., 3.), 0),
        (Vec3::new(-319.5, 136.5, 3.), 5),
        (Vec3::new(-268., 99., 3.), 3),
        (Vec3::new(-243., 47.5, 3.), 2),
    ];

    spawn_money_trail(commands, asset_server, texture_atlases, configs);
}

pub fn spawn_money_trail_14(
    commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let configs = vec![
        (Vec3::new(-220., 66., 3.), 1),
        (Vec3::new(-190., 114., 3.), 0),
        (Vec3::new(-129., 136.5, 3.), 5),
        (Vec3::new(-77.5, 99., 3.), 3),
        (Vec3::new(-52.5, 47.5, 3.), 2),
    ];

    spawn_money_trail(commands, asset_server, texture_atlases, configs);
}

pub fn spawn_money_trail_15(
    commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let configs = vec![
        (Vec3::new(-31., 67.5, 3.), 1),
        (Vec3::new(12.5, 132., 3.), 0),
        (Vec3::new(94.5, 168., 3.), 5),
        (Vec3::new(179., 120., 3.), 3),
        (Vec3::new(222., 47.5, 3.), 2),
    ];

    spawn_money_trail(commands, asset_server, texture_atlases, configs);
}

pub fn spawn_money_trail_16(
    commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let configs = vec![
        (Vec3::new(246.5, 72., 3.), 1),
        (Vec3::new(276.5, 153., 3.), 0),
        (Vec3::new(328.5, 216., 3.), 5),
        (Vec3::new(393.5, 196.5, 3.), 3),
        (Vec3::new(426., 146.5, 3.), 2),
    ];

    spawn_money_trail(commands, asset_server, texture_atlases, configs);
}

pub fn spawn_money_trail_17(
    commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let configs = vec![
        (Vec3::new(446., 151.5, 3.), 1),
        (Vec3::new(477.5, 208.5, 3.), 0),
        (Vec3::new(537., 255., 3.), 5),
        (Vec3::new(609.5, 225., 3.), 3),
        (Vec3::new(654., 164.5, 3.), 2),
    ];

    spawn_money_trail(commands, asset_server, texture_atlases, configs);
}

pub fn collect_money(
    mut commands: Commands,
    mut collision_event_reader: EventReader<Collision>,
    bottle_part_query: Query<Entity, With<BottlePart>>,
    money_query: Query<Entity, With<Money>>,
    grabbing: Res<Grabbing>,
    mut score_event_writer: EventWriter<ScoreEvent>,
    round_id: Res<RoundId>,
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
                    score_event_writer.send(ScoreEvent::new(COLLECT_MONEY_SCORE, round_id.0));

                    despawned.push(contacts.entity2);
                    commands.entity(contacts.entity2).despawn_recursive();
                } else if contacts.entity1 == money && contacts.entity2 == bottle_part {
                    score_event_writer.send(ScoreEvent::new(COLLECT_MONEY_SCORE, round_id.0));

                    despawned.push(contacts.entity1);
                    commands.entity(contacts.entity1).despawn_recursive();
                }
            }
        }
    }
}

pub fn despawn_money(mut commands: Commands, money_query: Query<Entity, With<Money>>) {
    for money in &money_query {
        commands.entity(money).despawn_recursive();
    }
}
