use std::ops::DerefMut;

use bevy::{
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
    prelude::*,
};

use crate::{
    Game, GameOutcomes, GameResult, GameState, Player, RootNode, TvScreenActive, TvScreenSystems,
    game_canvas::GameCanvas,
    game_jeanie::{ActiveCheatCode, CheatCode, GameJeanieState},
};

pub(super) fn plugin(app: &mut App) {
    app.add_sub_state::<PungState>()
        .init_resource::<PungScore>()
        .init_resource::<PungGlobals>()
        .add_computed_state::<PungSetupGameState>()
        .add_state_scoped_event::<PungScored>(PungState::Running)
        .add_systems(OnEnter(PungState::SetupGlobals), setup_pung_globals)
        .add_systems(
            OnEnter(PungSetupGameState),
            (
                reset_score,
                spawn_ball,
                spawn_paddles,
                spawn_gutters,
                spawn_scoreboard,
            )
                .after(TvScreenSystems),
        )
        .add_systems(
            Update,
            (
                move_ball,
                handle_player_input,
                detect_scoring,
                move_ai,
                reset_ball.after(detect_scoring),
                update_score.after(detect_scoring),
                update_scoreboard.after(update_score),
                move_paddles.after(handle_player_input),
                project_positions.after(move_ball),
                handle_collisions.after(move_ball),
                check_for_game_over.after(update_scoreboard),
            )
                .run_if(in_state(PungState::Running)),
        )
        .add_systems(OnEnter(PungState::GameOver), game_over)
        .add_systems(
            Update,
            // click_gameover_button
            check_game_over_timer.run_if(in_state(PungState::GameOver)),
        );
}

#[derive(Debug, Hash, Eq, Clone, PartialEq)]
struct PungSetupGameState;

impl ComputedStates for PungSetupGameState {
    type SourceStates = (PungState, Option<TvScreenActive>);

