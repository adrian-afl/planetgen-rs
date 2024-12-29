mod base_icosphere;
mod cli_args;
mod cubemap_data;
mod erosion;
mod generate_icosphere;
mod generate_terrain;
mod generate_water;
mod json_input;
mod math_util;
mod noise;
mod random;

use crate::cli_args::CLIArgs;
use crate::generate_terrain::generate_terrain;
use crate::generate_water::generate_water;
use crate::json_input::parse_input_data;
use clap::Parser;
use std::fs;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::time::Instant;

fn main() {
    let cli_args = CLIArgs::parse();
    let input_json = fs::read_to_string(cli_args.input).expect("Failed to to read the input file");
    let input = parse_input_data(&*input_json);

    let start = Instant::now();

    generate_water(&input);
    generate_terrain(&input);

    let duration = start.elapsed();
    println!("Generation finished in: {:?}", duration);
}
