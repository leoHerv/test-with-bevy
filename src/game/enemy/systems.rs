use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

//use crate::enemy::components::*;
use super::components::*;
use super::resources::*;
use super::*;
use crate::game::player::*;

// System : to spawn a enemies in the game
pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
)
{
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES
    {
        let half_enemy_size = ENEMY_SIZE / 2.0;
        
        // rand => value between 0 and 1
        let random_x = random::<f32>() * (window.width() - ENEMY_SIZE) + half_enemy_size; 
        let random_y = random::<f32>() * (window.height() - ENEMY_SIZE) + half_enemy_size;

        let enemy_x = if random::<f32>() < 0.5
        {
            random::<f32>()
        }
        else
        {
            random::<f32>() * -1.0
        };

        let enemy_y = if random::<f32>() < 0.5
        {
            random::<f32>()
        }
        else
        {
            random::<f32>() * -1.0
        };

        commands.spawn(
            (
                SpriteBundle
                {
                    transform: Transform::from_xyz(random_x, random_y, 0.0),
                    texture: asset_server.load("sprites/ball_red_large.png"),
                    ..default()
                },
                Enemy{
                    direction: Vec2::new(enemy_x, enemy_y).normalize()
                },
        ));
    }
}

// System : to despawn enemies
pub fn despawn_enemies(
    mut commands: Commands,
    enemy_query: Query<Entity, With<Enemy>>,
)
{
    for enemy_entity in enemy_query.iter()
    {
        commands.entity(enemy_entity).despawn();
    }
}

// System : to limit that allow enemies to move
pub fn enemies_movement(
    mut enemy_query: Query<(&mut Transform, &Enemy)>,
    time: Res<Time>
)
{
    for (mut transform, enemy) in enemy_query.iter_mut()
    {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);

        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    } 
}

// System : redirect a enemy when is to close of a border
pub fn change_enemy_direction(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
)
{
    let window = window_query.get_single().unwrap();

    let half_enemy_size = PLAYER_SIZE / 2.0;

    let x_min = 0.0 + half_enemy_size;
    let y_min = 0.0 + half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    for (transform, mut enemy) in enemy_query.iter_mut()
    {
        let mut change_direction : bool = false;
        let translation = transform.translation;

        if translation.x < x_min || translation.x > x_max
        {
            enemy.direction.x *= -1.0;
            change_direction = true;
        }
        if translation.y < y_min || translation.y > y_max
        {
            enemy.direction.y *= -1.0;
            change_direction = true;
        }

        if change_direction
        {
            let sound_effect_1 = asset_server.load("audio/pluck_001.ogg");
            let sound_effect_2 = asset_server.load("audio/pluck_002.ogg");

            let sound_effect = if random::<f32>() > 0.5
            {
                sound_effect_1
            }
            else
            {
                sound_effect_2
            };

            let playback_settings = PlaybackSettings {
                repeat: false,
                volume: 0.01,
                speed: 1.0,
            };
            audio.play_with_settings(sound_effect.clone(), playback_settings);

            //audio.play(sound_effect);
        }
    }
}

// System : to limit enemy movement
pub fn limit_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
)
{
    let window = window_query.get_single().unwrap();
    
    let half_enemy_size = ENEMY_SIZE / 2.0;

    let x_min = 0.0 + half_enemy_size;
    let y_min = 0.0 + half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    for mut tranform in enemy_query.iter_mut()
    {
        let mut translation = tranform.translation;

        // Limit for the X axe
        if translation.x < x_min
        {
            translation.x = x_min;
        }
        else if translation.x > x_max
        {
            translation.x = x_max;
        }

        // Limit for the Y axe
        if translation.y < y_min
        {
            translation.y = y_min;
        }
        else if translation.y > y_max
        {
            translation.y = y_max;
        }

        tranform.translation = translation;
    }
}

pub fn tick_enemy_spawn_timer(
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
    time: Res<Time>,
)
{
    enemy_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_enemies_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: Res<EnemySpawnTimer>,
)
{
    if enemy_spawn_timer.timer.finished()
    {
        let window = window_query.get_single().unwrap();

        let half_enemy_size = ENEMY_SIZE / 2.0;

        let random_x = random::<f32>() * (window.width() - ENEMY_SIZE) + half_enemy_size; 
        let random_y = random::<f32>() * (window.height() - ENEMY_SIZE) + half_enemy_size;

        let enemy_x = if random::<f32>() < 0.5
        {
            random::<f32>()
        }
        else
        {
            random::<f32>() * -1.0
        };

        let enemy_y = if random::<f32>() < 0.5
        {
            random::<f32>()
        }
        else
        {
            random::<f32>() * -1.0
        };

        commands.spawn((
            SpriteBundle{
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy{direction: Vec2::new(enemy_x, enemy_y).normalize()},
        ));
    }
}