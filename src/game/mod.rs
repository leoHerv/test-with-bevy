use bevy::prelude::*;

pub mod enemy;
pub mod player;
pub mod score;
pub mod star;
pub mod systems;

use crate::events::*;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::*;

use crate::AppState;

pub struct GamePlugin;

impl Plugin for GamePlugin
{
    fn build(&self, app: &mut App) 
    {
        app
        // States
        .add_state::<SimulationState>()
        // Events
        .add_event::<GameOver>()
        // Plugins
        .add_plugin(EnemyPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(ScorePlugin)
        .add_plugin(StarPlugin)
        // Systems
        .add_system(toggle_simulation.run_if(in_state(AppState::Game)));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState
{
    Running,
    #[default]
    Paused,
}