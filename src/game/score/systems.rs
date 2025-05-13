use bevy::prelude::*;

use crate::events::GameOverEvent;

use super::resources::{HighScores, Score};

pub fn insert_score(mut commands: Commands) {
    commands.insert_resource(Score::default());
}

pub fn reset_score(mut commands: Commands) {
    commands.remove_resource::<Score>();
}

pub fn handle_high_scores(
    mut game_over_event_reader: EventReader<GameOverEvent>,
    mut high_scores: ResMut<HighScores>,
    score: Res<Score>,
) {
    for _game_over_event in game_over_event_reader.read() {
        let player_name = "Player"; // Replace with actual player name input
        high_scores
            .scores
            .push((player_name.to_string(), score.value));
        high_scores.scores.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by score descending
        high_scores.scores.truncate(10); // Keep only top 10 scores
        println!("High Scores: {:?}", high_scores.scores);
        return;
    }
}

/// This system was suggested by the tutorial to await for the high scores to be updated
/// but `handle_high_scores` already does that task well enough.
pub fn _high_scores_updated(high_scores: Res<HighScores>) {
    if high_scores.is_changed() {
        println!("High Scores Updated: {:?}", high_scores.scores);
    }
}

pub fn _update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value);
    }
}
