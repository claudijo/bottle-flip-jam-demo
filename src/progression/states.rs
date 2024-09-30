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
    Last,
}

impl LevelState {
    pub fn next(&self) -> Self {
        match self {
            LevelState::Initial => LevelState::First,
            LevelState::First => LevelState::Second,
            LevelState::Second => LevelState::Third,
            LevelState::Third => LevelState::Last,
            LevelState::Last => LevelState::Initial,
        }
    }
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
}
