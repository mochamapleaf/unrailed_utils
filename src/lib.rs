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
    fn from_states(state1: u64, state2: u64) -> Self{
        Self{
            state1,
            state2,
        }
    }
    fn from_seed_str(seed: &str) -> Self {
        todo!();
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
    rng: UnrailedRng,
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