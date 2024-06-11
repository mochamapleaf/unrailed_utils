use clap::{Arg, Parser};
use serde::{Deserialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use unrailed_utils::unrailed_defs::*;

use indicatif::{ProgressBar, ProgressStyle};
use std::thread;
use unrailed_utils::unrailed_seed::UnrailedSeed;

#[derive(Debug, Deserialize)]
struct Config {
    wagon_list: Vec<WagonType>,
    allowed_terrains: Vec<TerrainType>,
    difficulty: UnrailedGameDifficulty,
    margin: usize,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "Sets the input file to use")]
    input: String,
    #[arg(short, long, help = "Sets the output file to use")]
    output: Option<String>,
}


fn main() {
    let args = Args::parse();
    let input_path = args.input;
    let output_path = args.output;

    let file = File::open(&input_path).expect("Failed to open file");
    let config: Config = serde_yaml::from_reader(file).expect("Failed to deserialize YAML");

    let progress_cur = Arc::new(AtomicUsize::new(0));
    let progress_total = Arc::new(AtomicUsize::new(0));
    let result = Arc::new(Mutex::new(Vec::<(UnrailedSeed, usize)>::new()));

    let pb = ProgressBar::new(100);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})").expect("Failed to set style")
        .progress_chars("#>-"));

    let pb = Arc::new(Mutex::new(pb));

    let handle = thread::spawn({
        let progress_cur = progress_cur.clone();
        let progress_total = progress_total.clone();
        let pb = pb.clone();
        move || {
            while progress_cur.load(Ordering::Relaxed) < progress_total.load(Ordering::Relaxed) {
                pb.lock().unwrap().set_position(progress_cur.load(Ordering::Relaxed) as u64);
                pb.lock().unwrap().set_length(progress_total.load(Ordering::Relaxed) as u64);
                thread::sleep(std::time::Duration::from_millis(500));
            }
            pb.lock().unwrap().finish_with_message("done");
        }
    });

    unrailed_utils::find_seed(&config.allowed_terrains, &config.wagon_list, config.difficulty, config.margin,
                              progress_cur.clone(), progress_total.clone(), result.clone());

    handle.join().unwrap();

    let mut out : Vec<String> = result.lock().unwrap().iter().map(|(seed, cnt)| format!("{} {}", seed.to_string(), cnt)).collect();

    match output_path {
        Some(path) => {
            match File::create(Path::new(&path)) {
                Ok(mut file) => {
                for line in &out {
                    writeln!(file, "{}", line).expect("Failed to write to file");
                }
                },
                Err(e) => {
                    eprintln!("Failed to create file: {}", e);
                    eprintln!("Dumping to stdout");
                    out.iter().for_each(|line| println!("{}", line));
                }
            }
        },
        None => {
            for line in &out {
                println!("{}", line);
            }
        }
    }
}
