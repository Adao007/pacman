use bevy::prelude::*; 

mod map; 
mod player;
mod walls; 
mod collider;
mod movement;
use map::MapPlugin;
use player::PlayerPlugin;
use walls::WallPlugin;
use collider::*;
use movement::MovementPlugin;

fn main() {
    App::new()
        // .add_event::<CollisionEvent>()
        .add_plugins((
            DefaultPlugins, 
            MapPlugin,
            PlayerPlugin,
            WallPlugin,
            //ColliderPlugin,
            MovementPlugin,
        ))
        .run();
}
