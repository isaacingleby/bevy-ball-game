use bevy::prelude::*;
use systems::*;

use super::{SimulationState, states_::AppState};

pub mod components;
mod resources;
pub mod systems;

const PLAYER_SIZE: f32 = 64.0; // Size of the player sprite
pub const PLAYER_SIZE_HALF: f32 = PLAYER_SIZE / 2.0;

#[derive(SystemSet, Hash, Debug, PartialEq, Eq, Clone)]
pub struct MovementSystemSet;

#[derive(SystemSet, Hash, Debug, PartialEq, Eq, Clone)]
pub struct ConfinementSystemSet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, MovementSystemSet.before(ConfinementSystemSet))
            .add_systems(OnEnter(AppState::Game), spawn_player)
            .add_systems(
                Update,
                (
                    player_movement.in_set(MovementSystemSet),
                    confine_player_movement.in_set(ConfinementSystemSet),
                )
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(OnExit(AppState::Game), despawn_player);
    }
}
