use crate::score::components::ScoreDisplay;
use crate::score::resources::Score;
use bevy::prelude::*;

pub fn spawn_score_display(
    mut commands: Commands,
    mut score: ResMut<Score>,
    asset_server: Res<AssetServer>,
) {
    score.0 = 0;

    commands.spawn((
        TextBundle::from_section(
            format!("{:?}", score.0),
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
    ));
}
