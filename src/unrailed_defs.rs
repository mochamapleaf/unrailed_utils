
pub const QUICKMODE_RNG_OFFSET: u64 = 2;
pub const RNG_BASE: u64 = 1000000;
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
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

pub enum WagonType{
    TrackWagon = 0,
    GhostWagon,
    SuperChargerWagon,
    LightWagon,
    ConvertingWagon,
    CollectorWagon,
    StorageWagon,
    DynamiteWagon,
    MiningWagon,
    BucketWagon,
    MilkWagon,
}

#[derive(Debug, PartialEq)]
pub enum UnrailedGameDifficulty{
    Easy = 0,
    Medium = 1,
    Hard = 2,
    Extreme = 3,
    Kids = 4,
}

#[derive(Debug, PartialEq)]
pub enum UnrailedGameMode{
    Time,
    Versus,
    Sandbox,
    Endless,
    Quick,
}
