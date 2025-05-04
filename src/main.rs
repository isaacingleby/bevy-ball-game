use bevy::{
    app::{Startup, Update},
    prelude::App,
};
use bevy_ball_game::{
    dx12_plugin::dx12_plugin,
    enemy::{resources::*, systems::*},
    events::GameOverEvent,
    player::systems::*,
    score::{resources::*, systems::*},
    star::{resources::*, systems::*},
    systems::*,
};

fn main() {
    App::new()
        .add_plugins(dx12_plugin())
        .init_resource::<Score>()
        .init_resource::<HighScores>()
        .init_resource::<StarSpawnTimer>()
        .init_resource::<EnemySpawnTimer>()
        .add_event::<GameOverEvent>()
        .add_systems(
            Startup,
            (spawn_camera, spawn_player, spawn_enemies, spawn_stars),
        )
        .add_systems(
            Update,
            (
                update_score,
                increment_star_spawn_timer,
                player_movement,
                confine_player_movement,
                player_hit_star,
                spawn_stars_over_time,
                increment_enemy_spawn_timer,
                spawn_enemies_over_time,
                enemy_movement,
                confine_enemy_movement,
                update_enemy_direction,
                enemy_hit_player,
                handle_game_over,
                handle_high_scores,
                exit_game,
            ),
        )
        .run();
}
