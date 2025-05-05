use bevy::{prelude::*, window::PrimaryWindow};
use rand::random;

use crate::game::{
    player::{PLAYER_SIZE_HALF, components::Player},
    score::resources::Score,
};

use super::{components::Star, resources::StarSpawnTimer};

const NUMBER_OF_STARS: usize = 10;
const STAR_SIZE: f32 = 30.0; // Size of the star sprite
const STAR_SIZE_HALF: f32 = STAR_SIZE / 2.0;

pub fn spawn_stars(
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

pub fn despawn_stars(
    mut commands: Commands,
    star_query: Query<Entity, With<Star>>,
) {
    for star_entity in star_query.iter() {
        commands.entity(star_entity).despawn();
    }
}

pub fn player_hit_star(
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

pub fn increment_star_spawn_timer(time: Res<Time>, mut star_spawn_timer: ResMut<StarSpawnTimer>) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time(
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
