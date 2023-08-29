use bevy::prelude::*;

use crate::ui::game_game_over::components::*;

use crate::events::GameOver;


pub fn update_final_score_label(
    mut game_over_event_reader: EventReader<GameOver>,
    mut final_score_label_query: Query<&mut Text, With<ScoreLabel>>, 
) {
    for event in game_over_event_reader.iter() 
    {
        println!("score event {}", event.score);
        for mut text in final_score_label_query.iter_mut() 
        {
            text.sections[0].value = event.score.to_string();
            println!("score {}", event.score);
        }
    }
}

pub fn update_best_score_label(
    mut game_over_event_reader: EventReader<GameOver>,
    mut best_score_label_query: Query<&mut Text, With<BestScoreLabel>>,
) {
    for event in game_over_event_reader.iter() 
    {
        println!("best event {}", event.best_score);
        for mut text in best_score_label_query.iter_mut() 
        {
            text.sections[0].value = event.best_score.to_string();
            println!("best {}", event.best_score);
        }
    }
}