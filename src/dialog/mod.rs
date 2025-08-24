use bevy::prelude::*;
use bevy_yarnspinner::prelude::*;
use bevy_yarnspinner_example_dialogue_view::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        // Register the Yarn Spinner plugin using its default settings, which will look for Yarn files in the "dialogue" folder.
        // If this app should support Wasm or Android, we cannot load files without specifying them, so use the following instead.
        // YarnSpinnerPlugin::with_yarn_source(YarnFileSource::file("dialogue/hello_world.yarn")),
        YarnSpinnerPlugin::with_yarn_sources([YarnFileSource::file("dialog/cut_scenes.yarn")]),
        // Initialize the bundled example UI
        ExampleYarnSpinnerDialogueViewPlugin::new(),
    ))
    .add_systems(
        Update,
        // Spawn the dialogue runner once the Yarn project has finished compiling
        spawn_dialogue_runner.run_if(resource_added::<YarnProject>),
    );
}

fn spawn_dialogue_runner(mut commands: Commands, project: Res<YarnProject>) -> Result {
    // Create a dialogue runner from the project.
    let dialogue_runner = project.create_dialogue_runner(&mut commands);

    commands.spawn((Name::new("Dialogue Runner"), dialogue_runner));

    Ok(())
}
