
pub const QUICKMODE_RNG_OFFSET: u64 = 2;
pub enum TerrainType{
    Plain,
    Dessart,
    Snow,
    Hell,
    aSg,
    aSH,
    Mars,
    aSI
}

pub enum WagonType{
    GhostWagon,
    SuperChargerWagon,
    TrackWagon,
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
