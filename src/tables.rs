use std::{ops::Deref, path::Path};

#[derive(Debug)]
pub struct ProbabilityDistributionTables {
    open_texts_distribution: [f64; 20],
    keys_distribution: [f64; 20],
}

impl ProbabilityDistributionTables {
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

    pub fn open_texts_distribution(&self) -> &[f64; 20] {
        &self.open_texts_distribution
    }

    pub fn keys_distribution(&self) -> &[f64; 20] {
        &self.keys_distribution
    }
}

#[derive(Debug)]
pub struct CipherTable([[u32; 20]; 20]);

impl CipherTable {
    pub fn new(path: &Path) -> Self {
        let reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path(path)
            .unwrap();

        let row_iter = reader.into_deserialize::<[u32; 20]>();

        Self(
            row_iter
                .map(|x| x.unwrap())
                .take(20)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        )
    }

    #[inline]
    pub fn get_ciphertext(&self, m: u32, k: u32) -> u32 {
        self.0[k as usize][m as usize]
    }
}

impl Deref for CipherTable {
    type Target = [[u32; 20]; 20];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
