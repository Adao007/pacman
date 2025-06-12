use bevy::prelude::*;
use bevy::color::palettes::basic::YELLOW;

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
            .add_systems(Update, (movement_input, boundary_enforcer));
    }
}

#[derive(PartialEq, Copy, Clone)]
enum Direction {
    None, 
    Up, 
    Down,
    Left,
    Right,
}

#[derive(Component)]
struct Player {
    // Linear speed in meters per second
    movement_speed: f32,
    direction: Direction,
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Player { movement_speed: 100.0, direction: Direction::None }, 
        Mesh2d(meshes.add(Circle::new(10.0))),
        MeshMaterial2d(material.add(Color::from(YELLOW))),
        Transform::from_xyz(0.0, 0.0, 1.0),
    ));
}

fn movement_input(
    input: Res<ButtonInput<KeyCode>>,
    mut player: Query<(&mut Transform, &mut Player)>, 
) {
    for (mut transform, mut p) in &mut player.iter_mut() {
        if input.pressed(KeyCode::ArrowUp) || input.pressed(KeyCode::KeyW) {
            p.direction = Direction::Up;
        }
        else if input.pressed(KeyCode::ArrowDown) || input.pressed(KeyCode::KeyR) {
            p.direction = Direction::Down;
        }
        else if input.pressed(KeyCode::ArrowLeft) || input.pressed(KeyCode::KeyA) {
            p.direction = Direction::Left;
        }
        else if input.pressed(KeyCode::ArrowRight) || input.pressed(KeyCode::KeyS) {
            p.direction = Direction::Right;
        }
        else {
            p.direction = p.direction;
        };

        match p.direction {
            Direction::Up =>    { transform.translation.y += 1.0; }
            Direction::Down =>  { transform.translation.y -= 1.0; }
            Direction::Left =>  { transform.translation.x -= 1.0; }
            Direction::Right => { transform.translation.x += 1.0; }
            Direction::None =>  { transform.translation += 0.0; }
        }
    }

}

fn boundary_enforcer(
    mut query: Query<&mut Transform, With<Player>>,
) {
    let bottom_boundary = BOT_BOUND + PADDING;
    let top_boundary: f32 = TOP_BOUND - PADDING;
    let left_boundary: f32 = LEFT_BOUND + PADDING;
    let right_boundary: f32 = RIGHT_BOUND - PADDING; 

    for mut transform in query.iter_mut() {
        // Stop at the Y Boundaries
        transform.translation.y = transform.translation.y.clamp(bottom_boundary, top_boundary);
        // Stop at the X Boundaries 
        transform.translation.x = transform.translation.x.clamp(left_boundary, right_boundary);
    }
}