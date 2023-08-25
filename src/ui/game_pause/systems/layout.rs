use bevy::prelude::*;

use crate::ui::game_pause::components::*;

use crate::ui::game_pause::styles::*;

pub fn spawn_game_pause(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) 
{
    //let game_pause_entity = build_game_pause(&mut commands, &asset_server);
    build_game_pause(&mut commands, &asset_server);
}

pub fn despawn_game_pause(
    mut commands: Commands,
    game_pause_query: Query<Entity, With<GamePause>>,
) 
{   
    if let Ok(game_pause_query) = game_pause_query.get_single()
    {
        commands.entity(game_pause_query).despawn_recursive();
    }
}

pub fn build_game_pause(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) -> Entity
{
    let game_pause_entity = commands.spawn(
        (NodeBundle
        {
            style: GAME_PAUSE_STYLE,
            background_color: NODE_COLOR.into(),
            z_index: ZIndex::Local(1),
            ..default()
        },
        GamePause{},
    )
    )
    .with_children(|parent| 
    {
        // === Title ===

    parent.spawn(
        
        NodeBundle
        {
            style: TITLE_STYLE,
            ..default()
        }
    )
    .with_children(| parent| 
    {
        // Text
        parent.spawn(
            TextBundle
            {
                text: Text
                {
                    sections: vec![
                        TextSection::new(
                            "Pause Menu", 
                            get_title_text_style(&asset_server)
                        )
                    ],
                    alignment: TextAlignment::Center,
                    ..default()
                },
                ..default()
            }
        );
    }
    );

        // === Resume Button ===
        parent.spawn(
            (
                ButtonBundle 
                {
                    style: BUTTON_STYLE,
                    background_color: NORMAL_BUTTON_COLOR.into(),
                    ..default()
                },
                ResumeButton {},
            )
        )
        .with_children(|parent| 
            {
                parent.spawn(
                    TextBundle
                    {
                        text: Text
                        {
                            sections: vec![
                                TextSection::new(
                                    "Resume", 
                                    get_button_text_style(&asset_server)
                                )
                            ],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    }
                );
            }
        );

        // === Main Menu Button ===
        parent.spawn(
            (
                ButtonBundle 
                {
                    style: BUTTON_STYLE,
                    background_color: NORMAL_BUTTON_COLOR.into(),
                    ..default()
                },
                MainMenuButton {},
            )
        )
        .with_children(|parent| 
            {
                parent.spawn(
                    TextBundle
                    {
                        text: Text
                        {
                            sections: vec![
                                TextSection::new(
                                    "Main Menu", 
                                    get_button_text_style(&asset_server)
                                )
                            ],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    }
                );
            }
        );

        // === Quit Button ===
        parent.spawn(
            (
                ButtonBundle 
                {
                    style: BUTTON_STYLE,
                    background_color: NORMAL_BUTTON_COLOR.into(),
                    ..default()
                },
                QuitButton {},
            )
        )
        .with_children(|parent| 
            {
                parent.spawn(
                    TextBundle
                    {
                        text: Text
                        {
                            sections: vec![
                                TextSection::new(
                                    "Quit", 
                                    get_button_text_style(&asset_server)
                                )
                            ],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    }
                );
            }
        );

    })
    .id();

    game_pause_entity
}

