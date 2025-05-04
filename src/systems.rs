use bevy::{
    app::AppExit,
    core_pipeline::core_2d::Camera2d,
    ecs::{
        event::{EventReader, EventWriter},
        query::With,
        system::{Commands, Query, Res},
    },
    input::{ButtonInput, keyboard::KeyCode},
    render::camera::Camera,
    transform::components::Transform,
    utils::default,
    window::{PrimaryWindow, Window},
};

use crate::events::GameOverEvent;

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
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

pub fn handle_game_over(mut game_over_event_reader: EventReader<GameOverEvent>) {
    for game_over_event in game_over_event_reader.read() {
        println!("Game Over! Final Score: {}", game_over_event.score);
    }
}

pub fn exit_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.write(AppExit::Success);
    }
}
