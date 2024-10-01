use crate::config::ASSETS_SCALE_FACTOR;
use crate::main_menu::components::{MainMenu, PlayButton};
use crate::progression::states::{GameState, LevelState};
use bevy::prelude::*;

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Start,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            MainMenu,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    top: Val::Px(20.),
                    width: Val::Px(176. * ASSETS_SCALE_FACTOR),
                    height: Val::Px(64. * ASSETS_SCALE_FACTOR),
                    ..default()
                },
                image: UiImage::new(asset_server.load("images/jumbotron.png")),
                ..default()
            });

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            top: Val::Px(12. * ASSETS_SCALE_FACTOR),
                            width: Val::Px(64. * ASSETS_SCALE_FACTOR),
                            height: Val::Px(32. * ASSETS_SCALE_FACTOR),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        ..default()
                    },
                    PlayButton,
                ))
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        style: Style {
                            width: Val::Px(64. * ASSETS_SCALE_FACTOR),
                            height: Val::Px(32. * ASSETS_SCALE_FACTOR),
                            ..default()
                        },
                        image: UiImage::new(asset_server.load("images/play_button.png")),
                        ..default()
                    });
                });
        });
}

pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    for main_menu in &main_menu_query {
        commands.entity(main_menu).despawn_recursive();
    }
}

pub fn handle_play_button_interactions(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_progression_wave_state: ResMut<NextState<LevelState>>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            next_game_state.set(GameState::InGame);
            next_progression_wave_state.set(LevelState::First);
        }
    }
}
