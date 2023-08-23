use bevy::prelude::*;

pub const NODE_COLOR: Color = Color::rgba(0.25, 0.25, 0.25, 0.50);

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
    size: Size::new(Val::Percent(10.0), Val::Px(80.0)),
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

pub const IMAGE_STYLE : Style = Style
{
    size: Size::new(Val::Px(32.0), Val::Px(32.0)),
    margin: UiRect::new(
                Val::Px(8.0), 
                Val::Px(8.0), 
                Val::Px(8.0),
                Val::Px(8.0),
    ),
    ..Style::DEFAULT
};