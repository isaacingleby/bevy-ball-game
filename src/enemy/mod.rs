use bevy::prelude::*;
use resources::EnemySpawnTimer;
use systems::*;

mod components;
pub mod resources;
pub mod systems;

const ENEMY_SIZE: f32 = 64.0; // Size of the enemy sprite
const ENEMY_SIZE_HALF: f32 = ENEMY_SIZE / 2.0;
const MUMBER_OF_ENEMIES: usize = 4;
const ENEMY_SPEED: f32 = 200.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            .add_systems(Startup, spawn_enemies)
            .add_systems(
                Update,
                (
                    enemy_movement,
                    confine_enemy_movement,
                    update_enemy_direction,
                    enemy_hit_player,
                    spawn_enemies_over_time,
                    increment_enemy_spawn_timer,
                ),
            );
    }
}
