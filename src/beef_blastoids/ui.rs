use bevy::prelude::*;

use crate::{
    Game, GameState, RootNode,
    beef_blastoids::{BeefBlastoidsState, Lives, Score},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(BeefBlastoidsState::Running), spawn_scoreboard)
        .add_systems(
            Update,
            update_scoreboard.run_if(in_state(GameState::Playing(Game::BeefBlastoids))),
        );
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct PlayerScore;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct PlayerLives;

fn spawn_scoreboard(
    mut commands: Commands,
    root_node: Single<Entity, With<RootNode>>,
    score: Res<Score>,
    lives: Res<Lives>,
) {
    commands.spawn((
        Name::new("Scoreboard"),
        ChildOf(*root_node),
        StateScoped(GameState::Playing(crate::Game::BeefBlastoids)),
        Node {
            ..Default::default()
        },
        children![
            Text::new("Score: "),
            (PlayerScore, Text::new(format!("{}", score.0)),),
            Text::new("Lives: "),
            (PlayerLives, Text::new(format!("{}", lives.0)),)
        ],
    ));
}

fn update_scoreboard(
    mut score_text: Single<&mut Text, With<PlayerScore>>,
    mut lives_text: Single<&mut Text, (With<PlayerLives>, Without<PlayerScore>)>,
    score: Res<Score>,
    lives: Res<Lives>,
) {
    if score.is_changed() {
        score_text.0 = score.to_string();
    }

    if lives.is_changed() {
        lives_text.0 = lives.to_string();
    }
}
