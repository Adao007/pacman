use bevy::prelude::*;
use crate::player::*;
use crate::collider::*;

pub struct MovementPlugin;

#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    None, 
    Up, 
    Down,
    Left,
    Right,
}

#[derive(Component)]
pub struct Movement {
    pub direction: Direction,
    pub speed: f32,
    pub velocity: Vec2,
}

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, movement_input);
    }
}

pub fn movement_input(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Movement, With<Player>>, 
) {
    for mut movement in query.iter_mut() { // iter_mut gives read write access
        if input.pressed(KeyCode::ArrowUp) || input.pressed(KeyCode::KeyW) {
            movement.direction = Direction::Up;
        }
        else if input.pressed(KeyCode::ArrowDown) || input.pressed(KeyCode::KeyR) {
            movement.direction = Direction::Down;
        }
        else if input.pressed(KeyCode::ArrowLeft) || input.pressed(KeyCode::KeyA) {
            movement.direction = Direction::Left;
        }
        else if input.pressed(KeyCode::ArrowRight) || input.pressed(KeyCode::KeyS) {
            movement.direction = Direction::Right;
        }
        
        movement.velocity = match movement.direction {
            Direction::Up => Vec2::new(0.0, 1.0),
            Direction::Down => Vec2::new(0.0, -1.0),
            Direction::Left => Vec2::new(-1.0, 0.0),
            Direction::Right => Vec2::new(1.0, 0.0),
            Direction::None => Vec2::ZERO,
        } * movement.speed;
    }
}