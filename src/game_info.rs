use crate::core::graphics::gpu_3d::renderer_3d::WidescreenOption;
use crate::settings::{Arm7Emu, SettingId, SettingValue};
use lazy_static::lazy_static;

pub struct SettingRecommendation {
    pub settings: Vec<(SettingId, SettingValue)>,
    pub info: &'static str,
}

lazy_static! {
    static ref MARIO_KART: SettingRecommendation = SettingRecommendation {
        settings: vec![(SettingId::Arm7Emu, SettingValue::Int(Arm7Emu::Hle as _)), (SettingId::Widescreen, SettingValue::Int(WidescreenOption::Both as _))],
        info: "If arm7 emulation is not set to HLE, the game will occasionally freeze. If you are playing the CTGP Nitro rom hack, set it to SoundHLE",
    };
    static ref SOUNDHLE_WIDESCREEN: SettingRecommendation = SettingRecommendation {
        settings: vec![(SettingId::Arm7Emu, SettingValue::Int(Arm7Emu::SoundHle as _)), (SettingId::Widescreen, SettingValue::Int(WidescreenOption::Both as _))],
        info: "",
    };
    static ref HLE_WIDESCREEN: SettingRecommendation = SettingRecommendation {
        settings: vec![(SettingId::Arm7Emu, SettingValue::Int(Arm7Emu::Hle as _)), (SettingId::Widescreen, SettingValue::Int(WidescreenOption::Both as _))],
        info: "",
    };
    static ref HLE_NO_3D_SKIP: SettingRecommendation = SettingRecommendation {
        settings: vec![(SettingId::Arm7Emu, SettingValue::Int(Arm7Emu::Hle as _)), (SettingId::Geometry3DSkip, SettingValue::Bool(false))],
        info: "",
    };
    static ref POKEMON_GEN4: SettingRecommendation = SettingRecommendation {
        settings: vec![(SettingId::Arm7Emu, SettingValue::Int(Arm7Emu::Hle as _)), (SettingId::Widescreen, SettingValue::Int(WidescreenOption::Only3d as _))],
        info: "",
    };
    static ref POKEMON_GEN5: SettingRecommendation = SettingRecommendation {
        settings: vec![(SettingId::Arm7Emu, SettingValue::Int(Arm7Emu::Hle as _)), (SettingId::Widescreen, SettingValue::Int(WidescreenOption::Only3d as _))],
        info: "Watching movie scenes in Pokestar Studios does not work with HLE, set Arm7 Emulation to anything else in this scenario, you can always go back to HLE afterwards for better performance",
    };
    static ref NSMB: SettingRecommendation = SettingRecommendation {
        settings: vec![(SettingId::Arm7Emu, SettingValue::Int(Arm7Emu::Hle as _)), (SettingId::Geometry3DSkip, SettingValue::Bool(false))],
        info: "Mini games don't work with HLE, set to anything else if you want to play them",
    };
    static ref NI_NO_KUNI: SettingRecommendation = SettingRecommendation {
        settings: vec![(SettingId::Arm7Emu, SettingValue::Int(Arm7Emu::Hle as _)), (SettingId::Geometry3DSkip, SettingValue::Bool(false)), (SettingId::Upscale3DFactor, SettingValue::Int(0))],
        info: "",
    };
    static ref DRAGON_QUEST_V: SettingRecommendation = SettingRecommendation {
        settings: vec![(SettingId::Arm7Emu, SettingValue::Int(Arm7Emu::SoundHle as _)), (SettingId::Geometry3DSkip, SettingValue::Bool(false)), (SettingId::Upscale3DFactor, SettingValue::Int(0))],
        info: "",
    };
    pub static ref GAME_INFOS: [(u32, &'static SettingRecommendation); 18] = [
        (0x355659, &DRAGON_QUEST_V), // Dragon Quest V
        (0x394B42, &SOUNDHLE_WIDESCREEN), // Kingdom Hearts Re:coded
        (0x414441, &POKEMON_GEN4), // Pokemon Diamond
        (0x415249, &POKEMON_GEN5), // Pokemon White
        (0x425249, &POKEMON_GEN5), // Pokemon Black
        (0x434D41, &MARIO_KART), // Mario Kart
        (0x443241, &NSMB), // NSMB
        (0x455249, &POKEMON_GEN5), // Pokemon Black 2
        (0x455A41, &HLE_WIDESCREEN), // The Legend of Zelda PH
        (0x474B59, &SOUNDHLE_WIDESCREEN), // Kingdom Hearts 358/2 Days
        (0x484D41, &HLE_WIDESCREEN), // Metroid Prime Hunters
        (0x494B42, &HLE_WIDESCREEN), // The Legend of Zelda ST
        (0x4B5049, &POKEMON_GEN4), // Pokemon HG
        (0x4D4441, &SOUNDHLE_WIDESCREEN), // Animal Crossing Wild World
        (0x514459, &HLE_WIDESCREEN), // Dragon Quest IX
        (0x524E54, &NI_NO_KUNI), // Ni no Kuni
        (0x555043, &POKEMON_GEN4), // Pokemon Platinum
        (0x594543, &HLE_NO_3D_SKIP), // Aliens Infestation
    ];
}

pub fn get_game_info(game_code: u32) -> Option<&'static SettingRecommendation> {
    debug_assert!(GAME_INFOS.is_sorted_by_key(|(game_code, _)| *game_code));
    GAME_INFOS
        .binary_search_by_key(&(game_code & 0xFFFFFF), |(game_code, _)| *game_code)
        .ok()
        .map(|index| GAME_INFOS[index].1)
}
