use bevy::prelude::*;

pub mod resources;
mod systems;

use systems::*;

use crate::AppState;

pub struct EnemiesNumberPlugin;

impl Plugin for EnemiesNumberPlugin
{

    fn build(&self, app: &mut App) 
    {
        app
        // Enter State Systems
        .add_system(insert_enemies_number.in_schedule(OnEnter(AppState::Game)))
        // Systems
        .add_system(update_enemies_number.run_if(in_state(AppState::Game)))
        // Exit State Systems
        .add_system(remove_enemies_number.in_schedule(OnExit(AppState::Game)));
    }
}