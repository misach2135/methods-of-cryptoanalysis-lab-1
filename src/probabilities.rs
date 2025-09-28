use std::ops::Deref;

use crate::tables::{CipherTable, ProbabilityDistributionTables};

pub fn get_ciphertext_probabilities(
    table: &CipherTable,
    distribution: &ProbabilityDistributionTables,
) -> ([f64; 20], [[f64; 20]; 20]) {
    let mut ciphertexts_probabilities = [0f64; 20];
    let mut m_and_c_probabilities = [[0f64; 20]; 20];

    for (k_id, row) in table.deref().iter().enumerate() {
        for (m_id, c_id) in row.iter().enumerate() {
            let key_and_m_prob =
                distribution.get_probability_of_key_and_text(m_id as u32, k_id as u32);

            ciphertexts_probabilities[*c_id as usize] += key_and_m_prob;
            m_and_c_probabilities[m_id][*c_id as usize] += key_and_m_prob;
        }
    }

    (ciphertexts_probabilities, m_and_c_probabilities)
}
