use bevy::{prelude::*, window::PrimaryWindow};
use rand::{random, seq::IndexedRandom};

use crate::{
    events::GameOverEvent,
    game::{
        player::{PLAYER_SIZE_HALF, components::Player},
        score::resources::Score,
    },
};

use super::{
    ENEMY_SIZE_HALF, ENEMY_SPEED, MUMBER_OF_ENEMIES, components::Enemy, resources::EnemySpawnTimer,
};

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single().unwrap();
    for _i in 0..MUMBER_OF_ENEMIES {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            Sprite {
                image: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Transform::from_xyz(random_x, random_y, 0.0),
            Enemy {
                direction: Vec2::new(random(), random()),
            },
        ));
    }
}

pub fn despawn_enemies(mut commands: Commands, enemy_query: Query<Entity, With<Enemy>>) {
    for enemy_entity in enemy_query.iter() {
        commands.entity(enemy_entity).despawn();
    }
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        transform.translation += enemy.direction.extend(0.0) * ENEMY_SPEED * time.delta_secs();
    }
}

pub fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single().unwrap();
    let x_min = ENEMY_SIZE_HALF;
    let x_max = window.width() - ENEMY_SIZE_HALF;
    let y_min = ENEMY_SIZE_HALF;
    let y_max = window.height() - ENEMY_SIZE_HALF;

    for mut transform in enemy_query.iter_mut() {
        // Bound the enemy x position
        if transform.translation.x < x_min {
            transform.translation.x = x_min;
        } else if transform.translation.x > x_max {
            transform.translation.x = x_max;
        }
        // Bound the enemy y position
        if transform.translation.y < y_min {
            transform.translation.y = y_min;
        } else if transform.translation.y > y_max {
            transform.translation.y = y_max;
        }
    }
}

pub fn update_enemy_direction(
    mut commands: Commands,
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single().unwrap();
    let x_min = ENEMY_SIZE_HALF;
    let x_max = window.width() - ENEMY_SIZE_HALF;
    let y_min = ENEMY_SIZE_HALF;
    let y_max = window.height() - ENEMY_SIZE_HALF;

    let sound_effects = vec!["audio/pluck_001.ogg", "audio/pluck_002.ogg"];

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let mut direction_has_changed = false;

        // Check if the enemy is out of bounds and if so reverse its direction
        if transform.translation.x < x_min || transform.translation.x > x_max {
            enemy.direction.x *= -1.0;
            direction_has_changed = true;
        }
        if transform.translation.y < y_min || transform.translation.y > y_max {
            enemy.direction.y *= -1.0;
            direction_has_changed = true;
        }

        // Play sound effect if direction changes
        if direction_has_changed {
            let sound_effect_path = sound_effects.choose(&mut rand::rng()).unwrap();
            commands.spawn((
                AudioPlayer::new(asset_server.load(*sound_effect_path)),
                PlaybackSettings::DESPAWN,
            ));
        }
    }
}

pub fn enemy_hit_player(
    mut commands: Commands,
    enemy_query: Query<&Transform, With<Enemy>>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    mut game_over_event_writer: EventWriter<GameOverEvent>,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
) {
    if let Ok((player_entity, player_transform)) = player_query.single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            if distance < (PLAYER_SIZE_HALF + ENEMY_SIZE_HALF) {
                println!("Enemy hit player! Game Over!");
                commands.spawn((
                    AudioPlayer::new(asset_server.load("audio/explosionCrunch_000.ogg")),
                    PlaybackSettings::DESPAWN,
                ));
                commands.entity(player_entity).despawn();
                game_over_event_writer.write(GameOverEvent { score: score.value });
            }
        }
    }
}

pub fn spawn_enemies_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    enemy_spawn_timer: Res<EnemySpawnTimer>,
    asset_server: Res<AssetServer>,
) {
    if enemy_spawn_timer.timer.finished() {
        let window = window_query.single().unwrap();
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            Sprite {
                image: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Transform::from_xyz(random_x, random_y, 0.0),
            Enemy {
                direction: Vec2::new(random(), random()).normalize(),
            },
        ));
    }
}

pub fn increment_enemy_spawn_timer(
    time: Res<Time>,
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
) {
    enemy_spawn_timer.timer.tick(time.delta());
}
