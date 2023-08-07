pub mod events;
mod systems;
mod game;
mod main_menu;

use game::GamePlugin;
use main_menu::MainMenuPlugin;

use systems::*;

use bevy::prelude::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(GamePlugin)
    .add_plugin(MainMenuPlugin)
    .add_startup_system(spawn_camera)
    .add_system(exit_game)
    .add_system(handle_game_over)
    .run();
}

