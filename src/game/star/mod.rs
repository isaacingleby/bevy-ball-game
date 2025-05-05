mod components;
pub mod resources;
pub mod systems;

use bevy::prelude::*;
use resources::StarSpawnTimer;
use systems::*;

use super::{SimulationState, states_::AppState};

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            .add_systems(OnEnter(AppState::Game), spawn_stars)
            .add_systems(
                Update,
                (
                    player_hit_star,
                    increment_star_spawn_timer,
                    spawn_stars_over_time,
                )
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(OnExit(AppState::Game), despawn_stars);
    }
}
