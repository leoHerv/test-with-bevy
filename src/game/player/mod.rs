use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

use crate::AppState;
use crate::game::SimulationState;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0; // Sprite size for the player.

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin
{
    fn build(&self, app: &mut App) 
    {
        app 
        // Startup Systems
        //.add_startup_system(spawn_player)
        // Enter State Systems
        .add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
        // Systems
        //.add_system(player_movement)
        .add_system(limit_player_movement
            .after(player_movement)
            .run_if(in_state(AppState::Game))
            .run_if(in_state(SimulationState::Running)))
        //.add_system(enemy_hit_player)
        //.add_system(player_hit_star)
        .add_systems(
            (
                player_movement,
                enemy_hit_player,
                player_hit_star,
            )
            .in_set(OnUpdate(AppState::Game))
            .in_set(OnUpdate(SimulationState::Running))
        )
        // Exit State Systems
        .add_system(despawn_players.in_schedule(OnExit(AppState::Game)));

    }
}