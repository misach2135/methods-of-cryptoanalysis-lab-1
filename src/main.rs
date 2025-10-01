use std::{
    fs::{DirBuilder, File},
    io::Write,
    path::{Path, PathBuf},
};

use chrono::Utc;

use crate::lab::{
    DecisionFunction, DeterministicDecision, EvaluatedProbabilities, LabContext,
    StochasticDecision, average_loss,
};

mod lab;
mod util;

const OUT_DIR: &str = "out/";
const PROBABILITY_TABLE_PATH: &str = "assets/prob_04.csv";
const CIPHERTABLE_PATH: &str = "assets/table_04.csv";

fn main() -> anyhow::Result<()> {
    let out_dir_name = format!("{}_out", Utc::now().format("%d-%m-%Y_%H-%M-%S"));
    let out_path = Path::new(&std::env::current_dir()?)
        .join(OUT_DIR)
        .join(out_dir_name);

    println!("Out path: {}", out_path.to_str().unwrap());

    DirBuilder::new().recursive(true).create(out_path)?;

    println!("Load probability table from: {PROBABILITY_TABLE_PATH}");
    println!("Load cipthertable from: {CIPHERTABLE_PATH}");

    let context = LabContext::load(
        Path::new(PROBABILITY_TABLE_PATH),
        Path::new(CIPHERTABLE_PATH),
    );
    let context = EvaluatedProbabilities::eval(context);

    let ddf = DeterministicDecision::evaluate(&context);
    let sdf = StochasticDecision::evaluate(&context);

    Ok(())
}
