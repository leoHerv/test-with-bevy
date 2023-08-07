use bevy::prelude::*;

pub mod resources;
mod systems;

use resources::*;
use systems::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin
{

    fn build(&self, app: &mut App) 
    {
        app 
        .init_resource::<Score>()
        .init_resource::<HighScores>()
        .add_system(udpate_high_scores)
        .add_system(update_score);
    }
}