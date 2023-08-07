use bevy::prelude::*;

// A enemy
#[derive(Component)]
pub struct Enemy
{
    pub direction: Vec2,
}