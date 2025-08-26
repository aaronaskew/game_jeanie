use std::ops::DerefMut;

use bevy::prelude::*;

use crate::{
    Game, GameState,
    game_jeanie::{ActiveCheatCode, CheatCode, GameJeanieState},
};

pub(super) fn plugin(app: &mut App) {
    app.add_sub_state::<RacePlaceState>().add_systems(
        OnEnter(RacePlaceState::SetupGlobals),
        setup_race_place_globals,
    );
}

#[derive(SubStates, Default, Clone, Eq, PartialEq, Debug, Hash)]
#[source(GameState = GameState::Playing(Game::RacePlace))]
#[states(scoped_entities)]
pub(crate) enum RacePlaceState {
    #[default]
    SetupGlobals,
    Running,
    GameOver,
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
#[allow(non_snake_case)]
pub struct RacePlaceGlobals {
    pub DESCRIPTION: String,
}

fn setup_race_place_globals(
    active_cheat_code: Res<ActiveCheatCode>,
    mut race_place_globals: ResMut<RacePlaceGlobals>,
    mut next_state: ResMut<NextState<RacePlaceState>>,
    game_jeanie_state: Res<State<GameJeanieState>>,
) -> Result {
    info!("setup_race_place_globals");

    match **game_jeanie_state {
        GameJeanieState::Inactive => {
            *race_place_globals.deref_mut() = get_race_place_globals(&CheatCode::DEFAULT);
        }
        GameJeanieState::Active => {
            let game = active_cheat_code
                .game
                .as_ref()
                .ok_or("No game set in active cheat code")?;

            if *game != Game::RacePlace {
                return Err(format!("active cheat code game set to {:?}", *game).into());
            }

            *race_place_globals.deref_mut() =
                if let Some(cheat_code) = active_cheat_code.cheat_code.as_ref() {
                    get_race_place_globals(cheat_code)
                } else {
                    get_race_place_globals(&CheatCode::DEFAULT)
                };
        }
    }

    #[cfg(debug_assertions)]
    {
        //  *race_place_globals.deref_mut() = get_race_place_globals(&CheatCode::EUOHAKBF);
        // *race_place_globals.deref_mut() = get_race_place_globals(&CheatCode::MXWLYTFM);
    }

    info!("RacePlaceGlobals: {:#?}", race_place_globals);

    next_state.set(RacePlaceState::Running);

    Ok(())
}

fn get_race_place_globals(cheat_code: &CheatCode) -> RacePlaceGlobals {
    match cheat_code {
        _ => RacePlaceGlobals {
            DESCRIPTION: "Default".into(),
        },
    }
}
