use bevy::prelude::*;

use crate::game::SimulationState;

use systems::layout::*;
use systems::interactions::*;

mod components;
mod styles;
mod systems;


pub struct GamePausePlugin;

impl Plugin for GamePausePlugin
{
    fn build(&self, app: &mut App) {
        app
        // On Enter State System
        .add_system(spawn_game_pause.in_schedule(OnEnter(SimulationState::Paused)))
        
        // Systems
        .add_systems(
            (
                interact_with_resume_button,
                interact_with_main_menu_button,
                interact_with_quit_button,
            ).in_set(OnUpdate(SimulationState::Paused)),
        )
        // On Exit Statet System
        .add_system(despawn_game_pause.in_schedule(OnExit(SimulationState::Paused)));
    }
}

