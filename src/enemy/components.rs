use bevy::{ecs::component::Component, math::Vec2};

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
}
