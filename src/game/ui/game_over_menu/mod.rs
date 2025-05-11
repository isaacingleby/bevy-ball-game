mod components;
mod systems;

use bevy::prelude::*;
use systems::{interactions::*, layout::*};

use crate::game::states_::AppState;

pub struct GameOverMenuPlugin;

impl Plugin for GameOverMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameOver), spawn_game_over_menu)
            .add_systems(
                Update,
                (
                    interact_with_restart_button,
                    interact_with_main_menu_button,
                    interact_with_quit_button,
                )
                    .run_if(in_state(AppState::GameOver)),
            )
            .add_systems(OnExit(AppState::GameOver), despawn_game_over_menu);
    }
}
