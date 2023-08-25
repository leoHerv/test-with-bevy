use bevy::prelude::*;

pub mod enemy;
pub mod player;
pub mod score;
pub mod star;
pub mod systems;
pub mod enemies_number;

use crate::events::*;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::*;
use enemies_number::EnemiesNumberPlugin;

use crate::AppState;

pub struct GamePlugin;

impl Plugin for GamePlugin
{
    fn build(&self, app: &mut App) 
    {
        app
        // States
        .add_state::<SimulationState>()
        // On Enter Systems
        .add_system(pause_simulation.in_schedule(OnEnter(AppState::Game)))
        // Events
        .add_event::<GameOver>()
        // Plugins
        .add_plugin(EnemyPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(ScorePlugin)
        .add_plugin(StarPlugin)
        .add_plugin(EnemiesNumberPlugin)
        // Systems
        .add_system(toggle_simulation.run_if(in_state(AppState::Game)))
        // On Exit Systems
        .add_system(resume_simulation.in_schedule(OnExit(AppState::Game)));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState
{
    #[default]
    Running,
    Paused,
}