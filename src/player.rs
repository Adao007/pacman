use bevy::prelude::*;
use bevy::color::palettes::basic::YELLOW;
use crate::movement::*;
use crate::collider::{Collider};
use crate::walls::Wall;

const TOP_BOUND:f32 = 320.0;
const BOT_BOUND:f32 = -320.0;
const LEFT_BOUND: f32 = -250.0;
const RIGHT_BOUND: f32 = 250.0;
const PADDING: f32 = 20.0;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(Update, apply_movement.after(movement_input));
    }
}

#[derive(Component)]
pub struct Player; 

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Player,
        Movement { speed: 100.0, direction: Direction::None, velocity: Vec2::new(0.0, 0.0)}, 
        Collider { size: Vec2::new(20.0, 20.0)},
        Mesh2d(meshes.add(Circle::new(10.0))),
        MeshMaterial2d(material.add(Color::from(YELLOW))),
        Transform::from_xyz(0.0, -150.0, 1.0),
    ));
}

// fn boundary_enforcer(
//     mut query: Query<&mut Transform, With<Player>>,
// ) {
//     let bottom_boundary = BOT_BOUND + PADDING;
//     let top_boundary: f32 = TOP_BOUND - PADDING;
//     let left_boundary: f32 = LEFT_BOUND + PADDING;
//     let right_boundary: f32 = RIGHT_BOUND - PADDING; 

//     for mut transform in query.iter_mut() {
//         // Stop at the Y Boundaries
//         transform.translation.y = transform.translation.y.clamp(bottom_boundary, top_boundary);
//         // Stop at the X Boundaries 
//         transform.translation.x = transform.translation.x.clamp(left_boundary, right_boundary);
//     }
// }

fn apply_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Movement), With<Player>>,
) {
    for (mut transform, movement) in &mut query {
        transform.translation += movement.velocity.extend(0.0) * time.delta_secs();
    }
}

