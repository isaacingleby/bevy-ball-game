mod components;
mod styles;
mod systems;

use bevy::prelude::*;
use systems::{
    interactions::*,
    layout::{despawn_main_menu, spawn_main_menu},
};

use crate::game::states_::AppState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            .add_systems(
                Update,
                (interact_with_play_button, interact_with_quit_button)
                    .run_if(in_state(AppState::MainMenu)),
            )
            .add_systems(OnExit(AppState::MainMenu), despawn_main_menu);
    }
}

pub fn main_menu() {
    println!("You are in the main menu!");
}
