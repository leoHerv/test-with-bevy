use bevy::prelude::*;

use super::resources::*;
use crate::events::*;

pub fn update_score(
    score: Res<Score>
)
{
    if score.is_changed()
    {
        println!("Score : {}", score.value.to_string());
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