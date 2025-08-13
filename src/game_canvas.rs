use bevy::prelude::*;

/// Component that designates the area that sub-games will use to mimic
/// their 4:3 aspect ratios. Child of the main camera to allow arbitrary
/// placement of the game screen.
#[derive(Component, Reflect, Debug, Deref)]
#[reflect(Component)]
pub struct GameCanvas(pub Vec2);

impl GameCanvas {
    pub fn width(&self) -> f32 {
        self.x
    }

    pub fn height(&self) -> f32 {
        self.y
    }
}

#[derive(Bundle, Debug)]
pub struct GameCanvasBundle {
    pub game_canvas: GameCanvas,
    pub transform: Transform,
    pub visibility: InheritedVisibility,
}
