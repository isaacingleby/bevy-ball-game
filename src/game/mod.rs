use bevy::prelude::*;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;

use crate::{events::GameOverEvent, systems::*};

pub mod enemy;
pub mod player;
pub mod score;
pub mod star;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameOverEvent>()
            .add_plugins(EnemyPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(ScorePlugin)
            .add_plugins(StarPlugin)
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, (handle_game_over, exit_game));
    }
}
