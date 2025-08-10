use crate::{GameState, pung::PungState};
use avian2d::prelude::PhysicsDebugPlugin;
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::{dev_tools::states::log_transitions, prelude::*, window::PrimaryWindow};

mod world_inspector;
use world_inspector::DebugWorldInspectorPlugin;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, window_size)
            // .add_plugins(FrameTimeDiagnosticsPlugin::default())
            // .add_plugins(LogDiagnosticsPlugin::default())
            .add_systems(Update, log_transitions::<GameState>)
            .add_systems(Update, log_transitions::<PungState>)
            .add_systems(Update, escape)
            .add_plugins(PhysicsDebugPlugin::default())
            .add_plugins(DebugWorldInspectorPlugin);
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

fn escape(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: EventWriter<AppExit>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        match state.get() {
            GameState::Playing(_) => {
                next_state.set(GameState::Menu);
            }
            _ => {
                exit.write(AppExit::Success);
            }
        }
    }
}
