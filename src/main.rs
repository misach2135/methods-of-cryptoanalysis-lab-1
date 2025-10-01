use std::path::Path;

use crate::lab::{
    EvaluatedProbabilities, LabContext, deterministic_decision_matrix, stochastic_decision_matrix,
};

mod lab;
mod util;

fn main() {
    let context = LabContext::load(
        Path::new("assets/prob_04.csv"),
        Path::new("assets/table_04.csv"),
    );

    let probabilities = EvaluatedProbabilities::eval(context);

    println!("{probabilities}");
    println!(
        "Deletrminisic decision function matrix:\n{}",
        deterministic_decision_matrix(&probabilities)
    );

    println!(
        "Stochastic decision function matrix:\n{}",
        stochastic_decision_matrix(&probabilities)
    );
}
