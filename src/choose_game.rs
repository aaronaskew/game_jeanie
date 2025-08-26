use bevy::prelude::*;

use crate::{Game, GameOutcomes, GameState, cut_scenes::CutScene, loading::TextureAssets};

const GLOW_COLOR: Color = Color::srgba(1.0, 1.0, 1.0, 0.68);

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(MeshPickingPlugin)
        .add_sub_state::<ChooseGameState>()
        .add_systems(
            OnEnter(GameState::ChooseGame),
            (
                setup_choose_game_textures,
                setup_clickable_meshes,
                handle_game_outcomes,
            )
                .chain(),
        )
        .add_systems(
            OnEnter(ChooseGameState::ChooseOrContinue),
            setup_choose_or_continue_ui,
        )
        .add_systems(
            Update,
            handle_buttons.run_if(in_state(ChooseGameState::ChooseOrContinue)),
        );
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct PungGlow;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct BeefBlastoidsGlow;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct RacePlaceGlow;

#[derive(SubStates, Default, Clone, Eq, PartialEq, Debug, Hash)]
#[source(GameState = GameState::ChooseGame)]
#[states(scoped_entities)]
pub(crate) enum ChooseGameState {
    #[default]
    /// Choose a game to play
    ChooseGame,
    /// Ask whether to choose a game or continue to story
    ChooseOrContinue,
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct ChooseAnotherGame;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct ContinueOn;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct ChooseOrContinueUI;

fn setup_choose_or_continue_ui(mut commands: Commands) {
    commands.spawn((
        ChooseOrContinueUI,
        StateScoped(GameState::ChooseGame),
        Node {
            display: Display::Flex,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
        children![(
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(50.0),
                ..Default::default()
            },
            children![
                (
                    ChooseAnotherGame,
                    Button,
                    Text::new("Choose another game to play.")
                ),
                (ContinueOn, Button, Text::new("\"These are way too hard!\""))
            ]
        )],
    ));
}

fn handle_buttons(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    interaction_query: Query<
        (
            &Interaction,
            Option<&ChooseAnotherGame>,
            Option<&ContinueOn>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    ui_entity: Single<Entity, With<ChooseOrContinueUI>>,
) {
    for (interaction, choose_another_game, continue_on) in &interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if choose_another_game.is_some() {
                    commands.entity(*ui_entity).despawn();
                }

                if continue_on.is_some() {
                    next_state.set(GameState::CutScene(CutScene::MiddleA));
                }
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

fn setup_choose_game_textures(
    mut commands: Commands,
    texture_assets: Res<TextureAssets>,
    game_outcomes: Res<GameOutcomes>,
) {
    commands.spawn((
        Name::new("Choose Game Background"),
        Sprite {
            image: texture_assets.panel2.clone(),
            custom_size: Some(Vec2::new(1280., 720.)),
            image_mode: SpriteImageMode::Auto,
            ..Default::default()
        },
        Transform::from_xyz(0., 0., -10.0),
        StateScoped(GameState::ChooseGame),
    ));

    commands.spawn((
        Name::new("Pung Glow"),
        PungGlow,
        Sprite {
            image: texture_assets.panel2_pung_glow.clone(),
            custom_size: Some(Vec2::new(1280., 720.)),
            image_mode: SpriteImageMode::Auto,
            color: GLOW_COLOR,
            ..Default::default()
        },
        Visibility::Hidden,
        Transform::from_xyz(0., 0., -1.0),
        StateScoped(GameState::ChooseGame),
    ));

    commands.spawn((
        Name::new("Beef Blastoids Glow"),
        BeefBlastoidsGlow,
        Sprite {
            image: texture_assets.panel2_blastoid_glow.clone(),
            custom_size: Some(Vec2::new(1280., 720.)),
            image_mode: SpriteImageMode::Auto,
            color: GLOW_COLOR,

            ..Default::default()
        },
        Visibility::Hidden,
        Transform::from_xyz(0., 0., -1.0),
        StateScoped(GameState::ChooseGame),
    ));

    commands.spawn((
        Name::new("Race Place Glow"),
        RacePlaceGlow,
        Sprite {
            image: texture_assets.panel2_raceplace_glow.clone(),
            custom_size: Some(Vec2::new(1280., 720.)),
            image_mode: SpriteImageMode::Auto,
            color: GLOW_COLOR,

            ..Default::default()
        },
        Visibility::Hidden,
        Transform::from_xyz(0., 0., -1.0),
        StateScoped(GameState::ChooseGame),
    ));

    commands.spawn((
        Name::new("Pung Seal"),
        Sprite {
            image: texture_assets.panel2_pung_seal.clone(),
            custom_size: Some(Vec2::new(1280., 720.)),
            image_mode: SpriteImageMode::Auto,
            ..Default::default()
        },
        if game_outcomes.pung.wins > 0 {
            Visibility::Visible
        } else {
            Visibility::Hidden
        },
        Transform::from_xyz(0., 0., -5.0),
        StateScoped(GameState::ChooseGame),
    ));

    commands.spawn((
        Name::new("Beef Blastoids Seal"),
        Sprite {
            image: texture_assets.panel2_blastoid_seal.clone(),
            custom_size: Some(Vec2::new(1280., 720.)),
            image_mode: SpriteImageMode::Auto,
            ..Default::default()
        },
        if game_outcomes.beef_blastoids.wins > 0 {
            Visibility::Visible
        } else {
            Visibility::Hidden
        },
        Transform::from_xyz(0., 0., -5.0),
        StateScoped(GameState::ChooseGame),
    ));

    commands.spawn((
        Name::new("Race Place Seal"),
        Sprite {
            image: texture_assets.panel2_raceplace_seal.clone(),
            custom_size: Some(Vec2::new(1280., 720.)),
            image_mode: SpriteImageMode::Auto,
            ..Default::default()
        },
        if game_outcomes.race_place.wins > 0 {
            Visibility::Visible
        } else {
            Visibility::Hidden
        },
        Transform::from_xyz(0., 0., -5.0),
        StateScoped(GameState::ChooseGame),
    ));
}

fn setup_clickable_meshes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn((
            Name::new("Pung"),
            StateScoped(GameState::ChooseGame),
            Mesh2d(meshes.add(Rectangle::from_size(Vec2::new(200.0, 50.0)))),
            MeshMaterial2d(
                materials.add(ColorMaterial::from_color(Srgba::new(1.0, 1.0, 1.0, 0.0))),
            ),
            Transform {
                translation: Vec3::new(-143.3, 108.5, 0.0),
                rotation: Quat::from_rotation_z(0.3),
                scale: Vec3::new(1.2, 1.1, 1.0),
            },
        ))
        .observe(
            |_: Trigger<Pointer<Click>>, mut next_state: ResMut<NextState<GameState>>| {
                next_state.set(GameState::Playing(Game::Pung));
            },
        )
        .observe(
            |_trigger: Trigger<Pointer<Over>>,
             mut visibility: Single<&mut Visibility, With<PungGlow>>| {
                **visibility = Visibility::Visible;
            },
        )
        .observe(
            |_trigger: Trigger<Pointer<Out>>,
             mut visibility: Single<&mut Visibility, With<PungGlow>>| {
                **visibility = Visibility::Hidden;
            },
        );

    commands
        .spawn((
            Name::new("Beef Blastoids"),
            StateScoped(GameState::ChooseGame),
            Mesh2d(meshes.add(Rectangle::from_size(Vec2::new(200.0, 50.0)))),
            MeshMaterial2d(
                materials.add(ColorMaterial::from_color(Srgba::new(1.0, 1.0, 1.0, 0.0))),
            ),
            Transform {
                translation: Vec3::new(-57.6, 23.5, 0.0),
                rotation: Quat::from_rotation_z(0.25),
                scale: Vec3::new(1.3, 1.1, 1.0),
            },
        ))
        .observe(
            |_: Trigger<Pointer<Click>>, mut next_state: ResMut<NextState<GameState>>| {
                next_state.set(GameState::Playing(Game::BeefBlastoids));
            },
        )
        .observe(
            |_trigger: Trigger<Pointer<Over>>,
             mut visibility: Single<&mut Visibility, With<BeefBlastoidsGlow>>| {
                **visibility = Visibility::Visible;
            },
        )
        .observe(
            |_trigger: Trigger<Pointer<Out>>,
             mut visibility: Single<&mut Visibility, With<BeefBlastoidsGlow>>| {
                **visibility = Visibility::Hidden;
            },
        );

    commands
        .spawn((
            Name::new("Race Place"),
            StateScoped(GameState::ChooseGame),
            Mesh2d(meshes.add(Rectangle::from_size(Vec2::new(200.0, 50.0)))),
            MeshMaterial2d(
                materials.add(ColorMaterial::from_color(Srgba::new(1.0, 1.0, 1.0, 0.0))),
            ),
            Transform {
                translation: Vec3::new(23.2, -216.6, 0.0),
                rotation: Quat::from_rotation_z(0.05),
                scale: Vec3::new(1.8, 1.5, 1.0),
            },
        ))
        .observe(
            |_: Trigger<Pointer<Click>>, mut next_state: ResMut<NextState<GameState>>| {
                next_state.set(GameState::Playing(Game::RacePlace));
            },
        )
        .observe(
            |_trigger: Trigger<Pointer<Over>>,
             mut visibility: Single<&mut Visibility, With<RacePlaceGlow>>| {
                **visibility = Visibility::Visible;
            },
        )
        .observe(
            |_trigger: Trigger<Pointer<Out>>,
             mut visibility: Single<&mut Visibility, With<RacePlaceGlow>>| {
                **visibility = Visibility::Hidden;
            },
        );
}

fn handle_game_outcomes(
    game_outcomes: Res<GameOutcomes>,
    mut next_choose_game_state: ResMut<NextState<ChooseGameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if game_outcomes.won_all_games() {
        next_game_state.set(GameState::CutScene(CutScene::EndA));
    } else if game_outcomes.lost_at_least_one() {
        next_choose_game_state.set(ChooseGameState::ChooseOrContinue);
    }
}