    fn compute((pung_state, tv_screen_active_state): Self::SourceStates) -> Option<Self> {
        match (pung_state, tv_screen_active_state) {
            (PungState::Running, Some(_)) => Some(Self),
            _ => None,
        }
    }
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
#[allow(non_snake_case)]
pub struct PungGlobals {
    pub DESCRIPTION: String,
    pub BALL_SPEED: f32,
    pub BALL_SIZE: f32,
    pub PLAYER_HAS_PADDLE: bool,
    pub PLAYER_PADDLE_IS_DYNAMIC: bool,
    pub PLAYER_PADDLE_SPEED: f32,
    pub PLAYER_PADDLE_WIDTH: f32,
    pub PLAYER_PADDLE_HEIGHT: f32,
    pub AI_PADDLE_SPEED: f32,
    pub AI_PADDLE_WIDTH: f32,
    pub AI_PADDLE_HEIGHT: f32,
    pub GUTTER_HEIGHT: f32,
    pub MAX_SCORE: u32,
}

#[derive(SubStates, Default, Clone, Eq, PartialEq, Debug, Hash)]
#[source(GameState = GameState::Playing(Game::Pung))]
#[states(scoped_entities)]
pub(crate) enum PungState {
    #[default]
    SetupGlobals,
    Running,
    GameOver,
}

#[derive(Component)]
struct PlayerScore;

#[derive(Component)]
struct AiScore;

#[derive(Resource, Default, Debug)]
struct PungScore {
    player: u32,
    ai: u32,
    result: Option<GameResult>,
}

enum Scorer {
    Ai,
    Player,
}

#[derive(Event)]
struct PungScored(Scorer);

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Paddle;

#[derive(Component)]
#[require(Position, Shape)]
struct Gutter;

#[derive(Component, Default)]
struct Position(Vec2);

#[derive(Component, Default)]
struct Velocity(Vec2);

#[derive(Component, Default)]
struct Shape(Vec2);

#[derive(Component)]
struct Ai;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

fn setup_pung_globals(
    active_cheat_code: Res<ActiveCheatCode>,
    mut pung_globals: ResMut<PungGlobals>,
    mut next_state: ResMut<NextState<PungState>>,
    game_jeanie_state: Res<State<GameJeanieState>>,
) -> Result {
    info!("setup_pung_globals");

    match **game_jeanie_state {
        GameJeanieState::Inactive => {
            *pung_globals.deref_mut() = get_pung_globals(&CheatCode::DEFAULT);
        }
        GameJeanieState::Active => {
            let game = active_cheat_code
                .game
                .as_ref()
                .ok_or("No game set in active cheat code")?;

            if *game != Game::Pung {
                return Err(format!("active cheat code game set to {:?}", *game).into());
            }

            *pung_globals.deref_mut() =
                if let Some(cheat_code) = active_cheat_code.cheat_code.as_ref() {
                    get_pung_globals(cheat_code)
                } else {
                    get_pung_globals(&CheatCode::DEFAULT)
                };
        }
    }

    #[cfg(debug_assertions)]
    {
        //  *pung_globals.deref_mut() = get_pung_globals(&CheatCode::EUOHAKBF);
        // *pung_globals.deref_mut() = get_pung_globals(&CheatCode::MXWLYTFM);
    }

    info!("PungGlobals: {:#?}", pung_globals);

    next_state.set(PungState::Running);

    Ok(())
}

fn get_pung_globals(cheat_code: &CheatCode) -> PungGlobals {
    match cheat_code {
        CheatCode::MXWLYTFM => PungGlobals {
            DESCRIPTION: "Huge ball, player paddle is dynamic".into(),
            BALL_SPEED: 4.,
            BALL_SIZE: 100.,
            GUTTER_HEIGHT: 96.,
            MAX_SCORE: 10,
            PLAYER_HAS_PADDLE: true,
            PLAYER_PADDLE_IS_DYNAMIC: true,
            PLAYER_PADDLE_SPEED: 4.0,
            PLAYER_PADDLE_WIDTH: 10.,
            PLAYER_PADDLE_HEIGHT: 50.,
            AI_PADDLE_SPEED: 4.0,
            AI_PADDLE_WIDTH: 10.,
            AI_PADDLE_HEIGHT: 50.,
        },
        CheatCode::EUOHAKBF => PungGlobals {
            DESCRIPTION: "Huge player paddle, slow AI, tiny AI paddle".into(),
            BALL_SPEED: 4.,
            BALL_SIZE: 5.,
            GUTTER_HEIGHT: 96.,
            MAX_SCORE: 10,
            PLAYER_HAS_PADDLE: true,
            PLAYER_PADDLE_IS_DYNAMIC: false,
            PLAYER_PADDLE_SPEED: 4.0,
            PLAYER_PADDLE_WIDTH: 10.,
            PLAYER_PADDLE_HEIGHT: 275.,
            AI_PADDLE_SPEED: 1.0,
            AI_PADDLE_WIDTH: 10.,
            AI_PADDLE_HEIGHT: 10.,
        },
        _ => PungGlobals {
            DESCRIPTION: "Default".into(),
            BALL_SPEED: 20.,
            BALL_SIZE: 5.,
            GUTTER_HEIGHT: 96.,
            MAX_SCORE: 10,
            PLAYER_HAS_PADDLE: true,
            PLAYER_PADDLE_IS_DYNAMIC: false,
            PLAYER_PADDLE_SPEED: 4.0,
            PLAYER_PADDLE_WIDTH: 10.,
            PLAYER_PADDLE_HEIGHT: 50.,
            AI_PADDLE_SPEED: 20.0,
            AI_PADDLE_WIDTH: 10.,
            AI_PADDLE_HEIGHT: 50.,
        },
    }
}

fn reset_score(mut score: ResMut<PungScore>) {
    *score = PungScore::default();
}

fn move_ai(
    mut ai: Query<(&mut Velocity, &Position), With<Ai>>,
    ball: Query<&Position, With<Ball>>,
) {
    if let Ok((mut velocity, position)) = ai.single_mut()
        && let Ok(ball_position) = ball.single()
    {
        let a_to_b = ball_position.0 - position.0;
        velocity.0.y = a_to_b.y.signum();
    }
}

fn update_scoreboard(
    mut player_score: Query<&mut Text, With<PlayerScore>>,
    mut ai_score: Query<&mut Text, (With<AiScore>, Without<PlayerScore>)>,
    score: Res<PungScore>,
) {
    if score.is_changed() {
        if let Ok(mut player_score) = player_score.single_mut() {
            player_score.0 = score.player.to_string();
        }

        if let Ok(mut ai_score) = ai_score.single_mut() {
            ai_score.0 = score.ai.to_string();
        }
    }
}

fn spawn_scoreboard(mut commands: Commands, root_node: Single<Entity, With<RootNode>>) {
    let font_size = 24.0;

    commands.spawn((
        PlayerScore,
        Text::new("0"),
        TextFont {
            font_size,
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            right: Val::Px(15.0),
            ..default()
        },
        ChildOf(*root_node),
        StateScoped(GameState::Playing(Game::Pung)),
    ));

    commands.spawn((
        AiScore,
        Text::new("0"),
        TextFont {
            font_size,
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(15.0),
            ..default()
        },
        ChildOf(*root_node),
        StateScoped(GameState::Playing(Game::Pung)),
    ));
}

fn update_score(mut score: ResMut<PungScore>, mut events: EventReader<PungScored>) {
    for event in events.read() {
        match event.0 {
            Scorer::Ai => score.ai += 1,
            Scorer::Player => score.player += 1,
        }
    }
}

fn detect_scoring(
    mut ball: Query<&mut Position, With<Ball>>,
    mut events: EventWriter<PungScored>,
    canvas: Single<&GameCanvas>,
) {
    let width = canvas.width();

    if let Ok(ball) = ball.single_mut() {
        if ball.0.x > width / 2. {
            events.write(PungScored(Scorer::Ai));
        } else if ball.0.x < -width / 2. {
            events.write(PungScored(Scorer::Player));
        }
    }
}

fn reset_ball(
    mut ball: Query<(&mut Position, &mut Velocity), With<Ball>>,
    mut events: EventReader<PungScored>,
) {
    for event in events.read() {
        if let Ok((mut position, mut velocity)) = ball.single_mut() {
            match event.0 {
                Scorer::Ai => {
                    position.0 = Vec2::new(0., 0.);
                    velocity.0 = Vec2::new(-1., 1.);
                }
                Scorer::Player => {
                    position.0 = Vec2::new(0., 0.);
                    velocity.0 = Vec2::new(1., 1.);
                }
            }
        }
    }
}

fn handle_player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut paddle: Query<&mut Velocity, With<Player>>,
) {
    if let Ok(mut velocity) = paddle.single_mut() {
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            velocity.0.y = 1.;
        } else if keyboard_input.pressed(KeyCode::ArrowDown) {
            velocity.0.y = -1.;
        } else {
            velocity.0.y = 0.;
        }
    }
}

