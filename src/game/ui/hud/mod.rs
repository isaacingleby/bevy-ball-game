pub mod components;
pub mod systems;

use bevy::prelude::*;
use systems::{layout::*, updates::*};

use crate::game::states_::AppState;

pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_hud_overlay)
            .add_systems(
                Update,
                (update_score_text, update_enemy_count).run_if(in_state(AppState::Game)),
            )
            .add_systems(OnExit(AppState::Game), despawn_hud_overlay);
    }
}
