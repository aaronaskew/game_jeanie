#![allow(clippy::type_complexity)]

// mod actions;
// mod audio;
mod debug;
mod loading;
mod menu;
// mod player;
pub mod pung;

// use crate::actions::ActionsPlugin;
// use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
// use crate::player::PlayerPlugin;
use crate::{debug::DebugPlugin, pung::PungPlugin};

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::prelude::*;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
enum Game {
    Pung,
    Asteroids,
    PolePosition,
}

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing(Game),
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_plugins(LoadingPlugin)
            .add_plugins(MenuPlugin)
            //
            // .add_plugins(ActionsPlugin)
            // .add_plugins(InternalAudioPlugin)
            // .add_plugins(PlayerPlugin)
            // .add_plugins((EguiPlugin::default(), WorldInspectorPlugin::new()))
            //
            .add_plugins(PungPlugin)
            ;

        #[cfg(debug_assertions)]
        {
            app.add_plugins(DebugPlugin);
        }
    }
}
