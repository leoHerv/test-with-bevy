use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;
use bevy::app::AppExit;

use crate::components::*;
use crate::events::*;
use crate::resources::*;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0; // Sprite size for the player.
pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0; // Sprite size for an enemy.
pub const NUMBER_OF_STARS: usize = 10;
pub const STAR_SIZE: f32 = 30.0; // Sprite size for a star.

// System : to spawn a player in the game
pub fn spawn_player(
    mut commands: Commands, 
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
)
{
    let window: &Window = window_query.get_single().unwrap();

    commands.spawn((
            SpriteBundle{
                transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
                texture: asset_server.load("sprites/ball_blue_large.png"),
                ..default()
            },
            Player{},
    ));
}

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

// System : to spawn stars
pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
)
{
    let window: &Window = window_query.get_single().unwrap();

    let half_star_size = STAR_SIZE / 2.0;

    for _ in 0..NUMBER_OF_STARS
    {
        let random_x = random::<f32>() * (window.width() - STAR_SIZE) + half_star_size; 
        let random_y = random::<f32>() * (window.height() - STAR_SIZE) + half_star_size;

        commands.spawn((
            SpriteBundle{
                transform: Transform::from_xyz(random_x, random_y, -1.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star{},
        ));
    }
    
}

// System : to spawn a camera in the game
pub fn spawn_camera(
    mut commands: Commands, 
    window_query: Query<&Window, With<PrimaryWindow>>,
)
{
    let window: &Window = window_query.get_single().unwrap();

    commands.spawn(
        Camera2dBundle{
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        }
    );
}

// System : to move the player
pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
)
{
    if let Ok(mut transform) = player_query.get_single_mut()
    {
        let mut direction = Vec3::ZERO;

        // Left
        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::Q)
        {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        // Right
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D)
        {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        // Up
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::Z)
        {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        // Down
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S)
        {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0
        {
            direction = direction.normalize();
        } 

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

// System : to limit player movement
pub fn limit_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
)
{
    if let Ok(mut player_transform) = player_query.get_single_mut()
    {
        let window = window_query.get_single().unwrap();
        
        let half_player_size = PLAYER_SIZE / 2.0;

        let x_min = 0.0 + half_player_size;
        let y_min = 0.0 + half_player_size;
        let x_max = window.width() - half_player_size;
        let y_max = window.height() - half_player_size;

        let mut translation = player_transform.translation;

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

        player_transform.translation = translation;

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
            audio.play(sound_effect);
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


pub fn enemy_hit_player(
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<GameOver>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    score: Res<Score>,
)
{
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut()
    {
        for enemy_transform in enemy_query.iter()
        {
            let distance = player_transform.translation.distance(enemy_transform.translation);

            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;

            if distance < player_radius + enemy_radius
            {
                println!("Game Over! You hit a enemy.");
                let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");
                audio.play(sound_effect);
                commands.entity(player_entity).despawn();
                game_over_event_writer.send(GameOver{score: score.value});
            }

        }
    }
}

pub fn player_hit_star(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut score: ResMut<Score>,
)
{
    if let Ok(player_transform) = player_query.get_single()
    {
        for (star_entity, star_transform) in star_query.iter()
        {
            let distance = player_transform.translation.distance(star_transform.translation);

            let player_radius = PLAYER_SIZE / 2.0;
            let star_radius = STAR_SIZE / 2.0;

            if distance < player_radius + star_radius
            {
                println!("You hit a star");
                score.value += 1;
                let sound_effect = asset_server.load("audio/impactSoft_heavy_002.ogg");
                audio.play(sound_effect);
                commands.entity(star_entity).despawn();
            }

        }
    }
}

pub fn update_score(
    score: Res<Score>
)
{
    if score.is_changed()
    {
        println!("Score : {}", score.value.to_string());
    }
}

pub fn tick_star_spawn_timer(
    mut star_spawn_timer: ResMut<StarSpawnTimer>,
    time: Res<Time>,
)
{
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<StarSpawnTimer>,
)
{
    if star_spawn_timer.timer.finished()
    {
        let window = window_query.get_single().unwrap();

        let half_star_size = STAR_SIZE / 2.0;

        let random_x = random::<f32>() * (window.width() - STAR_SIZE) + half_star_size; 
        let random_y = random::<f32>() * (window.height() - STAR_SIZE) + half_star_size;

        commands.spawn((
            SpriteBundle{
                transform: Transform::from_xyz(random_x, random_y, -1.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star{},
        ));
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

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
)
{
    if keyboard_input.pressed(KeyCode::Escape)
    {
        app_exit_event_writer.send(AppExit);
    }
}

pub fn handle_game_over(
    mut game_over_event_reader: EventReader<GameOver>,
)
{
    for event in game_over_event_reader.iter()
    {
        println!("Your final score is: {}", event.score.to_string());
    }
}

pub fn udpate_high_scores(
    mut game_over_event_reader: EventReader<GameOver>,
    mut high_scores: ResMut<HighScores>,
)
{
    for event in game_over_event_reader.iter()
    {
        high_scores.scores.push(("Player".to_string(), event.score));
    }
}