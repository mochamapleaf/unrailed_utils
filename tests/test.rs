#[cfg(test)]

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

use unrailed_seed_analyzer::UnrailedRng;
#[wasm_bindgen_test]
fn test_unrailed_rng(){
    let mut rng = UnrailedRng::new(0x1234567890ABCDEF, 0xFEDCBA0987654321);
    assert_eq!(rng.state1, 0xBC7AB861A376210D);
    assert_eq!(rng.state2, 0xFDB974130ECA8643);


    rng = UnrailedRng::from_states( 0xBC7AB861A376210D, 0xFDB974130ECA8643 );
    let series = [0xa5dfc31e, 0x58e56809, 0xd58f4726, 0xdbdedf1c, 0x501f1659];
    for i in 0..series.len(){
        assert_eq!(rng.next_u32(), series[i]);
    }

    rng = UnrailedRng::from_states( 0xBC7AB861A376210D, 0xFDB974130ECA8643 );
    let series = [0x3f3f863c, 0x3f4ad012, 0x3df47260, 0x3f3dbe38, 0x3e78b2c8];
    for i in 0..series.len(){
        assert_eq!(rng.gen_prob().to_bits(), series[i]);
    }

    rng = UnrailedRng::from_states(0xBC7AB861A376210D, 0xFDB974130ECA8643);
    let series = [18, 58, 6, 59, 57];
    for i in 0..series.len(){
        assert_eq!(rng.gen_range(0..(100+i as u32)), series[i]);
    }
}

use unrailed_seed_analyzer::{UnrailedSeed, UnrailedGameDifficulty, UnrailedGameMode};
#[wasm_bindgen_test]
fn test_seed_decoding(){
    let mut seed = UnrailedSeed::from_str("+pbHigU")
        .expect("Failed to decode seed");
    assert_eq!(seed.val, 0x8ac796fa);
    assert_eq!(seed.difficulty, UnrailedGameDifficulty::Easy);
    assert_eq!(seed.mode, UnrailedGameMode::Time);
}