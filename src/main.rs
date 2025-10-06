use std::{
    fs::{DirBuilder, File},
    io::Write,
    path::Path,
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

    DirBuilder::new().recursive(true).create(&out_path)?;

    println!("Load probability table from: {PROBABILITY_TABLE_PATH}");
    println!("Load cipthertable from: {CIPHERTABLE_PATH}");

    let context = EvaluatedProbabilities::eval(LabContext::load(
        Path::new(PROBABILITY_TABLE_PATH),
        Path::new(CIPHERTABLE_PATH),
    ));

    let mut file = File::create_new(out_path.join("c_distribution.csv"))?;
    writeln!(file, "{}", context.get_ciphertexts_probabilities())?;

    let mut file = File::create_new(out_path.join("m_and_c_distribution.csv"))?;
    writeln!(file, "{}", context.get_m_and_c_probabilities())?;

    let mut file = File::create_new(out_path.join("m_if_c_distribution.csv"))?;
    writeln!(file, "{}", context.get_m_if_c_probabilities())?;

    let ddf = DeterministicDecision::evaluate(&context);

    let mut file = File::create_new(out_path.join("ddf.csv"))?;
    writeln!(file, "{}", ddf.get_decision())?;

    let sdf = StochasticDecision::evaluate(&context);

    let mut file = File::create_new(out_path.join("sdf.csv"))?;
    writeln!(file, "{}", sdf.get_decision())?;

    let mut file = File::create_new(out_path.join("average_losses.txt"))?;
    writeln!(
        file,
        "DDF: {}\nSDF: {}",
        average_loss(&context, &ddf),
        average_loss(&context, &sdf)
    )?;

    Ok(())
}
