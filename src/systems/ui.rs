use bevy::prelude::*;
use crate::components::ScoreText;
use crate::resources::Score;

pub fn update_score_ui(
    score: Res<Score>,
    mut query: Query<&mut Text, With<ScoreText>>,
) {
    if let Ok(mut text) = query.single_mut() {
        text.0 = format!("Score: {}", score.0);
    }
}
