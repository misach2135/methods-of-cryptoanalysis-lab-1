use std::path::Path;

use crate::tables::CipherTable;

mod probabilities;
mod tables;

fn main() {
    let table = CipherTable::new(Path::new("assets/table_04.csv"));
}
