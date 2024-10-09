use crate::aerobat::components::{Aerobat, FlipMeter, Grounded, Resting};
use crate::aerobat::{count_landing_revolutions, evaluate_landing_type, LandingType};
use crate::progression::components::TargetPlatform;
use crate::progression::resources::RoundId;
use crate::progression::states::{GameState, LevelState, RoundState};
use crate::score::events::BonusEvent;
use crate::score::{CAP_FLIP_LAND_BONUS, FLIP_LAND_BONUS};
use bevy::prelude::*;

pub fn end_round(
    round_target_platform_query: Query<Entity, With<TargetPlatform>>,
    mut next_round_state: ResMut<NextState<RoundState>>,
    aerobat_query: Query<(&Grounded, &FlipMeter, &Transform), (With<Aerobat>, Added<Resting>)>,
    mut bonus_event_writer: EventWriter<BonusEvent>,
    round_id: Res<RoundId>,
) {
    for (grounded, flip_meter, transform) in &aerobat_query {
        let on_target_platform = round_target_platform_query
            .iter()
            .any(|target_platform| grounded.0 == Some(target_platform));

        let landing_type = evaluate_landing_type(&transform.rotation);
        let landing_revolutions = count_landing_revolutions(&landing_type, flip_meter.0);

        if landing_revolutions > 0 {
            match landing_type {
                LandingType::Upright => {
                    bonus_event_writer.send(BonusEvent::new(FLIP_LAND_BONUS, round_id.0));
                }
                LandingType::Cap => {
                    bonus_event_writer.send(BonusEvent::new(CAP_FLIP_LAND_BONUS, round_id.0));
                }
                _ => {}
            }
        }

        if on_target_platform {
            next_round_state.set(RoundState::Finished);
        } else {
            next_round_state.set(RoundState::Unfinished);
        }
    }
}

pub fn start_first_round(
    mut next_round_state: ResMut<NextState<RoundState>>,
    mut round_id: ResMut<RoundId>,
) {
    round_id.0 = 0;
    next_round_state.set(RoundState::Start);
}

pub fn start_next_round(
    level_state: Res<State<LevelState>>,
    mut next_level_state: ResMut<NextState<LevelState>>,
    mut next_round_state: ResMut<NextState<RoundState>>,
    mut round_id: ResMut<RoundId>,
) {
    round_id.0 += 1;
    next_level_state.set(level_state.next());
    next_round_state.set(RoundState::Start);
}

pub fn restart_round(
    mut next_round_state: ResMut<NextState<RoundState>>,
    mut round_id: ResMut<RoundId>,
) {
    round_id.0 += 1;
    next_round_state.set(RoundState::Start);
}

pub fn start_first_level(mut next_state: ResMut<NextState<LevelState>>) {
    next_state.set(LevelState::First)
}

pub fn reset_round_state(mut next_state: ResMut<NextState<RoundState>>) {
    next_state.set(RoundState::default());
}

pub fn reset_level_state(mut next_state: ResMut<NextState<LevelState>>) {
    next_state.set(LevelState::default());
}

pub fn reset_round_id(mut round_id: ResMut<RoundId>) {
    round_id.0 = 0;
}

pub fn restart_game(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::InGame);
}
