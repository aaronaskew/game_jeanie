use bevy::prelude::*;

use crate::{RootNode, beef_blastoids::BeefBlastoidsState, loading::FontAssets};

pub(super) fn plugin(app: &mut App) {
    // app.add_systems(
    //     OnEnter(BeefBlastoidsState::GameOver { won: true }),
    //     display_win,
    // )
    // .add_systems(
    //     OnEnter(BeefBlastoidsState::GameOver { won: false }),
    //     display_lose,
    // );
}

// fn display_win(
//     mut commands: Commands,
//     root_node: Single<Entity, With<RootNode>>,
//     font_assets: Res<FontAssets>,
// ) {
// }

// fn display_lose() {}

// fn generate_game_over_ui(
//     won: bool,
//     root_node: &Single<Entity, With<RootNode>>,
//     font_assets: &Res<FontAssets>,
// ) {

//     let message = if won {"You win!!"} else {"You lose."};

//     (
//         StateScoped(BeefBlastoidsState::GameOver { .. })
//         Text
//     )

// }
