use crate::progression::GameState;
use crate::soundtrack::components::{FadeIn, FadeOut};
use crate::soundtrack::resources::SoundtrackPlayer;
use crate::soundtrack::FADE_TIME;
use bevy::prelude::*;

pub fn setup_tracks(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.insert_resource(SoundtrackPlayer::new(vec![
        asset_server.load::<AudioSource>("sounds/bit_bit_loop.ogg"),
        asset_server.load::<AudioSource>("sounds/blippy_trance.ogg"),
    ]));
}

pub fn change_tracks(
    mut commands: Commands,
    soundtrack_player: Res<SoundtrackPlayer>,
    soundtrack: Query<Entity, With<AudioSink>>,
    game_state: Res<State<GameState>>,
    mut state_change_event_reader: EventReader<StateTransitionEvent<GameState>>,
) {
    for _ in state_change_event_reader.read() {
        for track in soundtrack.iter() {
            // Fade out all currently running tracks
            commands.entity(track).insert(FadeOut);

            // Stop fading in all currently running tracks
            commands.entity(track).remove::<FadeIn>();
        }

        // Volume is set to start at zero and is then increased by the fade_in system.
        match game_state.get() {
            GameState::MainMenu => {
                commands.spawn((
                    AudioBundle {
                        source: soundtrack_player.track_list.first().unwrap().clone(),
                        settings: PlaybackSettings {
                            mode: bevy::audio::PlaybackMode::Loop,
                            volume: bevy::audio::Volume::ZERO,
                            ..default()
                        },
                    },
                    FadeIn,
                ));
            }
            GameState::InGame => {
                commands.spawn((
                    AudioBundle {
                        source: soundtrack_player.track_list.get(1).unwrap().clone(),
                        settings: PlaybackSettings {
                            mode: bevy::audio::PlaybackMode::Loop,
                            volume: bevy::audio::Volume::ZERO,
                            ..default()
                        },
                    },
                    FadeIn,
                ));
            }
        }
    }
}

// Fades in the audio of entities that has the FadeIn component. Removes the FadeIn component once
// full volume is reached.
pub fn fade_in(
    mut commands: Commands,
    mut audio_sink: Query<(&mut AudioSink, Entity), With<FadeIn>>,
    time: Res<Time>,
) {
    for (audio, entity) in audio_sink.iter_mut() {
        audio.set_volume(audio.volume() + time.delta_seconds() / FADE_TIME);
        if audio.volume() >= 1.0 {
            audio.set_volume(1.0);
            commands.entity(entity).remove::<FadeIn>();
        }
    }
}

// Fades out the audio of entities that has the FadeOut component. Despawns the entities once audio
// volume reaches zero.
pub fn fade_out(
    mut commands: Commands,
    mut audio_sink: Query<(&mut AudioSink, Entity), With<FadeOut>>,
    time: Res<Time>,
) {
    for (audio, entity) in audio_sink.iter_mut() {
        audio.set_volume(audio.volume() - time.delta_seconds() / FADE_TIME);
        if audio.volume() <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}
