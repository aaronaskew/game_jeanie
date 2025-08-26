use crate::{
    // GameOutcome, GameOutcomes,
    GameState,
    TvScreenActive,
    beef_blastoids::{self, BeefBlastoidsState},
    cut_scenes::CutScenePlaying,
    pung::PungState,
};
use bevy::{dev_tools::states::log_transitions, prelude::*, window::PrimaryWindow};

mod debug_cut_scenes;

#[cfg(debug_assertions)]
mod world_inspector;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(debug_cut_scenes::plugin)
        .add_systems(Startup, window_size)
        .add_systems(Update, log_transitions::<GameState>)
        .add_systems(Update, log_transitions::<TvScreenActive>)
        .add_systems(Update, log_transitions::<PungState>)
        .add_systems(Update, log_transitions::<BeefBlastoidsState>)
        .add_systems(Update, log_transitions::<beef_blastoids::RunningState>)
        .add_systems(Update, log_transitions::<CutScenePlaying>)
        // .add_systems(Update, game_canvas_gizmo)
        ;

    #[cfg(debug_assertions)]
    {
        use avian2d::prelude::{PhysicsDebugPlugin, PhysicsGizmos};
        use bevy::{
            dev_tools::picking_debug::{DebugPickingMode, DebugPickingPlugin},
            input::common_conditions::input_just_pressed,
        };
        use world_inspector::DebugWorldInspectorPlugin;

        // use crate::GameOutcomes;

        app.add_systems(Update, escape)
            .add_plugins(PhysicsDebugPlugin::default())
            .add_plugins(DebugWorldInspectorPlugin)
            .add_plugins(DebugPickingPlugin)
            .insert_resource(DebugPickingMode::Normal)
            .add_systems(
                PreUpdate,
                (|mut mode: ResMut<DebugPickingMode>| {
                    *mode = match *mode {
                        DebugPickingMode::Disabled => DebugPickingMode::Normal,
                        _ => DebugPickingMode::Disabled,
                    };
                })
                .distributive_run_if(input_just_pressed(KeyCode::KeyP)),
            )
            .add_systems(
                Update,
                (|mut config_store: ResMut<GizmoConfigStore>| {
                    let (physics_gizmo_config, _) = config_store.config_mut::<PhysicsGizmos>();

                    physics_gizmo_config.enabled ^= true;
                })
                .distributive_run_if(input_just_pressed(KeyCode::KeyG)),
            )
            // .add_systems(
            //     Update,
            //     debug_game_outcomes.run_if(resource_added::<GameOutcomes>),
            // )
            ;
    }
}

// fn debug_game_outcomes(mut outcomes: ResMut<GameOutcomes>) {
//     *outcomes = GameOutcomes {
//         pung: GameOutcome { wins: 1, losses: 0 },
//         beef_blastoids: GameOutcome { wins: 1, losses: 0 },
//         race_place: GameOutcome { wins: 1, losses: 0 },
//     };
// }

fn window_size(window: Single<&Window, With<PrimaryWindow>>) {
    // let window = window

    info!(
        "window width: {} height: {}",
        window.resolution.width(),
        window.resolution.height()
    );
}

#[cfg(debug_assertions)]
fn escape(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: EventWriter<AppExit>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        match state.get() {
            GameState::Playing(_) => {
                next_state.set(GameState::ChooseGame);
            }
            _ => {
                exit.write(AppExit::Success);
            }
        }
    }
}

// fn game_canvas_gizmo(game_canvas_query: Single<(&GameCanvas, &Transform)>, mut gizmos: Gizmos) {
//     let (game_canvas, transform) = *game_canvas_query;

//     gizmos.rect_2d(
//         Isometry2d::from_translation(transform.translation.truncate()),
//         **game_canvas,
//         GREEN,
//     );
// }
