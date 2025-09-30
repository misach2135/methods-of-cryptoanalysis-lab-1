use std::path::Path;

use crate::lab::{EvaluatedProbabilities, LabContext};

mod lab;
mod util;

fn main() {
    let context = LabContext::load(
        Path::new("assets/prob_04.csv"),
        Path::new("assets/table_04.csv"),
    );

    let probabilities = EvaluatedProbabilities::eval(context);

    println!("{probabilities}");
}
