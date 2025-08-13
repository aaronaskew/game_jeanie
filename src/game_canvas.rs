use bevy::prelude::*;

/// Component that designates the area that sub-games will use to mimic
/// their 4:3 aspect ratios. Child of the main camera to allow arbitrary
/// placement of the game screen.
#[derive(Component, Reflect, Debug, Deref)]
#[reflect(Component)]
pub struct GameCanvas(pub Vec2);

#[derive(Bundle, Debug)]
pub struct GameCanvasBundle {
    pub game_canvas: GameCanvas,
    pub child_of: ChildOf,
    pub transform: Transform,
}
