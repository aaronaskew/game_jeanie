mod game_over;
mod scoreboard;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(scoreboard::plugin)
        .add_plugins(game_over::plugin);
}
