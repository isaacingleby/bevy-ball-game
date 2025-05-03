use bevy::{
    app::{AppExit, Startup, Update},
    asset::AssetServer,
    audio::{AudioPlayer, PlaybackSettings},
    core_pipeline::core_2d::Camera2d,
    ecs::{
        change_detection::DetectChanges,
        component::Component,
        entity::Entity,
        event::{Event, EventReader, EventWriter},
        query::With,
        resource::Resource,
        system::{Commands, Query, Res, ResMut},
    },
    input::{ButtonInput, keyboard::KeyCode},
    math::Vec2,
    prelude::App,
    render::camera::Camera,
    sprite::Sprite,
    time::{Time, Timer, TimerMode},
    transform::components::Transform,
    utils::default,
    window::{PrimaryWindow, Window},
};

use rand::{random, seq::IndexedRandom};

use bevy_ball_game::dx12_plugin::dx12_plugin;

const PLAYER_SIZE: f32 = 64.0; // Size of the player sprite
const PLAYER_SIZE_HALF: f32 = PLAYER_SIZE / 2.0;
const PLAYER_SPEED: f32 = 500.0;

const MUMBER_OF_ENEMIES: usize = 4;
const ENEMY_SIZE: f32 = 64.0; // Size of the enemy sprite
const ENEMY_SIZE_HALF: f32 = ENEMY_SIZE / 2.0;
const ENEMY_SPEED: f32 = 200.0;
const ENEMY_SPAWN_TIME: f32 = 10.0; // Time in seconds to spawn a new enemy

const NUMBER_OF_STARS: usize = 10;
const STAR_SIZE: f32 = 30.0; // Size of the star sprite
const STAR_SIZE_HALF: f32 = STAR_SIZE / 2.0;
const STAR_SPAWN_TIME: f32 = 1.0; // Time in seconds to spawn a new star

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

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy {
    direction: Vec2,
}

#[derive(Component)]
struct Star;

#[derive(Resource, Default)]
struct Score {
    value: u32,
}

#[derive(Resource, Default)]
struct HighScores {
    scores: Vec<(String, u32)>,
}

#[derive(Resource)]
struct StarSpawnTimer {
    timer: Timer,
}

impl Default for StarSpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

#[derive(Resource)]
struct EnemySpawnTimer {
    timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

#[derive(Event)]
struct GameOverEvent;

fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single().unwrap();
    commands.spawn((
        Sprite {
            image: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        Player,
    ));
}

fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.single().unwrap();

    commands.spawn((
        Camera2d::default(),
        Camera {
            hdr: true,
            ..default()
        },
        Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
    ));
}

fn spawn_enemies(
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

fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single().unwrap();
    for _i in 0..NUMBER_OF_STARS {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            Sprite {
                image: asset_server.load("sprites/star.png"),
                ..default()
            },
            Transform::from_xyz(random_x, random_y, 0.0),
            Star,
        ));
    }
}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.single_mut() {
        let mut direction = Vec2::ZERO;
        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            direction += Vec2::new(-1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec2::new(1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            direction += Vec2::new(0.0, 1.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
            direction += Vec2::new(0.0, -1.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction.extend(0.0) * PLAYER_SPEED * time.delta_secs();
    }
}

fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single().unwrap();
    if let Ok(mut player_transform) = player_query.single_mut() {
        let x_min = PLAYER_SIZE_HALF;
        let x_max = window.width() - PLAYER_SIZE_HALF;
        let y_min = PLAYER_SIZE_HALF;
        let y_max = window.height() - PLAYER_SIZE_HALF;

        // Bound the player x position
        if player_transform.translation.x < x_min {
            player_transform.translation.x = x_min;
        } else if player_transform.translation.x > x_max {
            player_transform.translation.x = x_max;
        }
        // Bound the player y position
        if player_transform.translation.y < y_min {
            player_transform.translation.y = y_min;
        } else if player_transform.translation.y > y_max {
            player_transform.translation.y = y_max;
        }
    }
}

fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        transform.translation += enemy.direction.extend(0.0) * ENEMY_SPEED * time.delta_secs();
    }
}

fn update_enemy_direction(
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

fn confine_enemy_movement(
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

fn enemy_hit_player(
    mut commands: Commands,
    enemy_query: Query<&Transform, With<Enemy>>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    mut game_over_event_writer: EventWriter<GameOverEvent>,
    asset_server: Res<AssetServer>,
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
                game_over_event_writer.write(GameOverEvent);
            }
        }
    }
}

fn player_hit_star(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    mut star_query: Query<(Entity, &Transform), With<Star>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    if let Ok(player_transform) = player_query.single() {
        for (star_entity, star_transform) in star_query.iter_mut() {
            let distance = player_transform
                .translation
                .distance(star_transform.translation);
            if distance < (PLAYER_SIZE_HALF + STAR_SIZE_HALF) {
                println!("Player hit star!");
                score.value += 1;
                commands.spawn((
                    AudioPlayer::new(asset_server.load("audio/laserLarge_000.ogg")),
                    PlaybackSettings::DESPAWN,
                ));
                commands.entity(star_entity).despawn();
            }
        }
    }
}

fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value);
    }
}

fn increment_star_spawn_timer(time: Res<Time>, mut star_spawn_timer: ResMut<StarSpawnTimer>) {
    star_spawn_timer.timer.tick(time.delta());
}

fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    star_spawn_timer: Res<StarSpawnTimer>,
    asset_server: Res<AssetServer>,
) {
    if star_spawn_timer.timer.finished() {
        let window = window_query.single().unwrap();
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            Sprite {
                image: asset_server.load("sprites/star.png"),
                ..default()
            },
            Transform::from_xyz(random_x, random_y, 0.0),
            Star,
        ));
    }
}

fn increment_enemy_spawn_timer(time: Res<Time>, mut enemy_spawn_timer: ResMut<EnemySpawnTimer>) {
    enemy_spawn_timer.timer.tick(time.delta());
}

fn spawn_enemies_over_time(
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

fn handle_game_over(game_over_event_reader: EventReader<GameOverEvent>, score: Res<Score>) {
    if game_over_event_reader.len() > 0 {
        println!("Game Over! Final Score: {}", score.value);
    }
}

fn handle_high_scores(
    game_over_event_reader: EventReader<GameOverEvent>,
    mut high_scores: ResMut<HighScores>,
    score: Res<Score>,
) {
    if game_over_event_reader.len() > 0 {
        let player_name = "Player"; // Replace with actual player name input
        high_scores
            .scores
            .push((player_name.to_string(), score.value));
        high_scores.scores.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by score descending
        high_scores.scores.truncate(10); // Keep only top 10 scores
        println!("High Scores: {:?}", high_scores.scores);
    }
}

fn high_scores_updated(high_scores: Res<HighScores>) {
    if high_scores.is_changed() {
        println!("High Scores Updated: {:?}", high_scores.scores);
    }
}

fn exit_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.write(AppExit::Success);
    }
}
