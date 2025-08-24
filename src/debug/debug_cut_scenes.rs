use bevy::{dev_tools::states::log_transitions, prelude::*};

use crate::cut_scenes::{
    CurrentCutScene, CutSceneFinishedEvent, NewCutSceneEvent,
    cut_scene_definitions::CutSceneDefinitionsLoadingState,
};

pub(super) fn plugin(app: &mut App) {
    app
    .add_observer(|trigger: Trigger<NewCutSceneEvent>| {
        info!("NewCutSceneEvent triggered: {trigger:#?}")
    })
    .add_observer(|trigger: Trigger<CutSceneFinishedEvent>| {
        info!("CutSceneFinishedEvent triggered: {trigger:#?}")
    })
    .add_systems(
        Update,
        current_cut_scene_changed.run_if(resource_changed::<CurrentCutScene>),
    )
    .add_systems(
        PreUpdate,
        log_transitions::<CutSceneDefinitionsLoadingState>,
    );
}

fn current_cut_scene_changed(current_cut_scene: Res<CurrentCutScene>) {
    info!("CurrentCutScene changed: {current_cut_scene:#?}");
}
