use bevy::prelude::*;

// To have the current score.
#[derive(Resource)]
pub struct Score 
{
    pub value: u32,
}

impl Default for Score
{
    fn default() -> Self 
    {
        Score {value: 0}
    }
}

// To have the list of the different scores.
#[derive(Resource)]
pub struct HighScores
{
    pub scores: Vec<(String, u32)>,
}

impl Default for HighScores
{
    fn default() -> Self {
        HighScores {scores: Vec::new()}
    }
}