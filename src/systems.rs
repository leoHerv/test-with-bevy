use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::app::AppExit;

use crate::events::*;
use crate::AppState;
use crate::game::SimulationState;

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

pub fn transition_to_game_state(
    //mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
)
{
    if keyboard_input.just_pressed(KeyCode::G) || keyboard_input.just_pressed(KeyCode::Space)
    {
        if app_state.0 != AppState::Game{
            //commands.insert_resource(NextState(Some(AppState::Game)));
            // Better
            next_app_state.set(AppState::Game);
            println!("Entered AppState::Game");
        }
    }
}

pub fn transition_to_main_menu_state(
    //mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
)
{
    if keyboard_input.just_pressed(KeyCode::M)
    {
        if app_state.0 != AppState::MainMenu{
            //commands.insert_resource(NextState(Some(AppState::MainMenu)));
            //commands.insert_resource(NextState(Some(SimulationState::Paused)));
            // Better
            next_app_state.set(AppState::MainMenu);
            next_simulation_state.set(SimulationState::Running);
            println!("Entered AppState::MainMenu");
        }
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
    //mut commands: Commands,
    mut game_over_event_reader: EventReader<GameOver>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
)
{
    for event in game_over_event_reader.iter()
    {
        println!("------Your final score is: {}", event.score.to_string());
        //commands.insert_resource(NextState(Some(AppState::GameOver)));
        //commands.insert_resource(NextState(Some(SimulationState::Paused)));
        // Better
        next_app_state.set(AppState::GameOver);
        next_simulation_state.set(SimulationState::Running);
    }   
}