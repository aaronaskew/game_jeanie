use bevy::prelude::*;
use rand::{Rng, thread_rng};

use crate::{GameState, loading::TextureAssets};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<CurrentCutScene>()
        .add_computed_state::<CutScenePlaying>()
        .enable_state_scoped_entities::<CutScenePlaying>()
        .add_systems(OnEnter(GameState::CutScene(CutScene::Intro)), setup_intro)
        .add_systems(OnEnter(GameState::CutScene(CutScene::Middle)), setup_middle)
        .add_systems(
            OnEnter(GameState::CutScene(CutScene::Closing)),
            setup_closing,
        )
        .add_systems(Update, play_cutscene.run_if(in_state(CutScenePlaying)))
        // TODO: replace button with dialog system
        // .add_systems(Update, handle_button.run_if(in_state(CutScenePlaying)))
        .add_systems(OnExit(CutScenePlaying), clear_current_cutscene);
}

fn setup_intro(mut current_cut_scene: ResMut<CurrentCutScene>, texture_assets: Res<TextureAssets>) {
    let queue = vec![
        CutSceneFrame {
            image: texture_assets.panel1_frame_a.clone(),
            timer_duration_min: 0.1,
            timer_duration_max: 1.0,
        },
        CutSceneFrame {
            image: texture_assets.panel1_frame_b.clone(),
            timer_duration_min: 0.1,
            timer_duration_max: 1.0,
        },
    ];

    *current_cut_scene = CurrentCutScene {
        descriptor: Some(CutSceneDescriptor {
            queue,
            should_loop: true,
        }),
        timer: None,
        started: false,
    };
}
fn setup_middle() {}
fn setup_closing() {}

fn play_cutscene(
    mut commands: Commands,
    mut current_cut_scene: ResMut<CurrentCutScene>,
    time: Res<Time>,
    mut cut_scene_idx: Local<usize>,
    current_cut_scene_sprite: Option<Single<Entity, With<CurrentCutSceneImage>>>,
) -> Result {
    let current_cut_scene = &mut *current_cut_scene;

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
            timer_duration_min,
            timer_duration_max,
        } = queue.get(*cut_scene_idx).ok_or("Can't index queue")?;

        commands.spawn((
            CurrentCutSceneImage,
            StateScoped(CutScenePlaying),
            Name::new("Cut Scene Frame"),
            Sprite {
                image: image.clone(),
                custom_size: Some(Vec2::new(1280., 720.)),
                ..Default::default()
            },
        ));

        current_cut_scene.timer = Some(Timer::from_seconds(
            rng.gen_range(*timer_duration_min..*timer_duration_max),
            TimerMode::Once,
        ));

        current_cut_scene.started = true;
    } else {
        let timer = current_cut_scene.timer.as_mut().ok_or("Can't find timer")?;

        timer.tick(time.delta());

        if timer.finished() {
            if *cut_scene_idx == queue.len() - 1 && !descriptor.should_loop {
                // TODO: finish cut scene, next_state stored in descriptor?
            } else {
                *cut_scene_idx = (*cut_scene_idx + 1) % queue.len();

                let CutSceneFrame {
                    image,
                    timer_duration_min,
                    timer_duration_max,
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
                    StateScoped(CutScenePlaying),
                    Name::new("Cut Scene Frame"),
                    Sprite {
                        image: image.clone(),
                        custom_size: Some(Vec2::new(1280., 720.)),
                        ..Default::default()
                    },
                ));

                *timer = Timer::from_seconds(
                    rng.gen_range(*timer_duration_min..*timer_duration_max),
                    TimerMode::Once,
                );
            }
        }
    }

    Ok(())
}

fn clear_current_cutscene(mut current_cut_scene: ResMut<CurrentCutScene>) {
    current_cut_scene.descriptor = None;
    current_cut_scene.timer = None;
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct CurrentCutSceneImage;

#[derive(Debug, Default, Reflect)]
struct CutSceneFrame {
    image: Handle<Image>,
    timer_duration_min: f32,
    timer_duration_max: f32,
}

#[derive(Debug, Default, Reflect)]
struct CutSceneDescriptor {
    queue: Vec<CutSceneFrame>,
    should_loop: bool,
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
struct CurrentCutScene {
    descriptor: Option<CutSceneDescriptor>,
    timer: Option<Timer>,
    started: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CutScene {
    Intro,
    Middle,
    Closing,
    TheEnd,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub(crate) struct CutScenePlaying;

impl ComputedStates for CutScenePlaying {
    type SourceStates = GameState;

    fn compute(sources: Self::SourceStates) -> Option<Self> {
        match sources {
            GameState::CutScene(_) => Some(Self),
            _ => None,
        }
    }
}
