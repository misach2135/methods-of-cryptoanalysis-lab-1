use std::path::Path;

use crate::tables::{CipherTable, ProbabilityDistributionTables};

mod probabilities;
mod tables;

fn main() {
    let ciphertable = CipherTable::new(Path::new("assets/table_04.csv"));
    let distribution = ProbabilityDistributionTables::new(Path::new("assets/prob_04.csv"));
}
