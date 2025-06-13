use bevy::prelude::*; 
use crate::player::Player;
use crate::walls::Wall;
use crate::movement::*;

pub struct ColliderPlugin;

#[derive(Component)]
pub struct Collider {
    pub size: Vec2,
}

#[derive(Event)]
pub struct CollisionEvent {
    pub entity: Entity,
    pub collided: Entity,
}

impl Plugin for ColliderPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                check_collisions,
                handle_collisions.after(check_collisions)
            ));
    }
}

pub fn check_collisions(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform, &Collider), With<Player>>,
    wall_query: Query<(Entity, &Transform, &Collider), With<Wall>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let Ok((
        player_entity, 
        player_transform, 
        player_collider
    )) = player_query.single_mut() else {
        error!("Expected exactly one player entity!");
        return;
    };

    for (wall_entity, wall_transform, wall_collider) in wall_query.iter() {
        if collides(
            player_transform.translation.truncate(), 
            player_collider.size, 
            wall_transform.translation.truncate(), 
            wall_collider.size,
        ) {
            collision_events.write(CollisionEvent {
                entity: player_entity,
                collided: wall_entity,
            });
        }
    }
}

pub fn collides(
    pos1: Vec2, size1: Vec2, 
    pos2:Vec2, size2: Vec2,
) -> bool {
    let half_size1 = size1 / 2.0;
    let half_size2 = size2 / 2.0; 

    pos1.x - half_size1.x < pos2.x + half_size2.x &&
    pos1.x + half_size1.x > pos2.x - half_size2.x &&
    pos1.y - half_size1.y < pos2.y + half_size2.y &&
    pos1.y + half_size1.y > pos2.y - half_size2.y
}

pub fn handle_collisions(
    mut collision_events: EventReader<CollisionEvent>,
    mut player_query: Query<&mut Movement, With<Player>>,
) {
    for event in collision_events.read() {
        if let Ok(mut movement) = player_query.get_mut(event.entity) {
            movement.velocity = Vec2::ZERO;
            movement.direction = Direction::None;
        }
    }
}