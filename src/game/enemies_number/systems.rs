use bevy::prelude::*;

use super::resources::*;


pub fn insert_enemies_number(
    mut commands: Commands,
)
{
    commands.insert_resource(EnemiesNumber::default());
}

pub fn remove_enemies_number(
    mut commands: Commands,
)
{
    commands.remove_resource::<EnemiesNumber>();
}

pub fn update_enemies_number(
    enemies_number: Res<EnemiesNumber>,
)
{
    if enemies_number.is_changed()
    {
        println!("Enemies number : {}", enemies_number.value.to_string());
    }
}