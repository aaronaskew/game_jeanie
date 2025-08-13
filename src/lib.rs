#![allow(clippy::type_complexity)]

use bevy::prelude::*;

mod actions;
pub mod asteroids;
mod game_canvas;
mod loading;
mod menu;
pub mod pole_position;
pub mod pung;

use crate::actions::ActionsPlugin;
use crate::asteroids::AsteroidsPlugin;
use crate::game_canvas::{GameCanvas, GameCanvasBundle};
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::pung::PungPlugin;

#[cfg(debug_assertions)]
mod debug;
#[cfg(debug_assertions)]
use crate::debug::DebugPlugin;

const GAME_CANVAS_SIZE: Vec2 = Vec2::new(640., 480.);

#[derive(Component)]
pub struct Player;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
enum Game {
    Pung,
    Asteroids,
    PolePosition,
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
            .add_plugins(AsteroidsPlugin)
            .add_systems(Startup, (setup_camera, setup_game_canvas).chain());

        #[cfg(debug_assertions)]
        {
            app.add_plugins(DebugPlugin);
        }
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Msaa::Off));
}

fn setup_game_canvas(mut commands: Commands, camera_entity: Single<Entity, With<Camera>>) {
    // Position the GameCanvas relative to the camera
    let transform = Transform::from_xyz(150., 100., 0.);

    commands.spawn((GameCanvasBundle {
        game_canvas: GameCanvas(GAME_CANVAS_SIZE),
        child_of: ChildOf(*camera_entity),
        transform,
    },));
}
