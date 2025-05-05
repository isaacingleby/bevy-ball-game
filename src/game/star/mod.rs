mod components;
pub mod resources;
pub mod systems;

use bevy::prelude::*;
use resources::StarSpawnTimer;
use systems::*;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            .add_systems(Startup, spawn_stars)
            .add_systems(
                Update,
                (
                    player_hit_star,
                    increment_star_spawn_timer,
                    spawn_stars_over_time,
                ),
            );
    }
}
