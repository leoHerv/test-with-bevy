use bevy::prelude::*;

pub const GAME_STYLE : Style = Style 
{ 
    flex_direction: FlexDirection::Row,
    justify_content: JustifyContent::SpaceBetween,
    align_items: AlignItems::Center,
    size: Size::new(Val::Percent(100.0), Val::Px(100.0)),
    padding: UiRect {
                left: Val::Percent(3.),
                right: Val::Percent(3.),
                top: Val::Percent(0.),
                bottom: Val::Percent(0.)
    },
    //gap: Size::new(Val::Px(8.0), Val::Px(8.0)),
    ..Style::DEFAULT
};

pub const NODE_STYLE : Style = Style 
{ 
    flex_direction: FlexDirection::Row,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Percent(15.0), Val::Px(100.0)),
    ..Style::DEFAULT
};

/*
pub const LABEL_STYLE : Style = Style
{
    flex_direction: FlexDirection::Row,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Px(200.0), Val::Px(120.0)),
    ..Style::DEFAULT
};*/

pub fn get_label_text_style(asset_server: &Res<AssetServer>) -> TextStyle
{
    TextStyle { 
        font: asset_server.load("fonts/FiraSans-Bold.ttf"), 
        font_size: 32.0, 
        color: Color::WHITE
    }
}