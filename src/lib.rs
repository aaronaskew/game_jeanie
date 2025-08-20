#![allow(clippy::type_complexity)]

use bevy::prelude::*;
use bevy::render::view::RenderLayers;

mod actions;
mod beef_blastoids;
mod choose_game;
mod game_canvas;
mod game_jeanie;
mod loading;
mod pung;
mod race_place;

use crate::actions::ActionsPlugin;
use crate::game_canvas::{GameCanvas, GameCanvasBundle};
use crate::loading::{LoadingPlugin, TextureAssets};
use crate::pung::PungPlugin;

#[cfg(debug_assertions)]
mod debug;

const GAME_CANVAS_SIZE: Vec2 = Vec2::new(640., 480.);
const GAME_CANVAS_POS: Vec2 = Vec2::new(243., 43.);
const ROOT_NODE_UI_TOP_LEFT: Vec2 = Vec2::new(563., 77.);

pub fn plugin(app: &mut App) {
    app.init_state::<GameState>()
        .add_computed_state::<TvScreenActive>()
        .enable_state_scoped_entities::<TvScreenActive>()
        .add_plugins(LoadingPlugin)
        .add_plugins(choose_game::plugin)
        .add_plugins(ActionsPlugin)
        .add_plugins(PungPlugin)
        .add_plugins(beef_blastoids::plugin)
        .add_systems(Startup, setup_camera);

    app.add_systems(
        OnEnter(TvScreenActive),
        (
            setup_game_canvas,
            setup_root_node,
            setup_playing_art_overlay,
        )
            .chain()
            .in_set(TvScreenSet),
    );

    #[cfg(debug_assertions)]
    {
        app.add_plugins(debug::plugin);
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
enum Game {
    Pung,
    BeefBlastoids,
    RacePlace,
}

#[derive(States, Clone, PartialEq, Eq, Debug, Hash)]
enum GameResult {
    Win,
    Lose,
}

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
#[states(scoped_entities)]
pub(crate) enum GameState {
    /// During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    /// Here the player chooses a game to play
    ChooseGame,
    /// During this State the actual game logic is executed
    Playing(Game),
    /// During this State choose the cheat codes for the chosen game
    _GameJeanie(Game),
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct TvScreenActive;

impl ComputedStates for TvScreenActive {
    /// We set the source state to be the state, or a tuple of states,
    /// we want to depend on. You can also wrap each state in an Option,
    /// if you want the computed state to execute even if the state doesn't
    /// currently exist in the world.
    type SourceStates = GameState;

    /// We then define the compute function, which takes in
    /// your SourceStates
    fn compute(sources: GameState) -> Option<Self> {
        match sources {
            // When we are in game, we want to return the InGame state
            GameState::Playing(_) | GameState::_GameJeanie(_) => Some(TvScreenActive),
            // Otherwise, we don't want the `State<InGame>` resource to exist,
            // so we return None.
            _ => None,
        }
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct ArtOverlayCamera;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct MainCamera;

fn setup_camera(mut commands: Commands) {
    commands.spawn((Name::new("Main Camera"), MainCamera, Camera2d, Msaa::Off));

    commands.spawn((
        Name::new("Art Overlay Camera"),
        ArtOverlayCamera,
        Camera2d,
        Camera {
            order: 1,
            clear_color: ClearColorConfig::None,
            ..Default::default()
        },
        RenderLayers::from_layers(&[1]),
        Msaa::Off,
    ));
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct RootNode;

#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct TvScreenSet;

fn setup_game_canvas(mut commands: Commands) {
    let transform = Transform::from_translation(GAME_CANVAS_POS.extend(0.));

    commands.spawn((
        StateScoped(TvScreenActive),
        Name::new("GameCanvas"),
        GameCanvasBundle {
            game_canvas: GameCanvas(GAME_CANVAS_SIZE),
            transform,
            visibility: InheritedVisibility::default(),
        },
    ));
}

fn setup_root_node(mut commands: Commands, canvas: Single<(&GameCanvas,)>) {
    let screen_position_top_left = ROOT_NODE_UI_TOP_LEFT;

    dbg!(screen_position_top_left);

    commands.spawn((
        StateScoped(TvScreenActive),
        RootNode,
        Name::new("RootNode"),
        Node {
            position_type: PositionType::Absolute,
            width: Val::Px(canvas.0.width()),
            height: Val::Px(canvas.0.height()),
            left: Val::Px(screen_position_top_left.x),
            top: Val::Px(screen_position_top_left.y),
            ..Default::default()
        },
        BackgroundColor(Color::srgba(1.0, 0.0, 0.0, 0.0)),
    ));
}

fn setup_playing_art_overlay(mut commands: Commands, texture_assets: Res<TextureAssets>) {
    commands.spawn((
        StateScoped(TvScreenActive),
        Name::new("Playing Background"),
        Sprite {
            image: texture_assets.panel4.clone(),
            custom_size: Some(Vec2::new(1280., 720.)),
            image_mode: SpriteImageMode::Auto,
            ..Default::default()
        },
        Transform::from_xyz(0., 0., 10.),
        RenderLayers::from_layers(&[1]),
    ));
}
