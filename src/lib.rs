#![allow(clippy::type_complexity)]

use bevy::prelude::*;

mod actions;
pub mod beef_blastoids;
mod game_canvas;
mod loading;
mod menu;
pub mod pung;
pub mod race_place;

use crate::actions::ActionsPlugin;
use crate::beef_blastoids::BeefBlastoidsPlugin;
use crate::game_canvas::{GameCanvas, GameCanvasBundle};
use crate::loading::{LoadingPlugin, TextureAssets};
use crate::menu::MenuPlugin;
use crate::pung::PungPlugin;

#[cfg(debug_assertions)]
mod debug;
#[cfg(debug_assertions)]
use crate::debug::DebugPlugin;

const GAME_CANVAS_SIZE: Vec2 = Vec2::new(640., 480.);
const GAME_CANVAS_POS: Vec2 = Vec2::new(243., 43.);
const ROOT_NODE_UI_TOP_LEFT: Vec2 = Vec2::new(563., 77.);

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
    /// During this State the actual game logic is executed
    Playing(Game),
    /// Here the menu is drawn and waiting for player interaction
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_plugins(LoadingPlugin)
            .add_plugins(MenuPlugin)
            .add_plugins(ActionsPlugin)
            .add_plugins(PungPlugin)
            .add_plugins(BeefBlastoidsPlugin)
            .add_systems(
                Startup,
                (setup_camera, setup_game_canvas, setup_root_node).chain(),
            );

        app.add_systems(OnEnter(GameState::Menu), setup_playing_background);

        #[cfg(debug_assertions)]
        {
            app.add_plugins(DebugPlugin);
        }
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Msaa::Off));
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct RootNode;

fn setup_game_canvas(mut commands: Commands) {
    let transform = Transform::from_translation(GAME_CANVAS_POS.extend(0.));

    commands.spawn((
        GameCanvasBundle {
            game_canvas: GameCanvas(GAME_CANVAS_SIZE),
            transform,
            visibility: InheritedVisibility::default(),
        },
        Name::new("GameCanvas"),
    ));
}

fn setup_root_node(mut commands: Commands, canvas: Single<(&GameCanvas,)>) {
    let screen_position_top_left = ROOT_NODE_UI_TOP_LEFT;

    dbg!(screen_position_top_left);

    commands.spawn((
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

// TODO: Move this to the appropriate file/state.
fn setup_playing_background(mut commands: Commands, texture_assets: Res<TextureAssets>) {
    commands.spawn((
        Name::new("Playing Background"),
        Sprite {
            image: texture_assets.panel4.clone(),
            custom_size: Some(Vec2::new(1280., 720.)),
            image_mode: SpriteImageMode::Auto,
            ..Default::default()
        },
    ));
}
