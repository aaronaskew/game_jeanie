use std::collections::HashMap;

use bevy::prelude::*;

use crate::Game;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<ActiveCheatCode>()
        .init_state::<GameJeanieState>()
        .add_systems(Startup, setup_cheat_codes);
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum GameJeanieState {
    #[default]
    Inactive,
    Active,
}

fn setup_cheat_codes(mut commands: Commands) -> Result {
    let mut cheat_codes: HashMap<Game, CheatCode> = HashMap::new();

    cheat_codes.insert(Game::Pung, CheatCode::MXWLYTFM);
    cheat_codes.insert(Game::Pung, CheatCode::EUOHAKBF);
    cheat_codes.insert(Game::BeefBlastoids, CheatCode::XXPHIHCS);
    cheat_codes.insert(Game::BeefBlastoids, CheatCode::PCLFZZOG);

    commands.insert_resource(CheatCodes(cheat_codes));

    Ok(())
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct ActiveCheatCode {
    pub game: Option<Game>,
    pub cheat_code: Option<CheatCode>,
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
struct CheatCodes(HashMap<Game, CheatCode>);

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Reflect)]
pub enum CheatCode {
    MXWLYTFM,
    EUOHAKBF,
    XXPHIHCS,
    PCLFZZOG,
    // ZCOXATGX,
    // LBWDMDNY,
    // MFLUTNWO,
    // GJVIEEQF,
    // ULDIXDVX,
    // UQORSPUU,
    // PKWRKYZM,
    // YQDFJYZP,
    // VZAGXYPD,
    // AXREKXJG,
    // GBKDLAJQ,
    // VQYFHEOQ,
    // LDGHXNXL,
    // BTDFCCEE,
    // KZLEDNHW,
    // GENFLCXA,
    // BTXPJSQJ,
    // KUYZOXRQ,
    // XTJYJIFN,
    // UCJLTUME,
    // LBHMGCYH,
    // RYDAJZBE,
    // XKGHWIKY,
    // GVRHWLED,
    // COMUDEMR,
    // KTOHLIUU,
    // QFBLMGEZ,
    // OALYFHDV,
    // OYPJHZFY,
    // CPTSMSMY,
    // AKKRSOVA,
    // MFYELPRG,
    // XKGYQFIQ,
    // QBUBUMBZ,
    // DCLTIOMB,
    // YFGKZCZN,
    // CDZITHID,
    // FDWULYOM,
    // DQTVDDWC,
    // GRIRLPNQ,
    // CKQHEOUE,
    // YFCDNRCE,
    // LQVALVFQ,
    // JHNRNYDX,
    // CPUWSUQE,
    // DNDQNXND,
    // QWWFIKLL,
    // HURFNOCS,
    // EGLOQRIC,
    // LEBVFVMR,
    // MDYIAPCN,
    // HTMOJNDN,
    // FNTGWGOJ,
    // KKKNDHVM,
    // JVHOAUUC,
    // PERNJTKP,
    // YIVPTDRA,
    // KIWGXAFC,
    // QTBBGEAB,
    // DLHAEWNE,
    // KNIOFCHU,
    // FNEOXVHE,
    // FUTLBUPN,
    // NIJWDHSO,
    // ERKACTHP,
    // VGAKVKDN,
    // EUREXNKH,
    // YHCWUCGQ,
    // BKDUAMLH,
    // IOPGNHUQ,
    // GUXPNJHN,
    // TVHDPIRX,
    // DWJDQCNY,
    // ZABXOCLI,
    // XCUTJUTR,
    // VQLGKVXT,
    // BJWLXEDR,
    // BFAQAXKO,
    // WAATWNOC,
    // PJRPHINI,
    // DYZPCBQK,
    // NJKDUOJN,
    // FXIBNQKY,
    // CDADYKGR,
    // XOESAQSG,
    // OUHRKGUY,
    // NUIJVOVX,
    // EEJYLHRE,
    // AONLWKNW,
    // NWCXPEHP,
    // GLLKMDJR,
    // EXPYPVNM,
    // VZEEDDJM,
    DEFAULT,
}
