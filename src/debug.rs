use crate::GameState;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::{dev_tools::states::log_transitions, prelude::*, window::PrimaryWindow};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, window_size)
            .add_systems(Update, log_transitions::<GameState>)
            // .add_plugins(FrameTimeDiagnosticsPlugin::default())
            // .add_plugins(LogDiagnosticsPlugin::default())
            //
            ;
    }
}

fn window_size(window: Single<&Window, With<PrimaryWindow>>) {
    // let window = window

    info!(
        "window width: {} height: {}",
        window.resolution.width(),
        window.resolution.height()
    );
}
