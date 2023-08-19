use bevy::prelude::*;

use crate::AppState;

use systems::layout::*;
use systems::interactions::*;

mod components;
mod styles;
mod systems;

pub struct GameOverlayPlugin;

impl Plugin for GameOverlayPlugin
{
    fn build(&self, app: &mut App) {
        app
        // On Enter State System
        .add_system(spawn_game_overlay.in_schedule(OnEnter(AppState::Game)))
        // Systems
        .add_systems(
            (
                update_enemies_number_text,
                update_score_text,
            ).in_set(OnUpdate(AppState::Game)),
        )

        // On Exit Statet System
        .add_system(despawn_game_overlay.in_schedule(OnExit(AppState::Game)));
    }
}