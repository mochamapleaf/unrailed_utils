use crate::rand_selector::RandSelector;
use crate::terrain_generator::TerrainGenerator;
use crate::unrailed_defs::{QUICKMODE_RNG_OFFSET, RNG_BASE, TerrainType, WagonType};
use crate::unrailed_rng::UnrailedRng;
use crate::unrailed_seed::UnrailedSeed;

pub struct WagonGenerator{
    rng: UnrailedRng,
    wagon_selector: RandSelector<WagonType>,
    add_on: bool,
}

impl WagonGenerator{

    const INITAL_WAGONS: &'static [WagonType] = &[
        WagonType::DynamiteWagon,
        WagonType::MiningWagon,
        WagonType::BucketWagon,
    ];

    const ADDON_WAGONS: &'static [WagonType] = &[
        WagonType::GhostWagon,
        WagonType::SuperChargerWagon,
        WagonType::TrackWagon,
        WagonType::LightWagon,
        WagonType::ConvertingWagon,
        WagonType::CollectorWagon,
        WagonType::StorageWagon,
        WagonType::DynamiteWagon,
        WagonType::MiningWagon,
        WagonType::BucketWagon,
        WagonType::MilkWagon,
    ];

    const UNIQUE_WAGONS: &'static [WagonType] = &[
        WagonType::CompassWagon,
        WagonType::BucketWagon,
        WagonType::MilkWagon,
        WagonType::ConvertingWagon,
        WagonType::LightWagon,
    ];

    const WAGON_RNG_OFFSET: u64 = 25;
    pub fn new(seed: &UnrailedSeed) -> Self{
        let mut rng = UnrailedRng::new(seed.val as u64, RNG_BASE + QUICKMODE_RNG_OFFSET + Self::WAGON_RNG_OFFSET);
        let mut wagon_selector = RandSelector::from_iter(Self::INITAL_WAGONS.iter().cloned());
        let tg = TerrainGenerator::new(&seed);
        if tg.terrain_pool.contains(&TerrainType::Hell){
            wagon_selector.add(WagonType::MilkWagon);
        }
        Self{
            rng,
            wagon_selector,
            add_on: true,
        }
    }
}
impl Iterator for WagonGenerator{
    type Item = WagonType;
    fn next(&mut self) -> Option<Self::Item>{
        let ret = self.wagon_selector.select(self.rng.gen_prob()).clone();
        if self.add_on{
            self.wagon_selector = RandSelector::from_iter(Self::ADDON_WAGONS.iter().cloned());
            self.add_on = false;
        }
        self.wagon_selector.update_weight(&ret, if Self::UNIQUE_WAGONS.contains(&ret) { 0.0 } else { 0.5 } );
        Some(ret)
    }
}
