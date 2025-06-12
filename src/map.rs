use bevy::prelude::*; 
use bevy::color::palettes::basic::BLACK;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>, 
) {
    // Lights, Camera, Map! 
    commands.spawn(Camera2d);

    // MAP 
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(600.0, 650.0))),
        MeshMaterial2d(material.add(Color::from(BLACK))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}