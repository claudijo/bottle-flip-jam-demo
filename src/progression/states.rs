use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum RoundState {
    #[default]
    Initial,
    Start,
    Finished,
    Unfinished,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum LevelState {
    #[default]
    Initial,
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth,
    Ninth,
    Tenth,
    Eleventh,
    Twelfth,
    Thirteenth,
    Fourteenth,
    Fifteenth,
    Sixteenth,
    Seventeenth,
    Last,
}

impl LevelState {
    pub fn next(&self) -> Self {
        match self {
            LevelState::Initial => LevelState::First,
            LevelState::First => LevelState::Second,
            LevelState::Second => LevelState::Third,
            LevelState::Third => LevelState::Fourth,
            LevelState::Fourth => LevelState::Fifth,
            LevelState::Fifth => LevelState::Sixth,
            LevelState::Sixth => LevelState::Seventh,
            LevelState::Seventh => LevelState::Eighth,
            LevelState::Eighth => LevelState::Ninth,
            LevelState::Ninth => LevelState::Tenth,
            LevelState::Tenth => LevelState::Eleventh,
            LevelState::Eleventh => LevelState::Twelfth,
            LevelState::Twelfth => LevelState::Thirteenth,
            LevelState::Thirteenth => LevelState::Fourteenth,
            LevelState::Fourteenth => LevelState::Fifteenth,
            LevelState::Fifteenth => LevelState::Sixteenth,
            LevelState::Sixteenth => LevelState::Seventeenth,
            LevelState::Seventeenth => LevelState::Last,
            LevelState::Last => LevelState::Initial,
        }
    }
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
    GameOptionsMenu,
    Restarting,
}
