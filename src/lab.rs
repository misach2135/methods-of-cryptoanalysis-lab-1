use std::{ops::Deref, path::Path};

use crate::util::Matrix;

pub struct LabContext {
    cipthertable: Matrix<u32, 20, 20>,
    messages_distribution: Matrix<f64, 1, 20>,
    keys_distribution: Matrix<f64, 1, 20>,
}

impl LabContext {
    pub fn load(path_to_probability_table: &Path, path_to_ciphertable: &Path) -> Self {
        let reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path(path_to_probability_table)
            .unwrap();

        let mut prob_table_row_iter = reader.into_deserialize::<[f64; 20]>();

        let reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path(path_to_ciphertable)
            .unwrap();

        let row_iter = reader.into_deserialize::<[u32; 20]>();

        Self {
            messages_distribution: prob_table_row_iter.next().unwrap().unwrap().into(),
            keys_distribution: prob_table_row_iter.next().unwrap().unwrap().into(),
            cipthertable: row_iter
                .map(|x| x.unwrap())
                .take(20)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        }
    }

    pub fn get_union_probability_of_m_and_k(&self, m_id: u32, k_id: u32) -> f64 {
        self.messages_distribution[m_id as usize] * self.keys_distribution[k_id as usize]
    }

    pub fn calc_ciphertext_probabilities(&self) -> (Matrix<f64, 1, 20>, Matrix<f64, 20, 20>) {
        let mut ciphertexts_probabilities = [0f64; 20];
        let mut m_and_c_probabilities = [[0f64; 20]; 20];

        for (k_id, row) in self.cipthertable.deref().iter().enumerate() {
            for (m_id, c_id) in row.iter().enumerate() {
                let key_and_m_prob =
                    self.get_union_probability_of_m_and_k(m_id as u32, k_id as u32);

                ciphertexts_probabilities[*c_id as usize] += key_and_m_prob;
                m_and_c_probabilities[m_id][*c_id as usize] += key_and_m_prob;
            }
        }

        (
            ciphertexts_probabilities.into(),
            m_and_c_probabilities.into(),
        )
    }

    pub fn calc_m_if_c_probabilities(
        m_and_c_probabilities: &[[f64; 20]; 20],
        c_probabilities: &[f64; 20],
    ) -> Matrix<f64, 20, 20> {
        let mut m_if_c_probabilities = [[0f64; 20]; 20];
        for (m_id, row) in m_and_c_probabilities.iter().enumerate() {
            for (c_id, m_and_c_prob) in row.iter().enumerate() {
                m_if_c_probabilities[m_id][c_id] = m_and_c_prob / c_probabilities[c_id];
            }
        }

        m_if_c_probabilities.into()
    }
}
