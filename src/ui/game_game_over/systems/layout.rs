use bevy::prelude::*;

use crate::ui::game_game_over::components::*;
use crate::ui::game_game_over::styles::*;

pub fn spawn_game_game_over(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) 
{
    //let game_game_over_entity = build_game_game_over(&mut commands, &asset_server);
    build_game_game_over(&mut commands, &asset_server);
}

pub fn despawn_game_game_over(
    mut commands: Commands,
    game_pause_query: Query<Entity, With<GameGameOver>>,
) 
{   
    if let Ok(game_pause_query) = game_pause_query.get_single()
    {
        commands.entity(game_pause_query).despawn_recursive();
    }
}

pub fn build_game_game_over(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) -> Entity
{
    let game_game_over_entity = commands.spawn(
        (NodeBundle
        {
            style: GAME_GAME_OVER_STYLE,
            background_color: NODE_COLOR.into(),
            z_index: ZIndex::Local(1),
            ..default()
        },
        GameGameOver{},
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
                            "Game Over", 
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

    parent.spawn(
        
        NodeBundle
        {
            style: TEXT_STYLE,
            ..default()
        }
    )
    .with_children(| parent| 
    {
        // Text for the previous game score
        parent.spawn(
            TextBundle
            {
                text: Text
                {
                    sections: vec![
                        TextSection::new(
                            "Score : ", 
                            get_button_text_style(&asset_server)
                        )
                    ],
                    alignment: TextAlignment::Center,
                    ..default()
                },
                ..default()
            },
        );

        parent.spawn(
            (
            TextBundle
            {
                text: Text
                {
                    sections: vec![
                        TextSection::new(
                            "0", 
                            get_button_text_style(&asset_server)
                        )
                    ],
                    alignment: TextAlignment::Center,
                    ..default()
                },
                ..default()
            },
            ScoreLabel{}
        )
        );
    }
    );

    parent.spawn(
        
        NodeBundle
        {
            style: TEXT_STYLE,
            ..default()
        }
    )
    .with_children(| parent| 
    {
        // Text for the best score
        parent.spawn(
            TextBundle
            {
                text: Text
                {
                    sections: vec![
                        TextSection::new(
                            "Best Score : ", 
                            get_button_text_style(&asset_server)
                        )
                    ],
                    alignment: TextAlignment::Center,
                    ..default()
                },
                ..default()
            }
        );

        parent.spawn(
            (
            TextBundle
            {
                text: Text
                {
                    sections: vec![
                        TextSection::new(
                            "0", 
                            get_button_text_style(&asset_server)
                        )
                    ],
                    alignment: TextAlignment::Center,
                    ..default()
                },
                ..default()
            },
            BestScoreLabel{}
        )
        );
    }
    );

        // === Restart Button ===
        parent.spawn(
            (
                ButtonBundle 
                {
                    style: BUTTON_STYLE,
                    background_color: NORMAL_BUTTON_COLOR.into(),
                    ..default()
                },
                RestartButton {},
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
                                    "Restart", 
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

    game_game_over_entity
}

