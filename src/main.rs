use std::path::Path;

use crate::{
    probabilities::{get_ciphertext_probabilities, get_m_if_c_probabilities},
    tables::{CipherTable, ProbabilityDistributionTables},
};

mod probabilities;
mod tables;

fn main() {
    let ciphertable = CipherTable::new(Path::new("assets/table_04.csv"));
    let distribution = ProbabilityDistributionTables::new(Path::new("assets/prob_04.csv"));

    let (c_probs, m_and_c_probs) = get_ciphertext_probabilities(&ciphertable, &distribution);

    let res = get_m_if_c_probabilities(&m_and_c_probs, &c_probs);

    println!("{:?}", res);
}
