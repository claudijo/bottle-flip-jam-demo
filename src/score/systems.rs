use crate::score::components::{ScoreDisplay, ScoreIcon};
use crate::score::events::{BonusEvent, ScoreEvent};
use crate::score::resources::Score;
use bevy::prelude::*;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub fn spawn_score_display(
    mut commands: Commands,
    mut score: ResMut<Score>,
    asset_server: Res<AssetServer>,
) {
    score.0 = HashMap::<u64, u32>::new();

    commands.spawn((
        TextBundle::from_section(
            format!("{:?}", score.0.values().sum::<u32>()),
            TextStyle {
                font: asset_server.load("fonts/cyberpunk_pixel.otf"),
                font_size: 40.,
                color: Color::srgba(1., 1., 1., 0.8),
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            right: Val::Px(50.0),
            ..default()
        }),
        ScoreDisplay,
    ));
}

pub fn reset_score(mut score: ResMut<Score>) {
    score.0 = HashMap::<u64, u32>::new();
}

pub fn despawn_score_display(
    mut commands: Commands,
    score_display_query: Query<Entity, With<ScoreDisplay>>,
) {
    for entity in &score_display_query {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn spawn_score_icon(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle = asset_server.load("images/money_sprite.png");
    let texture_atlas = TextureAtlasLayout::from_grid(UVec2::splat(24), 6, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn((
        ImageBundle {
            style: Style {
                width: Val::Px(48.),
                height: Val::Px(48.),
                position_type: PositionType::Absolute,
                top: Val::Px(2.0),
                right: Val::Px(5.0),
                ..default()
            },
            image: UiImage::new(texture_handle).with_color(Color::srgba(1., 1., 1., 0.75)),
            ..default()
        },
        TextureAtlas::from(texture_atlas_handle),
        ScoreIcon,
    ));
}

pub fn despawn_score_icon(
    mut commands: Commands,
    score_display_query: Query<Entity, With<ScoreIcon>>,
) {
    for entity in &score_display_query {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn update_score_display(
    mut score_display_query: Query<&mut Text, With<ScoreDisplay>>,
    score: Res<Score>,
) {
    if score.is_changed() {
        for mut text in &mut score_display_query {
            text.sections[0].value = format!("{:?}", score.0.values().sum::<u32>());
        }
    }
}

pub fn collect_score(mut score: ResMut<Score>, mut score_event_reader: EventReader<ScoreEvent>) {
    for event in score_event_reader.read() {
        if let Entry::Vacant(vacant_entry) = score.0.entry(event.round_id) {
            vacant_entry.insert(event.value);
        } else {
            let new_score = score.0.get_mut(&event.round_id).unwrap();
            *new_score += event.value;
        }
    }
}

pub fn collect_bonus(mut score: ResMut<Score>, mut bonus_event_reader: EventReader<BonusEvent>) {
    for event in bonus_event_reader.read() {
        if let Some(new_score) = score.0.get_mut(&event.round_id) {
            *new_score *= event.value;
        }
    }
}
