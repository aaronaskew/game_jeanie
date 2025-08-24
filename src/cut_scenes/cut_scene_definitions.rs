use bevy::prelude::*;
use std::collections::HashMap;

use crate::{
    GameState,
    cut_scenes::{CutScene, CutSceneDescriptor, CutSceneFrame},
    loading::TextureAssets,
};

pub(super) fn plugin(app: &mut App) {
    app.init_state::<CutSceneDefinitionsLoadingState>()
        .init_resource::<CutSceneDefinitions>()
        .add_systems(
            Update,
            define_cut_scenes.run_if(
                not(in_state(GameState::Loading)).and(resource_added::<CutSceneDefinitions>),
            ),
        );
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum CutSceneDefinitionsLoadingState {
    #[default]
    Loading,
    Complete,
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct CutSceneDefinitions(pub HashMap<CutScene, CutSceneDescriptor>);

fn define_cut_scenes(
    mut definitions: ResMut<CutSceneDefinitions>,
    texture_assets: Res<TextureAssets>,
    mut next_state: ResMut<NextState<CutSceneDefinitionsLoadingState>>,
) -> Result {
    let mut definitions_map = HashMap::new();

    definitions_map.insert(
        CutScene::StartA,
        CutSceneDescriptor::new(
            vec![
                CutSceneFrame::new(texture_assets.start_a_01.clone(), Some(0.1..1.0)),
                CutSceneFrame::new(texture_assets.start_a_02.clone(), Some(0.1..1.0)),
            ],
            true,
            Some("StartA".to_string()),
            GameState::ChooseGame,
        ),
    );
    definitions_map.insert(
        CutScene::MiddleA,
        CutSceneDescriptor::new(
            vec![CutSceneFrame::new(texture_assets.middle_a_01.clone(), None)],
            false,
            Some("MiddleA".to_string()),
            GameState::CutScene(CutScene::MiddleB),
        ),
    );
    definitions_map.insert(
        CutScene::MiddleB,
        CutSceneDescriptor::new(
            vec![CutSceneFrame::new(texture_assets.middle_b_01.clone(), None)],
            false,
            Some("MiddleB".to_string()),
            GameState::CutScene(CutScene::MiddleC),
        ),
    );
    definitions_map.insert(
        CutScene::MiddleC,
        CutSceneDescriptor::new(
            vec![
                CutSceneFrame::new(texture_assets.middle_c_01.clone(), Some(1.0..1.0)),
                CutSceneFrame::new(texture_assets.middle_c_02.clone(), Some(1.0..1.0)),
            ],
            false,
            None,
            GameState::CutScene(CutScene::MiddleD),
        ),
    );
    definitions_map.insert(
        CutScene::MiddleD,
        CutSceneDescriptor::new(
            vec![
                CutSceneFrame::new(texture_assets.middle_d_01.clone(), Some(0.1..0.3)),
                CutSceneFrame::new(texture_assets.middle_d_02.clone(), Some(0.1..0.3)),
                CutSceneFrame::new(texture_assets.middle_d_03.clone(), Some(0.1..0.3)),
            ],
            true,
            Some("MiddleD".to_string()),
            GameState::CutScene(CutScene::MiddleE),
        ),
    );
    definitions_map.insert(
        CutScene::MiddleE,
        CutSceneDescriptor::new(
            vec![
                CutSceneFrame::new(texture_assets.middle_e_01.clone(), Some(1.0..1.0)),
                CutSceneFrame::new(texture_assets.middle_e_02.clone(), Some(1.0..1.0)),
                CutSceneFrame::new(texture_assets.middle_e_03.clone(), Some(0.5..0.5)),
                CutSceneFrame::new(texture_assets.middle_e_04.clone(), Some(0.5..0.5)),
            ],
            false,
            None,
            GameState::CutScene(CutScene::MiddleF),
        ),
    );
    definitions_map.insert(
        CutScene::MiddleF,
        CutSceneDescriptor::new(
            vec![CutSceneFrame::new(texture_assets.middle_f_01.clone(), None)],
            false,
            Some("MiddleF".to_string()),
            GameState::CutScene(CutScene::MiddleG),
        ),
    );
    definitions_map.insert(
        CutScene::MiddleG,
        CutSceneDescriptor::new(
            vec![CutSceneFrame::new(texture_assets.middle_g_01.clone(), None)],
            false,
            Some("MiddleG".to_string()),
            GameState::CutScene(CutScene::MiddleH),
        ),
    );
    definitions_map.insert(
        CutScene::MiddleH,
        CutSceneDescriptor::new(
            vec![CutSceneFrame::new(texture_assets.middle_h_01.clone(), None)],
            false,
            Some("MiddleH".to_string()),
            GameState::CutScene(CutScene::MiddleI),
        ),
    );
    definitions_map.insert(
        CutScene::MiddleI,
        CutSceneDescriptor::new(
            vec![CutSceneFrame::new(texture_assets.middle_g_01.clone(), None)],
            false,
            Some("MiddleI".to_string()),
            GameState::ChooseGame,
        ),
    );
    definitions_map.insert(
        CutScene::EndA,
        CutSceneDescriptor::new(
            vec![CutSceneFrame::new(texture_assets.end_a_01.clone(), None)],
            false,
            Some("EndA".to_string()),
            GameState::CutScene(CutScene::EndB),
        ),
    );
    definitions_map.insert(
        CutScene::EndB,
        CutSceneDescriptor::new(
            vec![
                CutSceneFrame::new(texture_assets.end_b_01.clone(), Some(1.0..1.0)),
                CutSceneFrame::new(texture_assets.end_b_02.clone(), Some(2.0..2.0)),
            ],
            false,
            None,
            GameState::CutScene(CutScene::EndC),
        ),
    );
    definitions_map.insert(
        CutScene::EndC,
        CutSceneDescriptor::new(
            vec![CutSceneFrame::new(texture_assets.end_b_02.clone(), None)],
            false,
            Some("EndC".to_string()),
            GameState::CutScene(CutScene::EndD),
        ),
    );
    definitions_map.insert(
        CutScene::EndD,
        CutSceneDescriptor::new(
            vec![
                CutSceneFrame::new(texture_assets.end_b_02.clone(), Some(1.0..1.0)),
                CutSceneFrame::new(texture_assets.end_d_02.clone(), Some(1.5..1.5)),
                CutSceneFrame::new(texture_assets.end_d_03.clone(), Some(1.0..1.0)),
                CutSceneFrame::new(texture_assets.end_d_04.clone(), Some(1.0..1.0)),
                CutSceneFrame::new(texture_assets.end_d_05.clone(), Some(1.0..1.0)),
                CutSceneFrame::new(texture_assets.end_d_06.clone(), Some(1.0..1.0)),
            ],
            false,
            None,
            GameState::CutScene(CutScene::EndE),
        ),
    );
    definitions_map.insert(
        CutScene::EndE,
        CutSceneDescriptor::new(
            vec![CutSceneFrame::new(texture_assets.end_e_01.clone(), None)],
            false,
            Some("EndE".to_string()),
            GameState::TheEnd,
        ),
    );

    definitions.0 = definitions_map;

    next_state.set(CutSceneDefinitionsLoadingState::Complete);

    Ok(())
}
