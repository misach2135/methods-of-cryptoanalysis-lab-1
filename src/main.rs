use std::path::Path;

use crate::{
    probabilities::{get_ciphertext_probabilities, get_m_if_c_probabilities},
    tables::{CipherTable, ProbabilityDistributionTables},
    util::Matrix,
};

mod probabilities;
mod tables;
mod util;

fn main() {
    let ciphertable = CipherTable::new(Path::new("assets/table_04.csv"));
    let distribution = ProbabilityDistributionTables::new(Path::new("assets/prob_04.csv"));
}
