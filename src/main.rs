use std::path::Path;

use crate::lab::LabContext;

mod lab;
mod util;

fn main() {
    let context = LabContext::load(
        Path::new("assets/prob_04.csv"),
        Path::new("assets/table_04.csv"),
    );

    let (ciphertext_distribution, c_and_m_distribution) = context.calc_ciphertext_probabilities();

    println!("P(C):\n{ciphertext_distribution}");
    println!("P(C, M):\n{c_and_m_distribution}");
}
