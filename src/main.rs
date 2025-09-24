use std::path::Path;

#[derive(Debug)]
pub struct ProbabilityDistributionTable {
    open_texts_distribution: [f64; 20],
    keys_distribution: [f64; 20],
}

impl ProbabilityDistributionTable {
    pub fn new(path: &Path) -> Self {
        let reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path(path)
            .unwrap();

        let mut row_iter = reader.into_deserialize::<[f64; 20]>();

        Self {
            open_texts_distribution: row_iter.next().unwrap().unwrap(),
            keys_distribution: row_iter.next().unwrap().unwrap(),
        }
    }
}

pub struct CipherTable([[f64; 20]; 20]);

fn main() {
    println!(
        "{:?}",
        ProbabilityDistributionTable::new(Path::new("assets/prob_04.csv"))
    )
}
