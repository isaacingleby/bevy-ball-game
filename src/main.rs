use bevy::{
    app::{Startup, Update}, asset::AssetServer, audio::{AudioLoader, AudioPlayer, AudioSink, AudioSource, PlaybackSettings}, core_pipeline::core_2d::Camera2d, ecs::{
        component::Component,
        query::With,
        system::{Commands, Query, Res},
    }, input::{keyboard::KeyCode, ButtonInput}, math::Vec2, prelude::App, render::camera::Camera, sprite::Sprite, time::Time, transform::components::Transform, utils::default, window::{PrimaryWindow, Window}
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

fn main() {
    App::new()
        .add_plugins(dx12_plugin())
        .add_systems(Startup, (spawn_camera, spawn_player, spawn_enemies))
        .add_systems(
            Update,
            (
                player_movement,
                confine_player_movement,
                enemy_movement,
                confine_enemy_movement,
                update_enemy_direction,
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

        let mut translation = player_transform.translation;

        // Bound the player x position
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }
        // Bound the player y position
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
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

    let sound_effects = vec!("audio/pluck_001.ogg", "audio/pluck_002.ogg");

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