fn spawn_gutters(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Single<(Entity, &GameCanvas)>,
    pung_globals: Res<PungGlobals>,
) {
    let (canvas_entity, canvas) = *query;

    let width = canvas.width();
    let height = canvas.height();

    let top_gutter_y = height / 2. - pung_globals.GUTTER_HEIGHT / 2.;
    let bottom_gutter_y = -height / 2. + pung_globals.GUTTER_HEIGHT / 2.;

    let shape = Rectangle::from_size(Vec2::new(width, pung_globals.GUTTER_HEIGHT));
    let color = Color::srgb(0., 0., 0.);

    let mesh_handle = meshes.add(shape);
    let material_handle = materials.add(color);

    commands.spawn((
        Gutter,
        Shape(shape.size()),
        Position(Vec2::new(0., top_gutter_y)),
        Mesh2d(mesh_handle.clone()),
        MeshMaterial2d(material_handle.clone()),
        StateScoped(GameState::Playing(Game::Pung)),
        ChildOf(canvas_entity),
    ));

    commands.spawn((
        Gutter,
        Shape(shape.size()),
        Position(Vec2::new(0., bottom_gutter_y)),
        Mesh2d(mesh_handle.clone()),
        MeshMaterial2d(material_handle.clone()),
        StateScoped(GameState::Playing(Game::Pung)),
        ChildOf(canvas_entity),
    ));
}

fn project_positions(mut positionables: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in &mut positionables {
        transform.translation = position.0.extend(0.);
    }
}

