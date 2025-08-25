use bevy::prelude::*;
use bevy_yarnspinner::{events::DialogueCompleteEvent, prelude::*};
use rand::{Rng, thread_rng};
use std::ops::Range;

use crate::{
    GameState,
    cut_scenes::cut_scene_definitions::{CutSceneDefinitions, CutSceneDefinitionsLoadingState},
};

pub mod cut_scene_definitions;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(cut_scene_definitions::plugin)
        .init_resource::<CurrentCutScene>()
        .register_type::<CurrentCutScene>()
        .init_resource::<DialogueActive>()
        .register_type::<DialogueActive>()
        .add_computed_state::<CutScenePlaying>()
        .add_systems(OnEnter(CutScenePlaying), trigger_new_cut_scene)
        .add_systems(
            OnTransition {
                exited: CutScenePlaying,
                entered: CutScenePlaying,
            },
            trigger_new_cut_scene,
        )
        .add_observer(setup_new_cut_scene)
        .add_systems(
            Update,
            (handle_dialogue_complete, play_cutscene, start_dialogue)
                .chain()
                .run_if(in_state(CutScenePlaying)),
        )
        .add_observer(end_and_cleanup_cut_scene);
}

#[derive(Event, Debug)]
pub struct NewCutSceneEvent;

#[derive(Event, Debug)]
pub struct CutSceneFinishedEvent;

fn trigger_new_cut_scene(mut commands: Commands) {
    commands.trigger(NewCutSceneEvent);
}

fn start_dialogue(
    mut dialogue_runner: Single<&mut DialogueRunner>,
    current_cut_scene: Res<CurrentCutScene>,
    dialogue_active: Res<DialogueActive>,
) -> Result {
    if dialogue_active.0
        && !dialogue_runner.is_running()
        && let Some(descriptor) = &current_cut_scene.descriptor
        && let Some(start_node) = &descriptor.dialogue_start_node
    {
        dialogue_runner.start_node(start_node);
    }

    Ok(())
}

fn handle_dialogue_complete(
    mut reader: EventReader<DialogueCompleteEvent>,
    mut commands: Commands,
    mut dialogue_active: ResMut<DialogueActive>,
) -> Result {
    if reader.read().next().is_some() {
        reader.clear();
        commands.trigger(CutSceneFinishedEvent);
        dialogue_active.0 = false;
    }

    Ok(())
}

fn end_and_cleanup_cut_scene(
    _trigger: Trigger<CutSceneFinishedEvent>,
    mut next_state: ResMut<NextState<GameState>>,
    mut current_cut_scene: ResMut<CurrentCutScene>,
    cut_scene_image: Single<Entity, With<CurrentCutSceneImage>>,
    mut commands: Commands,
) -> Result {
    commands.entity(*cut_scene_image).despawn();

    let next_game_state = &current_cut_scene
        .descriptor
        .as_ref()
        .ok_or("No CutSceneDescriptor in CurrentCutScene")?
        .next_game_state;

    next_state.set(next_game_state.clone());

    *current_cut_scene = CurrentCutScene::default();

    Ok(())
}

pub fn setup_new_cut_scene(
    _: Trigger<NewCutSceneEvent>,
    mut current_cut_scene: ResMut<CurrentCutScene>,
    cut_scene_state: Res<State<GameState>>,
    cut_scene_definitions: Res<CutSceneDefinitions>,
    mut dialogue_active: ResMut<DialogueActive>,
) -> Result {
    let descriptor = match &**cut_scene_state {
        GameState::CutScene(cut_scene) => cut_scene_definitions.0.get(cut_scene).ok_or(format!(
            "CutScene {cut_scene:?} not found in CutSceneDefinitions"
        ))?,
        _ => {
            return Err("Should only be seeing GameState::CutScene(_) states here.".into());
        }
    };

    *current_cut_scene = CurrentCutScene {
        descriptor: Some(descriptor.clone()),
        timer: None,
        started: false,
    };

    dialogue_active.0 = true;

    Ok(())
}

