use crate::{pung::PungState, GameState};
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::{dev_tools::states::log_transitions, prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, window_size)
            .add_systems(Update, log_transitions::<GameState>)
            .add_systems(Update, log_transitions::<PungState>)
            
            .add_systems(Update, escape)
            .add_plugins(EguiPlugin::default())
            .add_plugins(WorldInspectorPlugin::new())
            // .add_plugins(FrameTimeDiagnosticsPlugin::default())
            // .add_plugins(LogDiagnosticsPlugin::default())
            //
            ;
    }
}

// #[derive(Resource, Default)]
// struct InspectorToggle {
//     enabled: bool,
// }

// fn toggle_inspector(
//     mut toggle: ResMut<InspectorToggle>,
//     keyboard: Res<ButtonInput<KeyCode>>,
//     mut windows: Query<&mut bevy_inspector_egui::bevy_egui::EguiContext>,
// ) {
//     if keyboard.just_pressed(KeyCode::F1) {
//         toggle.enabled = !toggle.enabled;
//     }
    
//     // Force the inspector to stay visible
//     if toggle.enabled {
//         for mut context in windows.iter_mut() {
//             // Inspector UI code here
//         }
//     }
// }

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
