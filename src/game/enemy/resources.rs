use bevy::prelude::*;

const ENEMY_SPAWN_TIME: f32 = 10.0; // Time in seconds to spawn a new enemy

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

#[derive(Resource, Default)]
pub struct EnemySpawnCount {
    pub count: usize,
}
