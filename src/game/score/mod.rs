pub mod resources;
pub mod systems;

use bevy::prelude::*;
use resources::*;
use systems::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .init_resource::<HighScores>()
            .add_systems(Update, (update_score, handle_high_scores));
    }
}
