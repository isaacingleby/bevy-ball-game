use bevy::ecs::event::Event;

#[derive(Event, Debug)]
pub struct GameOverEvent {
    pub score: u32,
}
