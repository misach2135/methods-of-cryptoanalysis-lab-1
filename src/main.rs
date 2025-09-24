use std::path::Path;

use crate::{probabilities::calculate_ciphertexts_distribution, tables::CipherTable};

mod probabilities;
mod tables;

fn main() {
    let table = CipherTable::new(Path::new("assets/table_04.csv"));

    let distribution = calculate_ciphertexts_distribution(&table);

    println!("{:?}", distribution);
}
