use crate::GameState;
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
                .continue_to_state(GameState::Menu)
                // .load_collection::<AudioAssets>()
                .load_collection::<TextureAssets>()
                .load_collection::<ParticleAssets>(),
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
    #[asset(path = "textures/panel4.png")]
    pub panel4: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct ParticleAssets {
    #[asset(path = "particles/beef_blastoids_explosion.ron")]
    pub beef_blastoids_explosion: Handle<Particle2dEffect>,
}
