use bevy::prelude::*;

use crate::game::{
    enemy::resources::EnemySpawnCount,
    score::resources::Score,
    ui::hud::components::*,
};

pub fn update_score_text(mut text_query: Query<&mut Text, With<ScoreText>>, score: Res<Score>) {
    if score.is_changed() {
        if let Ok(mut text) = text_query.single_mut() {
            text.0 = format!("{}", score.value);
        }
    }
}

pub fn update_enemy_count(
    mut text_query: Query<&mut Text, With<EnemyText>>,
    enemy_count: Res<EnemySpawnCount>,
) {
    if let Ok(mut text) = text_query.single_mut() {
        text.0 = format!("{:?}", enemy_count.count);
    }
}
