use crate::unrailed_rng::UnrailedRng;
use crate::unrailed_defs::*;
use crate::unrailed_seed::UnrailedSeed;
use crate::rand_selector::RandSelector;

pub mod unrailed_rng;
pub mod unrailed_seed;
pub mod unrailed_defs;
pub mod rand_selector;

extern crate core;

use wasm_bindgen::prelude::*;
use web_sys::{HtmlInputElement, HtmlElement};
use base64::{engine, Engine};
use error_stack::{Context, Report, Result, ResultExt};



pub struct TerrainGenerator{
    terrain_selector: RandSelector<TerrainType>,
    pub terrain_pool: [TerrainType; 3],
    seed_val: u32,
    cnt: usize,
}

impl TerrainGenerator{
    const TERRAIN_RNG_OFFSET: u64 = 123;
    pub fn new(seed: UnrailedSeed) -> Self{
        let mut rng = UnrailedRng::new(seed.val as u64, RNG_BASE + QUICKMODE_RNG_OFFSET + Self::TERRAIN_RNG_OFFSET);
        rng.update_state();
        let mut quickmode_terrains = vec![
            TerrainType::Plain,
            TerrainType::Dessart,
            TerrainType::Snow,
            TerrainType::Hell,
            TerrainType::Space,
            TerrainType::Mars,
        ];
        //choose 3 terrains
        let mut terrain_pool = [TerrainType::Plain; 3];
        for i in 0..3{
            let selected_i = rng.gen_range(0..quickmode_terrains.len() as u32) as usize;
            terrain_pool[i] = quickmode_terrains[selected_i];
            quickmode_terrains.remove(selected_i);
        }
        terrain_pool.sort_unstable();
        //first terrain is added no matter what
        let mut terrain_selector = RandSelector::<TerrainType>::new();
        terrain_selector.add(terrain_pool[0]);
        terrain_pool[1..].iter()
            .filter(|x| **x <= TerrainType::Snow)
            .for_each(|x|{ terrain_selector.add(x.clone()); } );
        Self{terrain_selector, terrain_pool, seed_val: seed.val, cnt: 1}
    }
}

//Each time next() is called, the terrain generated is for the area between station at (cnt*35 - 45)m and station at (cnt*35 - 10)m
//where cnt means this is the cnt-th terrain generated
//example: tg.next() -> 0~25m (first time call next, cnt = 1), tg.skip(20).next() -> 690~725m (21th time call next, cnt = 21)
impl Iterator for TerrainGenerator{
    type Item = TerrainType;
    fn next(&mut self) -> Option<Self::Item>{
        match self.cnt{
            1 => {
                self.cnt += 1;
                return Some(self.terrain_pool[0]);
            },
            2 => {
                if self.terrain_pool.contains(&TerrainType::Hell){
                    self.terrain_selector.add(TerrainType::Hell);
                }
            },
            3 => {
                if self.terrain_pool.contains(&TerrainType::Space){
                    self.terrain_selector.add(TerrainType::Space);
                }
            },
            5 => {
                if self.terrain_pool.contains(&TerrainType::Mars){
                    self.terrain_selector.add(TerrainType::Mars);
                }
            },
            _ => {},
        }
        let mut rng = UnrailedRng::new(self.seed_val as u64, RNG_BASE + QUICKMODE_RNG_OFFSET + self.cnt as u64);
        let ret = self.terrain_selector.select(rng.gen_prob()).clone();
        self.terrain_selector.update_weight(&ret, 0.5);
        self.cnt += 1;
        Some(ret)
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