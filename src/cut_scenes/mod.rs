use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CutScene {
    Intro,
    Middle,
    Closing,
}
