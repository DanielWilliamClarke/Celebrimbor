use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Player {
    pub mass: f32,
}

#[derive(Component, Default)]
pub struct PlayerInput {
    pub direction: Vec2,
}
