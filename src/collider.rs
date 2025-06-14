use bevy::prelude::*; 
use crate::player::Player;
use crate::walls::Wall;
use crate::movement::*;

pub struct ColliderPlugin;

#[derive(Component)]
pub struct Collider {
    pub size: Vec2,
    pub position: Vec2,
}

impl Plugin for ColliderPlugin {
    fn build(&self, app: &mut App) {
        
          
    }
}

