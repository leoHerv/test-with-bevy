pub mod events;
mod systems;
mod game;
mod ui;

use game::GamePlugin;
use ui::main_menu::MainMenuPlugin;
use ui::game_overlay::GameOverlayPlugin;
use ui::game_pause::GamePausePlugin;
use ui::game_game_over::GameGameOverPlugin;


use systems::*;

use bevy::prelude::*;

fn main() {
    App::new()
    // Bevy Plugins
    .add_plugins(DefaultPlugins.set(
        WindowPlugin 
        {
            primary_window: Some(Window 
                {
                    title: "Bevy Ball Game !".into(),
                    resolution: (1280., 720.).into(),
                    
                    resize_constraints: WindowResizeConstraints
                    {
                        min_width: 1280.,
                        min_height: 720.,
                        max_width: 1280.,
                        max_height: 720.,
                    },
                    
                    decorations: true,
                    resizable: false,
                    ..default()
            }),
            ..default()
        }
    ))

    // States
    .add_state::<AppState>()
    // Plugins
    .add_plugin(GamePlugin)
    .add_plugin(MainMenuPlugin)
    .add_plugin(GameOverlayPlugin)
    .add_plugin(GamePausePlugin)
    .add_plugin(GameGameOverPlugin)
    // Startup Systems
    .add_startup_system(spawn_camera)
    // Systems
    .add_system(transition_to_game_state)
    .add_system(transition_to_main_menu_state)
    .add_system(exit_game)
    .add_system(handle_game_over)
    .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState
{
    #[default]
    MainMenu,
    Game,
    GameOver,
}