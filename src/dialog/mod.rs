use bevy::prelude::*;
use bevy_yarnspinner::{events::DialogueCompleteEvent, prelude::*};
use bevy_yarnspinner_example_dialogue_view::prelude::*;

use crate::{
    GameState,
    cut_scenes::{CurrentCutScene, CutScenePlaying, setup_cut_scene},
};

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
        OnEnter(CutScenePlaying),
        // Spawn the dialogue runner once the Yarn project has finished compiling
        spawn_dialogue_runner
            .run_if(resource_added::<YarnProject>)
            .after(setup_cut_scene),
    )
    .add_systems(
        Update,
        handle_end_of_dialog.run_if(in_state(CutScenePlaying)),
    );
}

fn spawn_dialogue_runner(
    mut commands: Commands,
    project: Res<YarnProject>,
    current_cut_scene: Res<CurrentCutScene>,
) -> Result {
    info!("current_cut_scene: {current_cut_scene:?}");

    // Create a dialogue runner from the project.
    let mut dialogue_runner = project.create_dialogue_runner(&mut commands);

    let start_node = current_cut_scene
        .into_inner()
        .descriptor
        .as_ref()
        .ok_or("No CutSceneDescriptor in CurrentCutScene")?
        .dialog_start_node
        .as_ref()
        .ok_or("No dialog start_node in this CutSceneDescriptor")?
        .as_str();

    // Immediately start showing the dialogue to the player
    dialogue_runner.start_node(start_node);

    commands.spawn((
        Name::new("Dialogue Runner"),
        StateScoped(CutScenePlaying),
        dialogue_runner,
    ));

    Ok(())
}

fn handle_end_of_dialog(
    mut reader: EventReader<DialogueCompleteEvent>,
    mut next_state: ResMut<NextState<GameState>>,
    current_cut_scene: Res<CurrentCutScene>,
) -> Result {
    let next_game_state = &current_cut_scene
        .into_inner()
        .descriptor
        .as_ref()
        .ok_or("No CutSceneDescriptor in CurrentCutScene")?
        .next_game_state;

    for evt in reader.read() {
        info!("DialogueCompleteEvent: {evt:?}");
        next_state.set(next_game_state.clone());
    }

    Ok(())
}
