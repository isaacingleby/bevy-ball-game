mod components;
mod styles;
mod systems;

use bevy::prelude::*;

use systems::{interactions::*, layout::*};

use crate::game::states_::*;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SimulationState::Paused), spawn_pause_menu)
        .add_systems(
            Update,
            (
                interact_with_resume_button,
                interact_with_main_menu_button,
                interact_with_quit_button,
            )
                .run_if(in_state(SimulationState::Paused)),
        )
        .add_systems(OnExit(SimulationState::Paused), despawn_pause_menu);
    }
}
