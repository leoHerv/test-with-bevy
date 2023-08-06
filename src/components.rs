use bevy::prelude::*;

// A player
#[derive(Component)]
pub struct Player {}

// A enemy
#[derive(Component)]
pub struct Enemy
{
    pub direction: Vec2,
}

// A star
#[derive(Component)]
pub struct Star {}