use std::path::Path;

use crate::lab::LabContext;

mod lab;
mod util;

fn main() {
    let context = LabContext::load(
        Path::new("assets/prob_04.csv"),
        Path::new("assets/table_04.csv"),
    );

    println!("{}", context);
    println!("{}", context.deterministic_decision(2));
}
