use bevy::prelude::*; 
use bevy::color::palettes::basic::BLUE;
use crate::collider::Collider;

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup,(
                spawn_boundaries, 
                spawn_jupi,
                spawn_container,
            ));
    }
}

#[derive(Component)]
pub struct Wall;

fn spawn_wall (
    commands: &mut Commands,
    mesh: &mut ResMut<Assets<Mesh>>,
    material: &mut ResMut<Assets<ColorMaterial>>,
    size: Vec2,
    position: Vec2, 
) {
    commands.spawn((
        Collider { size: Vec2::new(size.x, size.y), position: Vec2::new(position.x, position.y) },
        Mesh2d(mesh.add(Rectangle::new(size.x, size.y))),
        MeshMaterial2d(material.add(Color::from(BLUE))),
        Transform::from_xyz(position.x, position.y, 1.0),
    ));
}

fn spawn_boundaries (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>, 
) {
    // Map size 1000.0 Width, 650.0 Height
    // Top Boundary
    spawn_wall(
        &mut commands, &mut meshes, &mut material,
         Vec2::new(500.0, 10.0), Vec2::new(0.0, 320.0)
    );

    // Bottom Boundary
    spawn_wall(
        &mut commands, &mut meshes, &mut material,
         Vec2::new(500.0, 10.0), Vec2::new(0.0, -320.0)
    );

    // Left Boundary
    spawn_wall(
        &mut commands, &mut meshes, &mut material,
         Vec2::new(10.0, 650.0), Vec2::new(-250.0, 0.0)
    );

    // Right Boundary 
    spawn_wall(
        &mut commands, &mut meshes, &mut material,
         Vec2::new(10.0, 650.0), Vec2::new(250.0, 0.0)
    );
}

fn spawn_jupi (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,
) {
    // Make J
    spawn_wall (
        &mut commands, &mut meshes, &mut material,
        Vec2::new(90.0, 10.0), Vec2::new(-170.0, 280.0)
    );
    spawn_wall (
        &mut commands, &mut meshes, &mut material,
        Vec2::new(10.0, 70.0), Vec2::new(-170.0, 240.0)
    );
    spawn_wall (
        &mut commands, &mut meshes, &mut material,
        Vec2::new(50.0, 10.0), Vec2::new(-190.0, 200.0)
    );

    // Make U
    spawn_wall (
        &mut commands, &mut meshes, &mut material,
        Vec2::new(10.0, 80.0), Vec2::new(-90.0, 245.0)
    );
    spawn_wall (
        &mut commands, &mut meshes, &mut material,
        Vec2::new(50.0, 10.0), Vec2::new(-70., 200.0)
    );
    spawn_wall (
        &mut commands, &mut meshes, &mut material,
        Vec2::new(10.0, 80.0), Vec2::new(-50.0, 245.0)
    );

    // Make P
    spawn_wall (
        &mut commands, &mut meshes, &mut material,
        Vec2::new(10.0, 45.0), Vec2::new(70.0, 260.0)
    );
    spawn_wall (
        &mut commands, &mut meshes, &mut material,
        Vec2::new(50.0, 10.0), Vec2::new(50.0, 240.0)
    );
    spawn_wall (
        &mut commands, &mut meshes, &mut material,
        Vec2::new(10.0, 90.0), Vec2::new(30.0, 240.0)
    );
    spawn_wall (
        &mut commands, &mut meshes, &mut material,
        Vec2::new(50.0, 10.0), Vec2::new(50.0, 280.0)
    );

    // Make I
    spawn_wall (
        &mut commands, &mut meshes, &mut material,
        Vec2::new(10.0, 80.0), Vec2::new(170.0, 240.0)
    );

    spawn_wall (
        &mut commands, &mut meshes, &mut material,
        Vec2::new(90.0, 10.0), Vec2::new(170.0, 280.0)
    );

    spawn_wall (
        &mut commands, &mut meshes, &mut material,
        Vec2::new(90.0, 10.0), Vec2::new(170.0, 200.0)
    );
}

fn spawn_container (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,
) {
    spawn_wall (
        &mut commands, &mut meshes, &mut material,
        Vec2::new(150.0, 10.0), Vec2::new(0.0, 0.0)
    );
}