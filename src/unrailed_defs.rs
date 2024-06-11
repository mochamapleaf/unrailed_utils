use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

pub const QUICKMODE_RNG_OFFSET: u64 = 2;
pub const RNG_BASE: u64 = 1000000;

#[wasm_bindgen]
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Hash, Serialize, Deserialize)]
pub enum TerrainType{
    aSE = 0, //debug
    Plain = 1, //aSe
    Dessart = 2, //aSF
    Snow = 3, //aSf
    Hell = 4, //aSG
    Space = 5, //aSg
    aSH = 6,
    Mars = 7, //aSh
    aSI = 8
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum WagonType{
    TrackWagon = 0, //apG
    DynamiteWagon = 1, //apg
    TankWagon = 2, //apH
    GhostWagon = 3, //aph
    SuperChargerWagon = 4, //apI
    api = 5, //api
    CompassWagon = 6, //apJ
    LightWagon = 7,
    apK = 8,
    StorageWagon = 9, //apk
    MilkWagon = 10, //apL
    apl = 11,
    ConvertingWagon = 12, //apM
    apm = 13,
    apN = 14,
    BrakeWagon = 15, //apn
    apO = 16,
    CollectorWagon = 17, //apo
    MiningWagon = 18, //apP
    SlotMachineWagon = 19, //apj
    BucketWagon = 20, //apQ
}

#[derive(Debug, PartialEq, Clone, Copy,Serialize, Deserialize)]
pub enum UnrailedGameDifficulty{
    Easy = 0,
    Medium = 1,
    Hard = 2,
    Extreme = 3,
    Kids = 4,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum UnrailedGameMode{
    Tutorial = 0,
    Halloween = 1,
    QuickVS = 2,
    Endless = 3,
    BotsOnly = 4,
    TimeAttack = 5,
    Quick = 6,
    KingOfTheHill = 7,
    Sandbox = 8,
    Challenge = 9,
    QuickVSMerged = 10,
    EndlessRouge = 11,
    DebugPlayground = 12,
    MenuBackground = 13,
    None = 14,
}
