use bevy::prelude::*; 

mod map; 
mod player;
mod walls; 
use map::MapPlugin;
use player::PlayerPlugin;
use walls::WallPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins, 
            MapPlugin,
            PlayerPlugin,
            WallPlugin,
        ))
        .run();
}
