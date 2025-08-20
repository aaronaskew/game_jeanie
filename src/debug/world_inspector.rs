use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

pub struct DebugWorldInspectorPlugin;

impl Plugin for DebugWorldInspectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin::default())
            .add_plugins(WorldInspectorPlugin::default());
    }
}
