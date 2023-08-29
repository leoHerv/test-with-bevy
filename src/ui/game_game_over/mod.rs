use bevy::prelude::*;

use crate::*;

use systems::layout::*;
use systems::interactions::*;
use systems::updates::*;

mod components;
mod styles;
mod systems;


pub struct GameGameOverPlugin;

impl Plugin for GameGameOverPlugin
{
    fn build(&self, app: &mut App) {
        app
        // On Enter State System
        .add_system(spawn_game_game_over.in_schedule(OnEnter(AppState::GameOver)))
        // Systems
        .add_systems(
            (
                interact_with_restart_button,
                interact_with_main_menu_button,
                interact_with_quit_button,
                update_final_score_label.after(spawn_game_game_over),
                update_best_score_label.after(spawn_game_game_over),
            ).in_set(OnUpdate(AppState::GameOver)),
        )
        // On Exit Statet System
        .add_system(despawn_game_game_over.in_schedule(OnExit(AppState::GameOver)));
    }
}

