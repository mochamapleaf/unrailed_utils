use std::sync::Arc;
use std::sync::atomic::{Ordering, AtomicUsize};
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

use base64::{engine, Engine};
use error_stack::{Context, Report, Result, ResultExt};

pub struct SeedChecker{
    allowed_terrains: std::collections::HashSet<TerrainType>,
    wagon_list: Vec<WagonType>,
    strict_order: bool,
    cnt_limit: usize,
}

impl SeedChecker {
    pub fn new() -> SeedChecker {
        SeedChecker {
            allowed_terrains: std::collections::HashSet::new(),
            wagon_list: Vec::new(),
            strict_order: true,
            cnt_limit: usize::MAX,
        }
    }

    pub fn add_allowed_terrain(&mut self, terrain: TerrainType) {
        self.allowed_terrains.insert(terrain);
    }

    pub fn add_wagon(&mut self, wagon: WagonType) {
        self.wagon_list.push(wagon);
    }

    pub fn set_strict_order(&mut self, strict_order: bool) {
        self.strict_order = strict_order;
    }

    pub fn set_cnt_limit(&mut self, cnt_limit: usize) {
        self.cnt_limit = cnt_limit;
    }

    pub fn check_seed(&self, seed: &UnrailedSeed) -> Option<usize> {
        let mut terrain_generator = terrain_generator::TerrainGenerator::new(seed);
        if terrain_generator.terrain_pool.iter().any(|x| !self.allowed_terrains.contains(x)) {
            return None;
        }
        let mut wagon_generator = wagon_generator::WagonGenerator::new(seed);
        if self.strict_order {
            let mut cur_target = 0;
            for i in 0..self.cnt_limit {
                let wagon = wagon_generator.next().unwrap();
                if wagon == self.wagon_list[cur_target] {
                    cur_target += 1;
                    if cur_target == self.wagon_list.len() {
                        return Some(i + 1);
                    }
                }
            }
            None
        } else {
            None
        }
    }
}


use std::sync::Mutex;
use std::thread;
use wasm_bindgen::prelude::*;


pub fn find_seed(
    allowed_terrains: &[TerrainType],
    target_wagon: &[WagonType],
    difficulty: UnrailedGameDifficulty,
    wagon_margin: usize,
    progress_current: Arc<AtomicUsize>,
    progress_total: Arc<AtomicUsize>,
    output: Arc<Mutex<Vec<(UnrailedSeed, usize)>>>
) {
    let mut sc = SeedChecker::new();
    sc.strict_order = true;
    sc.wagon_list = Vec::from(target_wagon);
    sc.cnt_limit = sc.wagon_list.len() + wagon_margin;
    allowed_terrains.iter().for_each(|&terrain| {
        sc.allowed_terrains.insert(terrain);
    });

    let THREAD_CNT = 64;
    let mut threads = Vec::with_capacity(THREAD_CNT);
    progress_total.store(u32::MAX as usize, Ordering::SeqCst);
    let CHUNK_SIZE = u32::MAX / THREAD_CNT as u32;
    let arc_sc = Arc::new(sc);

    for i in 0..64 {
        let sc = arc_sc.clone();
        let output_vec = output.clone();
        let progress = progress_current.clone();

        threads.push(thread::spawn(move || {
            for j in (i * CHUNK_SIZE)..((i + 1) * CHUNK_SIZE) {
                let seed = UnrailedSeed::new(j, difficulty, UnrailedGameMode::TimeAttack);
                if let Some(n) = sc.check_seed(&seed) {
                    let mut output = output_vec.lock().unwrap();
                    let idx = output.binary_search_by_key(&n, |&(_, v)| v).unwrap_or_else(|e| e);
                    (*output).insert(idx, (seed.clone(), n));
                }
                progress.fetch_add(1, Ordering::Relaxed);
            }
        }));
    }

    for thread in threads {
        thread.join().unwrap();
    }
    progress_current.store(progress_total.load(Ordering::SeqCst), Ordering::SeqCst);
}
