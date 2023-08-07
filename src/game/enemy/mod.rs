use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

use systems::*;
use resources::*;

use crate::AppState;
use crate::game::SimulationState;

pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0; // Sprite size for an enemy.

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin
{
    fn build(&self, app: &mut App) 
    {
        app 
        // Resources
        .init_resource::<EnemySpawnTimer>()
        // Startup Systems
        //.add_startup_system(spawn_enemies)
        // Enter State Systems
        .add_system(spawn_enemies.in_schedule(OnEnter(AppState::Game)))
        // Systems
        //.add_system(enemies_movement)
        //.add_system(change_enemy_direction)
        .add_system(limit_enemy_movement
            .after(change_enemy_direction)
            .run_if(in_state(AppState::Game))
            .run_if(in_state(SimulationState::Running)))
        //.add_system(tick_enemy_spawn_timer)
        //.add_system(spawn_enemies_over_time)
        .add_systems(
            (
                enemies_movement,
                change_enemy_direction,
                tick_enemy_spawn_timer,
                spawn_enemies_over_time,
            )
            .in_set(OnUpdate(AppState::Game))
            .in_set(OnUpdate(SimulationState::Running))
        )
        // Exit State Systems
        .add_system(despawn_enemies.in_schedule(OnExit(AppState::Game)));
    }
}