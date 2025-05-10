pub mod enemy;
pub mod player;
pub mod score;
pub mod star;
pub mod states_;
mod systems;
mod ui;

use bevy::prelude::*;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use states_::*;
use systems::*;
use ui::*;

use crate::{events::GameOverEvent, systems::*};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SimulationState>()
            .add_event::<GameOverEvent>()
            .add_plugins(GameUiPlugin)
            .add_plugins(EnemyPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(ScorePlugin)
            .add_plugins(StarPlugin)
            .add_systems(Startup, spawn_camera)
            .add_systems(
                Update,
                (
                    toggle_simulation.run_if(in_state(AppState::Game)),
                    handle_game_over,
                    exit_game,
                ),
            );
    }
}
