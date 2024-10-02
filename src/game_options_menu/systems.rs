use crate::game_options_menu::components::{
    CancelButton, GameOptionsButton, GameOptionsMenu, QuitButton, RestartButton, ResumeButton,
};
use crate::progression::states::GameState;
use bevy::prelude::*;

pub fn spawn_game_options_button(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            ButtonBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(8.),
                    left: Val::Px(8.),
                    width: Val::Px(48.),
                    height: Val::Px(48.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            GameOptionsButton,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    width: Val::Px(32.),
                    height: Val::Px(32.),
                    ..default()
                },
                image: UiImage::new(asset_server.load("images/settings_icon.png"))
                    .with_color(Color::srgba(1., 1., 1., 0.75)),
                ..default()
            });
        });
}

pub fn despawn_game_options_button(
    mut commands: Commands,
    options_button_query: Query<Entity, With<GameOptionsButton>>,
) {
    for options_button in &options_button_query {
        commands.entity(options_button).despawn_recursive();
    }
}

pub fn handle_game_options_button_interactions(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<GameOptionsButton>)>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            next_game_state.set(GameState::GameOptionsMenu);
        }
    }
}

pub fn spawn_game_options_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor::from(Color::srgba(0., 0., 0., 0.6)),
                ..default()
            },
            GameOptionsMenu,
        ))
        .with_children(|parent| {
            parent
                .spawn(ImageBundle {
                    style: Style {
                        width: Val::Px(316.),
                        height: Val::Px(304.),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Start,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    image: UiImage::new(asset_server.load("images/options_menu.png")),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                padding: UiRect::px(22., 6., 10., 10.),
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::SpaceBetween,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(
                                TextBundle::from_section(
                                    "Options",
                                    TextStyle {
                                        font: asset_server.load("fonts/cyberpunk_pixel.otf"),
                                        font_size: 28.,
                                        color: Color::WHITE,
                                    },
                                )
                                .with_style(Style {
                                    left: Val::Px(12.),
                                    ..default()
                                }),
                            );

                            parent
                                .spawn((
                                    ButtonBundle {
                                        style: Style {
                                            width: Val::Px(32.),
                                            height: Val::Px(32.),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    CancelButton,
                                ))
                                .with_children(|parent| {
                                    parent.spawn(ImageBundle {
                                        style: Style {
                                            width: Val::Px(32.),
                                            height: Val::Px(32.),

                                            ..default()
                                        },
                                        image: UiImage::new(
                                            asset_server.load("images/cross_icon.png"),
                                        )
                                        .with_color(Color::srgb(
                                            210. / 255.,
                                            47. / 255.,
                                            30. / 255.,
                                        )),

                                        ..default()
                                    });
                                });
                        });

                    // Resume button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    margin: UiRect {
                                        top: Val::Px(8.),
                                        ..default()
                                    },
                                    width: Val::Px(256.),
                                    height: Val::Px(64.),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                ..default()
                            },
                            ResumeButton,
                        ))
                        .with_children(|parent| {
                            parent
                                .spawn(ImageBundle {
                                    style: Style {
                                        width: Val::Px(256.),
                                        height: Val::Px(64.),
                                        padding: UiRect {
                                            left: Val::Px(8.),
                                            right: Val::Px(8.),
                                            top: Val::Px(6.),
                                            bottom: Val::Px(0.),
                                        },
                                        flex_direction: FlexDirection::Row,
                                        justify_content: JustifyContent::Start,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    image: UiImage::new(
                                        asset_server.load("images/options_menu_button.png"),
                                    ),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn(ImageBundle {
                                        style: Style {
                                            width: Val::Px(32.),
                                            height: Val::Px(32.),
                                            flex_direction: FlexDirection::Row,
                                            justify_content: JustifyContent::Start,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        image: UiImage::new(
                                            asset_server.load("images/play_icon.png"),
                                        ),
                                        ..default()
                                    });
                                    parent.spawn(
                                        TextBundle::from_section(
                                            "Resume",
                                            TextStyle {
                                                font: asset_server
                                                    .load("fonts/cyberpunk_pixel.otf"),
                                                font_size: 38.,
                                                color: Color::WHITE,
                                            },
                                        )
                                        .with_style(
                                            Style {
                                                left: Val::Px(12.),
                                                ..default()
                                            },
                                        ),
                                    );
                                });
                        });

                    // Restart button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    margin: UiRect {
                                        top: Val::Px(16.),
                                        ..default()
                                    },
                                    width: Val::Px(256.),
                                    height: Val::Px(64.),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                ..default()
                            },
                            RestartButton,
                        ))
                        .with_children(|parent| {
                            parent
                                .spawn(ImageBundle {
                                    style: Style {
                                        width: Val::Px(256.),
                                        height: Val::Px(64.),
                                        padding: UiRect {
                                            left: Val::Px(8.),
                                            right: Val::Px(8.),
                                            top: Val::Px(6.),
                                            bottom: Val::Px(0.),
                                        },
                                        flex_direction: FlexDirection::Row,
                                        justify_content: JustifyContent::Start,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    image: UiImage::new(
                                        asset_server.load("images/options_menu_button.png"),
                                    ),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn(ImageBundle {
                                        style: Style {
                                            width: Val::Px(32.),
                                            height: Val::Px(32.),
                                            flex_direction: FlexDirection::Row,
                                            justify_content: JustifyContent::Start,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        image: UiImage::new(
                                            asset_server.load("images/restart_icon.png"),
                                        ),
                                        ..default()
                                    });
                                    parent.spawn(
                                        TextBundle::from_section(
                                            "Restart",
                                            TextStyle {
                                                font: asset_server
                                                    .load("fonts/cyberpunk_pixel.otf"),
                                                font_size: 38.,
                                                color: Color::WHITE,
                                            },
                                        )
                                        .with_style(
                                            Style {
                                                left: Val::Px(12.),
                                                ..default()
                                            },
                                        ),
                                    );
                                });
                        });

                    // Quit
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    margin: UiRect {
                                        top: Val::Px(16.),
                                        ..default()
                                    },
                                    width: Val::Px(256.),
                                    height: Val::Px(64.),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                ..default()
                            },
                            QuitButton,
                        ))
                        .with_children(|parent| {
                            parent
                                .spawn(ImageBundle {
                                    style: Style {
                                        width: Val::Px(256.),
                                        height: Val::Px(64.),
                                        padding: UiRect {
                                            left: Val::Px(8.),
                                            right: Val::Px(8.),
                                            top: Val::Px(6.),
                                            bottom: Val::Px(0.),
                                        },
                                        flex_direction: FlexDirection::Row,
                                        justify_content: JustifyContent::Start,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    image: UiImage::new(
                                        asset_server.load("images/options_menu_button.png"),
                                    ),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn(ImageBundle {
                                        style: Style {
                                            width: Val::Px(32.),
                                            height: Val::Px(32.),
                                            flex_direction: FlexDirection::Row,
                                            justify_content: JustifyContent::Start,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        image: UiImage::new(
                                            asset_server.load("images/exit_icon.png"),
                                        ),
                                        ..default()
                                    });
                                    parent.spawn(
                                        TextBundle::from_section(
                                            "Quit",
                                            TextStyle {
                                                font: asset_server
                                                    .load("fonts/cyberpunk_pixel.otf"),
                                                font_size: 38.,
                                                color: Color::WHITE,
                                            },
                                        )
                                        .with_style(
                                            Style {
                                                left: Val::Px(12.),
                                                ..default()
                                            },
                                        ),
                                    );
                                });
                        });
                });
        });
}

pub fn despawn_game_options_menu(
    mut commands: Commands,
    game_options_menu_query: Query<Entity, With<GameOptionsMenu>>,
) {
    for entity in &game_options_menu_query {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn handle_cancel_or_resume_button_interactions(
    interaction_query: Query<
        &Interaction,
        (
            Changed<Interaction>,
            Or<(With<CancelButton>, With<ResumeButton>)>,
        ),
    >,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            next_game_state.set(GameState::InGame);
        }
    }
}

pub fn handle_restart_button_interaction(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<RestartButton>)>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            next_game_state.set(GameState::Restarting);
        }
    }
}

pub fn handle_quit_button_interaction(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<QuitButton>)>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            next_game_state.set(GameState::MainMenu);
        }
    }
}
