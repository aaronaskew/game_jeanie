use bevy::prelude::*;
use bevy_yarnspinner::{events::DialogueCompleteEvent, prelude::*};
use bevy_yarnspinner_example_dialogue_view::prelude::*;

use crate::{
    GameState,
    cut_scenes::{CutScene, CutScenePlaying},
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        // Register the Yarn Spinner plugin using its default settings, which will look for Yarn files in the "dialogue" folder.
        // If this app should support Wasm or Android, we cannot load files without specifying them, so use the following instead.
        // YarnSpinnerPlugin::with_yarn_source(YarnFileSource::file("dialogue/hello_world.yarn")),
        YarnSpinnerPlugin::with_yarn_sources([
            YarnFileSource::file("dialog/intro.yarn"),
            YarnFileSource::file("dialog/middle.yarn"),
            YarnFileSource::file("dialog/closing.yarn"),
        ]),
        // Initialize the bundled example UI
        ExampleYarnSpinnerDialogueViewPlugin::new(),
    ))
    .add_systems(
        OnEnter(GameState::CutScene(CutScene::StartA)),
        // Spawn the dialogue runner once the Yarn project has finished compiling
        spawn_dialogue_runner_intro.run_if(resource_added::<YarnProject>),
    )
    .add_systems(
        Update,
        (|mut reader: EventReader<DialogueCompleteEvent>,
          mut next_state: ResMut<NextState<GameState>>| {
            for evt in reader.read() {
                info!("DialogueCompleteEvent: {evt:?}");
                next_state.set(GameState::ChooseGame);
            }
        })
        .run_if(in_state(GameState::CutScene(CutScene::StartA))),
    );
}

fn spawn_dialogue_runner_intro(mut commands: Commands, project: Res<YarnProject>) {
    // Create a dialogue runner from the project.
    let mut dialogue_runner = project.create_dialogue_runner(&mut commands);

    // Immediately start showing the dialogue to the player
    dialogue_runner.start_node("Intro");

    commands.spawn((
        Name::new("Dialogue Runner"),
        StateScoped(CutScenePlaying),
        dialogue_runner,
    ));
}
