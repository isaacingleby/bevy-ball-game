use bevy::{
    ecs::resource::Resource,
    time::{Timer, TimerMode},
};

const STAR_SPAWN_TIME: f32 = 1.0; // Time in seconds to spawn a new star
const ENEMY_SPAWN_TIME: f32 = 10.0; // Time in seconds to spawn a new enemy

#[derive(Resource, Default)]
pub struct Score {
    pub value: u32,
}

#[derive(Resource, Default)]
pub struct HighScores {
    pub scores: Vec<(String, u32)>,
}

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}

impl Default for StarSpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

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
