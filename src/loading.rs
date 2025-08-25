use crate::{GameState, cut_scenes::CutScene};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_enoki::Particle2dEffect;
// use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

/// This plugin loads all assets using \[`AssetLoader`\] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html>
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::CutScene(CutScene::StartA))
                // .load_collection::<AudioAssets>()
                .load_collection::<TextureAssets>()
                .load_collection::<ParticleAssets>()
                .load_collection::<FontAssets>(),
        );
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see <https://github.com/NiklasEi/bevy_asset_loader>)

// #[derive(AssetCollection, Resource)]
// pub struct AudioAssets {
//     #[asset(path = "audio/flying.ogg")]
//     pub _flying: Handle<AudioSource>,
// }

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    // Cut Scene Frames
    #[asset(path = "textures/cutscene_frames/start_A_01.png")]
    pub start_a_01: Handle<Image>,
    #[asset(path = "textures/cutscene_frames/start_A_02.png")]
    pub start_a_02: Handle<Image>,

    #[asset(path = "textures/cutscene_frames/middle_A_01.png")]
    pub middle_a_01: Handle<Image>,
    #[asset(path = "textures/cutscene_frames/middle_B_01.png")]
    pub middle_b_01: Handle<Image>,
    #[asset(path = "textures/cutscene_frames/middle_C_01.png")]
    pub middle_c_01: Handle<Image>,
    #[asset(path = "textures/cutscene_frames/middle_C_02.png")]
    pub middle_c_02: Handle<Image>,
    #[asset(path = "textures/cutscene_frames/middle_D_01.png")]
    pub middle_d_01: Handle<Image>,
    #[asset(path = "textures/cutscene_frames/middle_D_02.png")]
    pub middle_d_02: Handle<Image>,
    #[asset(path = "textures/cutscene_frames/middle_D_03.png")]
    pub middle_d_03: Handle<Image>,
    #[asset(path = "textures/cutscene_frames/middle_E_01.png")]
    pub middle_e_01: Handle<Image>,
    #[asset(path = "textures/cutscene_frames/middle_E_02.png")]
    pub middle_e_02: Handle<Image>,
    #[asset(path = "textures/cutscene_frames/middle_E_03.png")]
    pub middle_e_03: Handle<Image>,
    #[asset(path = "textures/cutscene_frames/middle_E_04.png")]
    pub middle_e_04: Handle<Image>,
    #[asset(path = "textures/cutscene_frames/middle_F_01.png")]
    pub middle_f_01: Handle<Image>,
    #[asset(path = "textures/cutscene_frames/middle_G_01.png")]
    pub middle_g_01: Handle<Image>,
    #[asset(path = "textures/cutscene_frames/middle_H_01.png")]
    pub middle_h_01: Handle<Image>,

    #[asset(path = "textures/cutscene_frames/end_A_01.png")]
    pub end_a_01: Handle<Image>,
    #[asset(path = "textures/cutscene_frames/end_B_01.png")]
    pub end_b_01: Handle<Image>,
    #[asset(path = "textures/cutscene_frames/end_B_02.png")]
    pub end_b_02: Handle<Image>,
    #[asset(path = "textures/cutscene_frames/end_D_02.png")]
    pub end_d_02: Handle<Image>,
    #[asset(path = "textures/cutscene_frames/end_D_03.png")]
    pub end_d_03: Handle<Image>,
    #[asset(path = "textures/cutscene_frames/end_D_04.png")]
    pub end_d_04: Handle<Image>,
    #[asset(path = "textures/cutscene_frames/end_D_05.png")]
    pub end_d_05: Handle<Image>,
    #[asset(path = "textures/cutscene_frames/end_D_06.png")]
    pub end_d_06: Handle<Image>,
    #[asset(path = "textures/cutscene_frames/end_E_01.png")]
    pub end_e_01: Handle<Image>,

    // Choose Game Screen
    #[asset(path = "textures/panel2.png")]
    pub panel2: Handle<Image>,
    #[asset(path = "textures/panel2_blastoid_glow.png")]
    pub panel2_blastoid_glow: Handle<Image>,
    #[asset(path = "textures/panel2_blastoid_seal.png")]
    pub panel2_blastoid_seal: Handle<Image>,
    #[asset(path = "textures/panel2_pung_glow.png")]
    pub panel2_pung_glow: Handle<Image>,
    #[asset(path = "textures/panel2_pung_seal.png")]
    pub panel2_pung_seal: Handle<Image>,
    #[asset(path = "textures/panel2_raceplace_glow.png")]
    pub panel2_raceplace_glow: Handle<Image>,
    #[asset(path = "textures/panel2_raceplace_seal.png")]
    pub panel2_raceplace_seal: Handle<Image>,

    // Gameplay Overlay
    #[asset(path = "textures/panel4.png")]
    pub panel4: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct ParticleAssets {
    #[asset(path = "particles/beef_blastoids_ship_explosion.ron")]
    pub beef_blastoids_ship_explosion: Handle<Particle2dEffect>,
    #[asset(path = "particles/beef_blastoids_beef_explosion.ron")]
    pub beef_blastoids_beef_explosion: Handle<Particle2dEffect>,
}

// RasterForgeRegular-JpBgm.ttf

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/RasterForgeRegular-JpBgm.ttf")]
    pub raster_forge: Handle<Font>,
}
