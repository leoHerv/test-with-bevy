use bevy::prelude::*;

use crate::game::enemy::*;

// To have the current enemies number.
#[derive(Resource)]
pub struct EnemiesNumber 
{
    pub value: usize,
}

impl Default for EnemiesNumber
{
    fn default() -> Self 
    {
        EnemiesNumber {value: NUMBER_OF_ENEMIES}
    }
}