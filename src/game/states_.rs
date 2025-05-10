use bevy::prelude::*;

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