fn move_ball(
    mut ball: Query<(&mut Position, &Velocity), With<Ball>>,
    pung_globals: Res<PungGlobals>,
) {
    if let Ok((mut position, velocity)) = ball.single_mut() {
        position.0 += velocity.0 * pung_globals.BALL_SPEED;
    }
}

fn move_paddles(
    player_paddle: Option<Single<(&mut Position, &Velocity), (With<Paddle>, With<Player>)>>,
    ai_paddle: Single<(&mut Position, &Velocity), (With<Paddle>, With<Ai>, Without<Player>)>,
    canvas: Single<&GameCanvas>,
    pung_globals: Res<PungGlobals>,
) {
    let height = canvas.height();

    // Player Paddle
    if let Some(player_paddle) = player_paddle {
        let (position, velocity) = &mut player_paddle.into_inner();
        let new_position = position.0 + velocity.0 * pung_globals.PLAYER_PADDLE_SPEED;
        if new_position.y.abs()
            < height / 2. - pung_globals.GUTTER_HEIGHT - pung_globals.PLAYER_PADDLE_HEIGHT / 2.
        {
            position.0 = new_position;
        }
    }

    // AI Paddle
    let (position, velocity) = &mut ai_paddle.into_inner();
    let new_position = position.0 + velocity.0 * pung_globals.AI_PADDLE_SPEED;
    if new_position.y.abs()
        < height / 2. - pung_globals.GUTTER_HEIGHT - pung_globals.AI_PADDLE_HEIGHT / 2.
    {
        position.0 = new_position;
    }
}

// Returns `Some` if `ball` collides with `wall`. The returned `Collision` is the
// side of `wall` that `ball` hit.
fn collide_with_side(ball: BoundingCircle, wall: Aabb2d) -> Option<Collision> {
    if !ball.intersects(&wall) {
        return None;
    }

    let closest = wall.closest_point(ball.center());
    let offset = ball.center() - closest;

    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0. {
            Collision::Left
        } else {
            Collision::Right
        }
    } else if offset.y > 0. {
        Collision::Top
    } else {
        Collision::Bottom
    };

    Some(side)
}

fn handle_collisions(
    mut ball: Query<(&mut Velocity, &Position, &Shape), With<Ball>>,
    mut other_things: Query<
        (&Position, &Shape, Option<&mut Velocity>, Option<&Player>),
        Without<Ball>,
    >,
    pung_globals: Res<PungGlobals>,
) {
    if let Ok((mut ball_velocity, ball_position, ball_shape)) = ball.single_mut() {
        for (position, shape, velocity, player) in &mut other_things {
            let circle = Circle {
                radius: ball_shape.0.x,
            };
            if let Some(collision) = collide_with_side(
                BoundingCircle::new(ball_position.0, circle.radius),
                Aabb2d::new(position.0, shape.0 / 2.0),
            ) {
                info!("collision!");

                match collision {
                    Collision::Left => {
                        ball_velocity.0.x *= -1.;
                    }
                    Collision::Right => {
                        ball_velocity.0.x *= -1.;
                    }
                    Collision::Top => {
                        ball_velocity.0.y *= -1.;
                    }
                    Collision::Bottom => {
                        ball_velocity.0.y *= -1.;
                    }
                }

                // Handle collison with player paddle if it is dynamic
                if player.is_some()
                    && let Some(mut velocity) = velocity
                    && pung_globals.PLAYER_PADDLE_IS_DYNAMIC
                {
                    velocity.0.x = ball_velocity.0.x.abs();
                }
            }
        }
    }
}

