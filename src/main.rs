pub mod components;
pub mod events;
pub mod resources;
mod systems;

use events::*;
use resources::*;
use systems::*;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>()
        .init_resource::<HighScores>()
        .init_resource::<StarSpawnTimer>()
        .init_resource::<EnemySpawnTimer>()
        .add_event::<GameOver>()
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_enemies)
        .add_startup_system(spawn_stars)
        .add_system(player_movement)
        .add_system(limit_player_movement)
        .add_system(enemies_movement)
        .add_system(change_enemy_direction)
        .add_system(limit_enemy_movement)
        .add_system(enemy_hit_player)
        .add_system(player_hit_star)
        .add_system(update_score)
        .add_system(tick_star_spawn_timer)
        .add_system(spawn_stars_over_time)
        .add_system(tick_enemy_spawn_timer)
        .add_system(spawn_enemies_over_time)
        .add_system(exit_game)
        .add_system(handle_game_over)
        .add_system(udpate_high_scores)
        .run();
}

