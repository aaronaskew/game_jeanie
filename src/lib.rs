#![allow(clippy::type_complexity)]

mod actions;
mod audio;
mod loading;
mod menu;
mod player;
pub mod pung;

// use crate::actions::ActionsPlugin;
// use crate::audio::InternalAudioPlugin;
// use crate::loading::LoadingPlugin;
// use crate::menu::MenuPlugin;
// use crate::player::PlayerPlugin;
use crate::pung::PungPlugin;

use bevy::app::App;
use bevy::asset::AssetMetaCheck;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            // .add_plugins((
            //     LoadingPlugin,
            //     MenuPlugin,
            //     ActionsPlugin,
            //     InternalAudioPlugin,
            //     PlayerPlugin,
            // ))
            // .add_plugins((EguiPlugin::default(), WorldInspectorPlugin::new()))
            .add_plugins(
                DefaultPlugins
                    .set(WindowPlugin {
                        primary_window: Some(Window {
                            title: "Bevy game".to_string(), // ToDo
                            // Bind to canvas included in `index.html`
                            canvas: Some("#bevy".to_owned()),
                            fit_canvas_to_parent: true,
                            // Tells wasm not to override default event handling, like F5 and Ctrl+R
                            prevent_default_event_handling: false,
                            ..default()
                        }),
                        ..default()
                    })
                    .set(AssetPlugin {
                        meta_check: AssetMetaCheck::Never,
                        ..default()
                    }),
            )
            .add_plugins(PungPlugin);

        #[cfg(debug_assertions)]
        {
            app.add_plugins((
                FrameTimeDiagnosticsPlugin::default(),
                LogDiagnosticsPlugin::default(),
            ));
        }
    }
}
