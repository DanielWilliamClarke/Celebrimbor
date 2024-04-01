use bevy::prelude::*;

use crate::animation::Animator;
use crate::player::{Player, PlayerAnimations};

#[derive(Resource, Default)]
pub struct PlayerMovementInput {
    movement: Vec2,
}

#[derive(Component, Default)]
pub struct PlayerMovement {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub speed: f32,
    pub friction: f32,
}

pub struct PlayerMovementPlugin;

impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(PlayerMovementInput::default())
            .add_systems(Update, (sample_user_input, move_player).chain());
    }
}

pub fn sample_user_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut last_direction: ResMut<PlayerMovementInput>,
    mut query: Query<(&Player, &mut Animator<PlayerAnimations>)>,
) {
    for (_, mut animator) in query.iter_mut() {
        let mut movement = Vec2::default();

        animator.current_animation = PlayerAnimations::IDLE;

        if keys.pressed(KeyCode::KeyW) {
            movement.y = 1.0;

            animator.current_animation = PlayerAnimations::UP;
        }
        if keys.pressed(KeyCode::KeyS) {
            movement.y = -1.0;

            animator.current_animation = PlayerAnimations::DOWN;
        }
        if keys.pressed(KeyCode::KeyA) {
            movement.x = -1.0;
        }
        if keys.pressed(KeyCode::KeyD) {
            movement.x = 1.0;
        }

        last_direction.movement.clone_from(&movement);
    }
}

pub fn move_player(
    time: Res<Time>,
    input: Res<PlayerMovementInput>,
    mut query: Query<(&Player, &mut PlayerMovement, &mut Transform)>,
) {
    for (player, mut player_movement, mut transform) in query.iter_mut() {
        let damping = -player_movement.velocity * player_movement.friction;
        let input_force = input.movement * player_movement.speed;
        let force = input_force + damping;

        let acceleration = force / player.mass;
        let velocity = player_movement.velocity + force * time.delta_seconds();
        let new_position = player_movement.position + velocity * time.delta_seconds();

        player_movement.acceleration = acceleration;
        player_movement.velocity = velocity;
        player_movement.position = new_position;

        transform.translation = Vec3::new(
            player_movement.position.x,
            player_movement.position.y,
            0.0,
        );
    }
}
