use bevy::prelude::*;
use rand::{Rng, thread_rng};
use std::ops::Range;

use crate::{GameState, loading::TextureAssets};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<CurrentCutScene>()
        .add_computed_state::<CutScenePlaying>()
        .enable_state_scoped_entities::<CutScenePlaying>()
        .add_systems(OnEnter(CutScenePlaying), setup_cut_scene)
        .add_systems(Update, play_cutscene.run_if(in_state(CutScenePlaying)))
        .add_systems(OnExit(CutScenePlaying), clear_current_cutscene);
}

fn setup_cut_scene(
    mut current_cut_scene: ResMut<CurrentCutScene>,
    texture_assets: Res<TextureAssets>,
    cut_scene_state: Res<State<GameState>>,
) -> Result {
    let descriptor = match &**cut_scene_state {
        GameState::CutScene(cut_scene) => match cut_scene {
            CutScene::StartA => CutSceneDescriptor::new(
                vec![
                    CutSceneFrame::new(texture_assets.start_a_01.clone(), Some(0.1..1.0)),
                    CutSceneFrame::new(texture_assets.start_a_02.clone(), Some(0.1..1.0)),
                ],
                true,
            ),
            CutScene::MiddleA => CutSceneDescriptor::new(
                vec![CutSceneFrame::new(texture_assets.middle_a_01.clone(), None)],
                false,
            ),
            CutScene::MiddleB => CutSceneDescriptor::new(
                vec![CutSceneFrame::new(texture_assets.middle_b_01.clone(), None)],
                false,
            ),
            CutScene::MiddleC => CutSceneDescriptor::new(
                vec![
                    CutSceneFrame::new(texture_assets.middle_c_01.clone(), Some(1.0..1.0)),
                    CutSceneFrame::new(texture_assets.middle_c_02.clone(), Some(1.0..1.0)),
                ],
                false,
            ),
            CutScene::MiddleD => CutSceneDescriptor::new(
                vec![
                    CutSceneFrame::new(texture_assets.middle_d_01.clone(), Some(0.1..0.3)),
                    CutSceneFrame::new(texture_assets.middle_d_02.clone(), Some(0.1..0.3)),
                    CutSceneFrame::new(texture_assets.middle_d_03.clone(), Some(0.1..0.3)),
                ],
                true,
            ),
            CutScene::MiddleE => CutSceneDescriptor::new(
                vec![
                    CutSceneFrame::new(texture_assets.middle_e_01.clone(), Some(1.0..1.0)),
                    CutSceneFrame::new(texture_assets.middle_e_02.clone(), Some(1.0..1.0)),
                    CutSceneFrame::new(texture_assets.middle_e_03.clone(), Some(0.5..0.5)),
                    CutSceneFrame::new(texture_assets.middle_e_04.clone(), Some(0.5..0.5)),
                ],
                false,
            ),
            CutScene::MiddleF => CutSceneDescriptor::new(
                vec![CutSceneFrame::new(texture_assets.middle_f_01.clone(), None)],
                false,
            ),
            CutScene::MiddleG => CutSceneDescriptor::new(
                vec![CutSceneFrame::new(texture_assets.middle_g_01.clone(), None)],
                false,
            ),
            CutScene::MiddleH => CutSceneDescriptor::new(
                vec![CutSceneFrame::new(texture_assets.middle_h_01.clone(), None)],
                false,
            ),
            CutScene::MiddleI => CutSceneDescriptor::new(
                vec![CutSceneFrame::new(texture_assets.middle_g_01.clone(), None)],
                false,
            ),
            CutScene::EndA => CutSceneDescriptor::new(
                vec![CutSceneFrame::new(texture_assets.end_a_01.clone(), None)],
                false,
            ),
            CutScene::EndB => CutSceneDescriptor::new(
                vec![
                    CutSceneFrame::new(texture_assets.end_b_01.clone(), Some(1.0..1.0)),
                    CutSceneFrame::new(texture_assets.end_b_02.clone(), Some(2.0..2.0)),
                ],
                false,
            ),
            CutScene::EndC => CutSceneDescriptor::new(
                vec![CutSceneFrame::new(texture_assets.end_b_02.clone(), None)],
                false,
            ),
            CutScene::EndD => CutSceneDescriptor::new(
                vec![
                    CutSceneFrame::new(texture_assets.end_b_02.clone(), Some(1.0..1.0)),
                    CutSceneFrame::new(texture_assets.end_d_02.clone(), Some(1.5..1.5)),
                    CutSceneFrame::new(texture_assets.end_d_03.clone(), Some(1.0..1.0)),
                    CutSceneFrame::new(texture_assets.end_d_04.clone(), Some(1.0..1.0)),
                    CutSceneFrame::new(texture_assets.end_d_05.clone(), Some(1.0..1.0)),
                    CutSceneFrame::new(texture_assets.end_d_06.clone(), Some(1.0..1.0)),
                ],
                false,
            ),
            CutScene::EndE => CutSceneDescriptor::new(
                vec![CutSceneFrame::new(texture_assets.end_e_01.clone(), None)],
                false,
            ),
        },
        _ => {
            return Err("Should only be seeing GameState::CutScene(_) states here.".into());
        }
    };

    *current_cut_scene = CurrentCutScene {
        descriptor: Some(descriptor),
        timer: None,
        started: false,
    };

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
            StateScoped(CutScenePlaying),
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
        let timer = current_cut_scene.timer.as_mut().ok_or("Can't find timer")?;

        timer.tick(time.delta());

        if timer.finished() {
            if *cut_scene_idx == queue.len() - 1 && !descriptor.should_loop {
                // TODO: finish cut scene, next_state stored in descriptor?
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
                    StateScoped(CutScenePlaying),
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

#[derive(Debug, Default, Reflect)]
struct CutSceneDescriptor {
    queue: Vec<CutSceneFrame>,
    should_loop: bool,
}

impl CutSceneDescriptor {
    fn new(queue: Vec<CutSceneFrame>, should_loop: bool) -> Self {
        Self { queue, should_loop }
    }
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
    type SourceStates = GameState;

    fn compute(sources: Self::SourceStates) -> Option<Self> {
        match sources {
            GameState::CutScene(_) => Some(Self),
            _ => None,
        }
    }
}
