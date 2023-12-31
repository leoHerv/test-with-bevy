use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.35, 0.75, 0.35);
pub const NODE_COLOR: Color = Color::rgba(0.25, 0.25, 0.25, 0.50);

pub const GAME_PAUSE_STYLE : Style = Style 
{ 
    position_type: PositionType::Absolute,
    flex_direction: FlexDirection::Column,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Percent(50.0), Val::Percent(60.0)),
    margin: UiRect {
        left: Val::Percent(25.),
        right: Val::Percent(25.),
        top: Val::Percent(12.),
        bottom: Val::Percent(0.)
},
    gap: Size::new(Val::Px(8.0), Val::Px(8.0)),
    ..Style::DEFAULT
};

pub const BUTTON_STYLE: Style = Style
{
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Px(200.0), Val::Px(80.0)),
    ..Style::DEFAULT
};

pub const TITLE_STYLE : Style = Style
{
    flex_direction: FlexDirection::Row,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Px(300.0), Val::Px(120.0)),
    ..Style::DEFAULT
};

pub fn get_button_text_style(asset_server: &Res<AssetServer>) -> TextStyle
{
    TextStyle { 
        font: asset_server.load("fonts/FiraSans-Bold.ttf"), 
        font_size: 32.0, 
        color: Color::WHITE
    }
}

pub fn get_title_text_style(asset_server: &Res<AssetServer>) -> TextStyle
{
    TextStyle { 
        font: asset_server.load("fonts/FiraSans-Bold.ttf"), 
        font_size: 64.0, 
        color: Color::WHITE
    }
}