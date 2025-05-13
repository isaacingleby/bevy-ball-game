pub mod resources;
pub mod systems;

use bevy::prelude::*;
use resources::*;
use systems::*;

use super::states_::AppState;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HighScores>()
            .add_systems(OnEnter(AppState::Game), insert_score)
            .add_systems(Update, handle_high_scores.run_if(in_state(AppState::Game)))
            .add_systems(OnExit(AppState::Game), reset_score);
    }
}
