extern crate core;

use core::fmt;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlInputElement, HtmlElement};
use base64::{engine, Engine};
use error_stack::{Context, Report, Result, ResultExt};

enum TerrainType{
    Plain,
    Dessart,
    Snow,
    Hell,
    aSg,
    aSH,
    Mars,
    aSI
}

enum WagonType{
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

struct UnrailedRng{
    state1: u64,
    state2: u64,
}

impl UnrailedRng{
    fn new(val1: u64, val2: u64) -> Self{
        let mut ret = Self{
            state1: 0,
            state2: 0,
        };
        ret.state2 = (val2 << 1) | 0b1;
        ret.update_state();
        ret.state1 = ret.state1.wrapping_add(val1);
        ret.update_state();
        ret
    }
    fn from_states(state1: u64, state2: u64) -> Self{
        Self{
            state1,
            state2,
        }
    }
    fn from_seed_str(seed: &str) -> Self {
        todo!();
    }
    fn update_state(&mut self){
        self.state1 = self.state1.wrapping_mul(6364136223846793005);
        self.state1 = self.state1.wrapping_add(self.state2);
    }

    fn next_u32(&mut self) -> u32{
        let mut ret: u64 = self.state1;
        self.update_state();
        let tmp1: u32 = ( (ret >> 18 ^ ret) >> 27 ) as u32;
        let tmp2 = ret >> 59;
        let neg_tmp2 =  (!tmp2).wrapping_add(1);
        return (tmp1 >> (tmp2 % 32)) | tmp1 << ( neg_tmp2 % 32);
    }

    fn gen_range(&mut self, range: core::ops::Range<u32>) -> u32{
        if range.start >= range.end { return range.start; }
        let range_size = range.end - range.start;
        let neg_range_size = (!range_size).wrapping_add(1);
        let threshold = neg_range_size % range_size;
        let mut ret = self.next_u32();
        while ret < threshold {  ret = self.next_u32();  }
        (ret % range_size) + range.start
    }

    fn gen_f64(&mut self)-> f64{ self.gen_range(0..1000000) as f64 / 1000000.0 }

    fn gen_bool(&mut self) -> bool{ self.gen_range(0..2) == 1 }

    fn gen_prob(&mut self) -> f32{
        let val = self.next_u32();
        let denominator = f32::from_bits( (127 << 23) | (0x7FFFFF) );
        let numerator = f32::from_bits( (127 << 23) | (0x7FFFFF & val) );
        let tmp = (numerator / denominator).to_bits() - 1;
        return 2.0 * (f32::from_bits(tmp) - 0.5);
    }
}

#[test]
fn test_rng(){
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


enum UnrailedGameDifficulty{
    Easy = 0,
    Medium = 1,
    Hard = 2,
    Extreme = 3,
    Kids = 4,
}

enum UnrailedGameMode{
    Time,
    Versus,
    Sandbox,
    Endless,
    Quick,
}

struct UnrailedSeed{
    pub val: u32,
    pub difficulty: UnrailedGameDifficulty,
    pub mode: UnrailedGameMode,
}

#[derive(Debug)]
struct InvalidArgumentError;

impl fmt::Display for InvalidArgumentError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        f.write_str("invalid argument")
    }
}
impl Context for InvalidArgumentError {}

impl UnrailedSeed{

    fn from_str(seed: &str) -> Result<Self, InvalidArgumentError>{
        let BASE64_ENGINE: base64::engine::GeneralPurpose = base64::engine::GeneralPurpose::new(
            &base64::alphabet::Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+-")
                .expect("invalid alphabet"),
            base64::engine::GeneralPurposeConfig::new()
                .with_decode_allow_trailing_bits(true)
                .with_decode_padding_mode(base64::engine::DecodePaddingMode::Indifferent)
                .with_encode_padding(true)
        );
        //base64 decode
        let decoded = BASE64_ENGINE.decode(seed.as_bytes())
            .change_context(InvalidArgumentError)?;
        let val = u32::from_le_bytes(decoded[0..4].try_into().unwrap());
        let difficulty = match (decoded[4] >> 5){
            0 => UnrailedGameDifficulty::Easy,
            1 => UnrailedGameDifficulty::Medium,
            2 => UnrailedGameDifficulty::Hard,
            3 => UnrailedGameDifficulty::Extreme,
            4 => UnrailedGameDifficulty::Kids,
            _ => return Err(Report::new(InvalidArgumentError).into()),
        };
        let mode = UnrailedGameMode::Time;
        Ok(Self{ val, difficulty, mode})
    }
}
struct RandSelector<T>{
    pool: Vec<T>,
    prob: Vec<f32>,
    rng: UnrailedRng,
}

impl<T> RandSelector<T> {
    fn add(&mut self, item: T) -> &Self{
        self.pool.push(item);
        self.prob.push(1.0);
        self
    }
}

#[wasm_bindgen]
pub fn analyze_seed(){
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let seed_input = document.get_element_by_id("seed").expect("no seed input")
        .dyn_into::<HtmlInputElement>().expect("seed input is not an input");
    let seed_str = seed_input.value();
    web_sys::console::log_1(&JsValue::from_str(&seed_str));
    let seed = UnrailedSeed::from_str(&seed_str).expect("invalid seed");
    let seed_display = document.get_element_by_id("seed_display").expect("no seed display");
    seed_display.set_inner_html(&format!("{:x}", seed.val));
}