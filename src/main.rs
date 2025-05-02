use bevy::{
    app::{Startup, Update},
    asset::AssetServer,
    core_pipeline::core_2d::Camera2d,
    ecs::{
        component::Component,
        query::With,
        schedule::IntoScheduleConfigs,
        system::{Commands, Query, Res},
    },
    input::{ButtonInput, keyboard::KeyCode},
    math::Vec2,
    prelude::App,
    render::camera::Camera,
    sprite::Sprite,
    time::Time,
    transform::components::Transform,
    utils::default,
    window::{PrimaryWindow, Window},
};

use rand::random;

use bevy_ball_game::dx12_plugin::dx12_plugin;

const PLAYER_SIZE: f32 = 64.0;  // Size of the player sprite
const PLAYER_SIZE_HALF: f32 = PLAYER_SIZE / 2.0;
const PLAYER_SPEED: f32 = 500.0;

const MUMBER_OF_ENEMIES: usize = 4;
const ENEMY_SPEED: f32 = 200.0;

fn main() {
    App::new()
        .add_plugins(dx12_plugin())
        .add_systems(Startup, (spawn_camera, spawn_player, spawn_enemies).chain())
        .add_systems(Update, (player_movement, confine_player_movement).chain())
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
                direction: Vec2::new(random(), random())
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
        // let window = window_query.single().unwrap();

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
