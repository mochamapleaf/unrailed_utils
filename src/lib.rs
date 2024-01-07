use std::sync::Arc;
use crate::unrailed_rng::UnrailedRng;
use crate::unrailed_defs::*;
use crate::unrailed_seed::UnrailedSeed;
use crate::rand_selector::RandSelector;

pub mod unrailed_rng;
pub mod unrailed_seed;
pub mod unrailed_defs;
pub mod rand_selector;
pub mod terrain_generator;
pub mod wagon_generator;

extern crate core;

use wasm_bindgen::prelude::*;
use web_sys::{HtmlInputElement, HtmlElement};
use base64::{engine, Engine};
use error_stack::{Context, Report, Result, ResultExt};

pub struct SeedChecker{
    allowed_terrains: std::collections::HashSet<TerrainType>,
    wagon_list: Vec<WagonType>,
    strict_order: bool,
    cnt_limit: usize,
}

impl SeedChecker{
    pub fn new() -> Self{
        Self{
            allowed_terrains: std::collections::HashSet::new(),
            wagon_list: Vec::new(),
            strict_order: true,
            cnt_limit: usize::MAX,
        }
    }

    //Return Some(cnt) if the criteria is met on cnt-th level
    //otherwise return None if criteria cannot be met within cnt levels
    //For practical use (in quick mode), cnt_limit should be less than 20
    pub fn check_seed(&self, seed: &UnrailedSeed) -> Option<usize>{
        let mut terrain_generator = terrain_generator::TerrainGenerator::new(seed);
        if terrain_generator.terrain_pool.iter().any(|x| !self.allowed_terrains.contains(x)){
            return None;
        }
        let mut wagon_generator = wagon_generator::WagonGenerator::new(seed);
        if self.strict_order{
            let mut cur_target = 0;
            for i in 0..self.cnt_limit{
                let wagon = wagon_generator.next().unwrap();
                if wagon == self.wagon_list[cur_target]{
                    cur_target += 1;
                    if cur_target == self.wagon_list.len(){
                        return Some(i+1);
                    }
                }
            }
            None
        }else{
            None
        }
    }
}


use std::thread;
pub fn find_seed(){
    let mut sc = SeedChecker::new();
    sc.strict_order = true;
    sc.wagon_list = vec![
        WagonType::DynamiteWagon,
        WagonType::LightWagon,
        WagonType::SuperChargerWagon,
        WagonType::SuperChargerWagon,
        WagonType::DynamiteWagon,
        WagonType::DynamiteWagon,
        WagonType::CollectorWagon,
    ];
    sc.cnt_limit = sc.wagon_list.len();
    sc.allowed_terrains.insert(TerrainType::Plain);
    sc.allowed_terrains.insert(TerrainType::Dessart);
    sc.allowed_terrains.insert(TerrainType::Snow);

    let THREAD_CNT = 64;
    let mut threads = Vec::with_capacity(THREAD_CNT);
    let CHUNK_SIZE = u32::MAX / THREAD_CNT as u32;
    let arc_sc = Arc::new(sc);
    for i in 0..64{
        let sc = arc_sc.clone();
        threads.push(thread::spawn(move || {
            for j in i*CHUNK_SIZE..(i+1)*CHUNK_SIZE{
                let seed = UnrailedSeed::new(j, UnrailedGameDifficulty::Easy, UnrailedGameMode::TimeAttack);
                if let Some(n) = sc.check_seed(&seed){
                    println!("{} => {}", seed.to_string(), n);
                }
            }
        }));
    }
    for thread in threads{
        thread.join().unwrap();
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