use bevy::prelude::*;

use crate::ui::game_overlay::components::*;

use crate::ui::game_overlay::styles::*;

pub fn spawn_game_overlay(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) 
{
    build_game_overlay(&mut commands, &asset_server);
}

pub fn despawn_game_overlay(
    mut commands: Commands,
    main_menu_query: Query<Entity, With<GameOverlay>>,
) 
{   
    if let Ok(game_overlay_entity) = main_menu_query.get_single()
    {
        commands.entity(game_overlay_entity).despawn_recursive();
    }
}

pub fn build_game_overlay(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) -> Entity
{
    // Main
    let game_overlay_entity = commands.spawn(
        (NodeBundle
        {
            style: GAME_STYLE,
            //background_color: Color::RED.into(),
            ..default()
        },
        GameOverlay{},
    )
    )
    .with_children(|parent| 
    {
        // === Score ===
        parent.spawn(
            NodeBundle
            {
                style: NODE_STYLE,
                background_color: Color::BLUE.into(),
                ..default()
            },
        ).with_children(|parent| 
        {
            parent.spawn(
                TextBundle
                {
                    text: Text
                    {
                        sections: vec![
                            TextSection::new(
                                "Score : ", 
                                get_label_text_style(&asset_server)
                            )
                        ],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    background_color: Color::BLACK.into(),
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
                                get_label_text_style(&asset_server)
                            )
                        ],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    background_color: Color::BLACK.into(),
                    ..default()
                },
                ScoreLabel {},
            )
            );
        }
        );

        // === Enemies Number ===
        parent.spawn(
            NodeBundle
            {
                style: NODE_STYLE,
                background_color: Color::BLUE.into(),
                ..default()
            },
        ).with_children(|parent| 
        {
            parent.spawn(
                TextBundle
                {
                    text: Text
                    {
                        sections: vec![
                            TextSection::new(
                                "Enemies : ", 
                                get_label_text_style(&asset_server)
                            )
                        ],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    background_color: Color::BLACK.into(),
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
                                "4", 
                                get_label_text_style(&asset_server)
                            )
                        ],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    background_color: Color::BLACK.into(),
                    ..default()
                },
                EnemiesNumberLabel {},
            )
            );
        }
        );

    })
    .id();

    game_overlay_entity
}
