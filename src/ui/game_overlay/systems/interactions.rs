use bevy::prelude::*;

use crate::ui::game_overlay::components::*;

use crate::game::score::resources::*;
use crate::game::enemies_number::resources::*;

pub fn update_score_text(
    mut text_query: Query<&mut Text, With<ScoreLabel>>, 
    score: Res<Score>
) 
{
    if score.is_changed() 
    {
        for mut text in text_query.iter_mut() 
        {
            text.sections[0].value = score.value.to_string();
        }
    }
}

pub fn update_enemies_number_text(
    mut text_query: Query<&mut Text, With<EnemiesNumberLabel>>, 
    score: Res<EnemiesNumber>
) 
{
    if score.is_changed() 
    {
        for mut text in text_query.iter_mut() 
        {
            text.sections[0].value = score.value.to_string();
        }
    }
}