fn spawn_paddles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Single<(Entity, &GameCanvas)>,
    pung_globals: Res<PungGlobals>,
) {
    println!("Spawning paddles...");

    let (canvas_entity, canvas) = *query;

    let width = canvas.width();
    let padding = 50.;
    let right_paddle_x = width / 2. - padding;
    let left_paddle_x = -width / 2. + padding;

    let player_shape = Rectangle::new(
        pung_globals.PLAYER_PADDLE_WIDTH,
        pung_globals.PLAYER_PADDLE_HEIGHT,
    );
    let player_mesh = meshes.add(player_shape);
    let player_color = materials.add(Color::srgb(0., 1., 0.));

    let ai_shape = Rectangle::new(pung_globals.AI_PADDLE_WIDTH, pung_globals.AI_PADDLE_HEIGHT);
    let ai_mesh = meshes.add(ai_shape);
    let ai_color = materials.add(Color::srgb(0., 0., 1.));

    if pung_globals.PLAYER_HAS_PADDLE {
        commands.spawn((
            Player,
            Paddle,
            Velocity::default(),
            Shape(player_shape.size()),
            Position(Vec2::new(right_paddle_x, 0.)),
            Mesh2d(player_mesh.clone()),
            MeshMaterial2d(player_color.clone()),
            StateScoped(GameState::Playing(Game::Pung)),
            ChildOf(canvas_entity),
        ));
    }

    commands.spawn((
        Ai,
        Paddle,
        Velocity::default(),
        Shape(ai_shape.size()),
        Position(Vec2::new(left_paddle_x, 0.)),
        Mesh2d(ai_mesh.clone()),
        MeshMaterial2d(ai_color.clone()),
        StateScoped(GameState::Playing(Game::Pung)),
        ChildOf(canvas_entity),
    ));
}

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    canvas_entity: Option<Single<Entity, With<GameCanvas>>>,
    pung_globals: Res<PungGlobals>,
) {
    println!("Spawning ball...");

    if let Some(canvas_entity) = canvas_entity {
        info!("canvas found");

        let shape = Circle::new(pung_globals.BALL_SIZE);
        let color = Color::srgb(1., 0., 0.);

        // `Assets::add` will load these into memory and return a `Handle` (an ID)
        // to these assets. When all references to this `Handle` are cleaned up
        // the asset is cleaned up.
        let mesh = meshes.add(shape);
        let material = materials.add(color);

        // Here we are using `spawn` instead of `spawn_empty` followed by an
        // `insert`. They mean the same thing, letting us spawn many components on a
        // new entity at once.
        commands.spawn((
            Ball,
            Position::default(),
            Velocity(Vec2::new(-1., 1.)),
            Shape(Vec2::splat(pung_globals.BALL_SIZE)),
            Mesh2d(mesh),
            MeshMaterial2d(material),
            StateScoped(GameState::Playing(Game::Pung)),
            ChildOf(*canvas_entity),
        ));
    } else {
        info!("canvas not found");
    }
}

fn check_for_game_over(
    mut score: ResMut<PungScore>,
    mut next_state: ResMut<NextState<PungState>>,
    pung_globals: Res<PungGlobals>,
) {
    if score.result.is_some() {
        panic!("Game result should not exist yet");
    }

    if score.ai >= pung_globals.MAX_SCORE {
        score.result = Some(GameResult::Lose);
        next_state.set(PungState::GameOver);
    }

    if score.player >= pung_globals.MAX_SCORE {
        score.result = Some(GameResult::Win);
        next_state.set(PungState::GameOver);
    }
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
struct GameOverTimer(Timer);

fn game_over(
    mut commands: Commands,
    score: Res<PungScore>,
    root_node: Single<Entity, With<RootNode>>,
    mut outcomes: ResMut<GameOutcomes>,
) {
    match score.result {
        Some(GameResult::Win) => outcomes.pung.wins += 1,
        Some(GameResult::Lose) => outcomes.pung.losses += 1,
        None => {}
    }

    let message_text = Text::new(match score.result {
        Some(GameResult::Win) => "You win!",
        Some(GameResult::Lose) => "You lose!",
        None => panic!("GameResult should be some."),
    });

    commands.spawn((
        StateScoped(PungState::GameOver),
        ChildOf(*root_node),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![message_text],
    ));

    commands.insert_resource(GameOverTimer(Timer::from_seconds(3.0, TimerMode::Once)));
}

fn check_game_over_timer(
    mut timer: ResMut<GameOverTimer>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
) {
    timer.0.tick(time.delta());

    if timer.0.finished() {
        next_state.set(GameState::ChooseGame);
        commands.remove_resource::<GameOverTimer>();
    }
}
