mod unrailed_rng;
mod unrailed_seed;
mod unrailed_defs;
mod rand_selector;

extern crate core;

use core::fmt;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlInputElement, HtmlElement};
use base64::{engine, Engine};
use error_stack::{Context, Report, Result, ResultExt};

use crate::unrailed_rng::UnrailedRng;
use crate::unrailed_defs::*;
use crate::unrailed_seed::UnrailedSeed;
use crate::rand_selector::RandSelector;

struct TerrainGenerator{
    rng: UnrailedRng,
    terrain_pool: RandSelector<TerrainType>,
}

impl TerrainGenerator{
    const TERRAIN_RNG_OFFSET: u64 = 123;
    fn new(seed: UnrailedSeed) -> Self{
        let rng = UnrailedRng::new(seed.val as u64, );
    }
}
impl Iterator for TerrainGenerator{
    type Item = TerrainType;
    fn next(&mut self) -> Option<Self::Item>{
        todo!()
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