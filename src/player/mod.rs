use bevy::prelude::*;
use systems::*;

pub mod components;
mod resources;
pub mod systems;

const PLAYER_SIZE: f32 = 64.0; // Size of the player sprite
pub const PLAYER_SIZE_HALF: f32 = PLAYER_SIZE / 2.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (player_movement, confine_player_movement));
    }
}
