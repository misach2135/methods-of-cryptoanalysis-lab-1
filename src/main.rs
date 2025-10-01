use std::path::Path;

use crate::lab::{
    DecisionFunction, DeterministicDecision, EvaluatedProbabilities, LabContext,
    StochasticDecision, average_loss,
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

    let ddf = DeterministicDecision::evaluate(&probabilities);
    let sdf = StochasticDecision::evaluate(&probabilities);

    println!(
        "Deletrminisic decision function matrix:\n{}",
        ddf.get_decision()
    );

    println!(
        "Stochastic decision function matrix:\n{}",
        sdf.get_decision()
    );

    println!("Average DDF loss: {}", average_loss(&probabilities, &ddf));
    println!("Average SDF loss: {}", average_loss(&probabilities, &sdf));
}
