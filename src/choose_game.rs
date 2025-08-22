use bevy::prelude::*;

use crate::{Game, GameState, GamesWon, loading::TextureAssets};

const GLOW_COLOR: Color = Color::srgba(1.0, 1.0, 1.0, 0.68);

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(MeshPickingPlugin)
        .add_sub_state::<ChooseGameState>()
        .add_systems(
            OnEnter(GameState::ChooseGame),
            (setup_choose_game_panel, setup_menu).chain(),
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
    FirstChoice,
    LosePreGameJeanie,
}

fn setup_choose_game_panel(
    mut commands: Commands,
    texture_assets: Res<TextureAssets>,
    games_won: Res<GamesWon>,
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
        if games_won.pung {
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
        if games_won.beef_blastoids {
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
        if games_won.race_place {
            Visibility::Visible
        } else {
            Visibility::Hidden
        },
        Transform::from_xyz(0., 0., -5.0),
        StateScoped(GameState::ChooseGame),
    ));
}

fn setup_menu(
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
