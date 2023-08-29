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
#[derive(Resource, Debug)]
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

impl HighScores
{
    pub fn sort(&mut self){
        self.scores.sort_by(|a, b| b.1.cmp(&a.1));
    }

    pub fn get_best_score(&mut self) -> u32
    {
        self.sort();
        if self.scores.len() != 0
        {
            return self.scores[0].1
        }
        return 0;
    }
}