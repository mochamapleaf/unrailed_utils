use unrailed_utils::*;
use unrailed_utils::unrailed_defs::TerrainType;
use unrailed_utils::unrailed_seed::UnrailedSeed;

fn test_terrain_generator(){
    let seed = UnrailedSeed::from_str("+pbHigU")
        .expect("Failed to decode seed");
    let mut terrain_generator = TerrainGenerator::new(seed);
    terrain_generator.take(10).for_each(|x| println!("{:?}", x));
}

fn main() {
    test_terrain_generator();
}