fn play_cutscene(
    mut commands: Commands,
    mut current_cut_scene: ResMut<CurrentCutScene>,
    time: Res<Time>,
    mut cut_scene_idx: Local<usize>,
    current_cut_scene_sprite: Option<Single<Entity, With<CurrentCutSceneImage>>>,
) -> Result {
    let current_cut_scene = &mut *current_cut_scene;

    if current_cut_scene.descriptor.is_some() {
        let descriptor = current_cut_scene
            .descriptor
            .as_ref()
            .ok_or("No CutSceneDescriptor")?;

        let queue = &descriptor.queue;

        let mut rng = thread_rng();

        if !current_cut_scene.started {
            //setup first frame
            *cut_scene_idx = 0;

            let CutSceneFrame {
                image,
                duration_range,
            } = queue.get(*cut_scene_idx).ok_or("Can't index queue")?;

            commands.spawn((
                CurrentCutSceneImage,
                Name::new("Cut Scene Frame"),
                Sprite {
                    image: image.clone(),
                    custom_size: Some(Vec2::new(1280., 720.)),
                    ..Default::default()
                },
            ));

            if let Some(duration_range) = duration_range {
                current_cut_scene.timer = Some(Timer::from_seconds(
                    if duration_range.is_empty() {
                        duration_range.start
                    } else {
                        rng.gen_range(duration_range.clone())
                    },
                    TimerMode::Once,
                ));
            }

            current_cut_scene.started = true;
        } else {
            let CutSceneFrame {
                image: _,
                duration_range,
            } = queue.get(*cut_scene_idx).ok_or("Can't index queue")?;

            let use_timer = duration_range.is_some();

            if use_timer {
                let timer = current_cut_scene.timer.as_mut().ok_or("Can't find timer")?;

                timer.tick(time.delta());

                if timer.finished() {
                    if *cut_scene_idx == queue.len() - 1 && !descriptor.should_loop {
                        commands.trigger(CutSceneFinishedEvent);
                    } else {
                        *cut_scene_idx = (*cut_scene_idx + 1) % queue.len();

                        let CutSceneFrame {
                            image,
                            duration_range,
                        } = queue.get(*cut_scene_idx).ok_or("Can't index queue")?;

                        commands
                            .entity(
                                **current_cut_scene_sprite
                                    .as_ref()
                                    .ok_or("Should find a current sprite")?,
                            )
                            .despawn();

                        commands.spawn((
                            CurrentCutSceneImage,
                            Name::new("Cut Scene Frame"),
                            Sprite {
                                image: image.clone(),
                                custom_size: Some(Vec2::new(1280., 720.)),
                                ..Default::default()
                            },
                        ));

                        if let Some(duration_range) = duration_range {
                            *timer = Timer::from_seconds(
                                if duration_range.is_empty() {
                                    duration_range.start
                                } else {
                                    rng.gen_range(duration_range.clone())
                                },
                                TimerMode::Once,
                            );
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

// fn clear_current_cutscene(mut current_cut_scene: ResMut<CurrentCutScene>) {
//     *current_cut_scene = CurrentCutScene::default();
// }

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct CurrentCutSceneImage;

#[derive(Debug, Default, Reflect, Clone)]
struct CutSceneFrame {
    image: Handle<Image>,
    duration_range: Option<Range<f32>>,
}

impl CutSceneFrame {
    fn new(image: Handle<Image>, duration_range: Option<Range<f32>>) -> Self {
        Self {
            image,
            duration_range,
        }
    }
}

#[derive(Debug, Default, Reflect, Clone)]
pub struct CutSceneDescriptor {
    queue: Vec<CutSceneFrame>,
    should_loop: bool,
    pub dialogue_start_node: Option<String>,
    pub next_game_state: GameState,
}

impl CutSceneDescriptor {
    fn new(
        queue: Vec<CutSceneFrame>,
        should_loop: bool,
        dialogue_start_node: Option<String>,
        next_game_state: GameState,
    ) -> Self {
        assert!(
            !should_loop || dialogue_start_node.is_some(),
            "cut scene should either have dialogue or should not loop"
        );

        if dialogue_start_node.is_none() {
            for frame in &queue {
                assert!(
                    frame.duration_range.is_some(),
                    "if cut scene doesn't have dialogue, then each frame should have a timer duration. else, the cut scene would never progress.\nframe: {frame:?}"
                );
            }
        }

        Self {
            queue,
            should_loop,
            dialogue_start_node,
            next_game_state,
        }
    }
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct CurrentCutScene {
    pub descriptor: Option<CutSceneDescriptor>,
    timer: Option<Timer>,
    started: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect)]
pub enum CutScene {
    StartA,
    MiddleA,
    MiddleB,
    MiddleC,
    MiddleD,
    MiddleE,
    MiddleF,
    MiddleG,
    MiddleH,
    MiddleI,
    EndA,
    EndB,
    EndC,
    EndD,
    EndE,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub(crate) struct CutScenePlaying;

impl ComputedStates for CutScenePlaying {
    type SourceStates = (GameState, CutSceneDefinitionsLoadingState);

    fn compute((game_state, cut_scene_loading_state): Self::SourceStates) -> Option<Self> {
        match (game_state, cut_scene_loading_state) {
            (GameState::CutScene(_), CutSceneDefinitionsLoadingState::Complete) => Some(Self),
            _ => None,
        }
    }
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct DialogueActive(bool);
