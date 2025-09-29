use std::{fmt::Display, ops::Deref, path::Path};

use crate::util::Matrix;

pub struct LabContext {
    cipthertable: Matrix<u32, 20, 20>,
    messages_distribution: Matrix<f64, 1, 20>,
    keys_distribution: Matrix<f64, 1, 20>,
    ciphertext_probabilites: Option<Matrix<f64, 1, 20>>,
    messages_and_ciphertexts_probabilities: Option<Matrix<f64, 20, 20>>,
    messages_if_ciphertexts_probabilities: Option<Matrix<f64, 20, 20>>,
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

        let mut context = Self {
            messages_distribution: prob_table_row_iter.next().unwrap().unwrap().into(),
            keys_distribution: prob_table_row_iter.next().unwrap().unwrap().into(),
            cipthertable: row_iter
                .map(|x| x.unwrap())
                .take(20)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
            ciphertext_probabilites: None,
            messages_and_ciphertexts_probabilities: None,
            messages_if_ciphertexts_probabilities: None,
        };

        context.calc_ciphertext_probabilities();
        context.calc_m_if_c_probabilities();

        context
    }

    pub fn get_union_probability_of_m_and_k(&self, m_id: u32, k_id: u32) -> f64 {
        self.messages_distribution[m_id as usize] * self.keys_distribution[k_id as usize]
    }

    pub fn calc_ciphertext_probabilities(&mut self) {
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

        self.ciphertext_probabilites = Some(ciphertexts_probabilities.into());
        self.messages_and_ciphertexts_probabilities = Some(m_and_c_probabilities.into());
    }

    pub fn calc_m_if_c_probabilities(&mut self) {
        if let (Some(ciphertext_probabilites), Some(messages_and_ciphertexts_probabilities)) = (
            &self.ciphertext_probabilites,
            &self.messages_and_ciphertexts_probabilities,
        ) {
            let mut m_if_c_probabilities: [[f64; 20]; 20] = [[0f64; 20]; 20];
            for (m_id, row) in messages_and_ciphertexts_probabilities.iter().enumerate() {
                for (c_id, m_and_c_prob) in row.iter().enumerate() {
                    m_if_c_probabilities[m_id][c_id] = m_and_c_prob / ciphertext_probabilites[c_id];
                }
            }

            self.messages_if_ciphertexts_probabilities = Some(m_if_c_probabilities.into());
        }
    }
}

impl Display for LabContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "P(M):\n{}", self.messages_distribution)?;
        writeln!(
            f,
            "Sum: {:.3}\n",
            self.messages_distribution.row_sum::<f64>(0)
        )?;

        writeln!(f, "P(K):\n{}", self.keys_distribution)?;
        writeln!(f, "Sum: {:.3}\n", self.keys_distribution.row_sum::<f64>(0))?;

        if let Some(ref ciphertext_probabilites) = self.ciphertext_probabilites {
            writeln!(f, "P(C):\n{}", ciphertext_probabilites)?;
            writeln!(f, "Sum: {:.3}\n", ciphertext_probabilites.row_sum::<f64>(0))?;
        }

        if let Some(ref messages_and_ciphertexts_probabilities) =
            self.messages_and_ciphertexts_probabilities
        {
            writeln!(f, "P(M, C):\n{}", messages_and_ciphertexts_probabilities)?;
        }

        if let Some(ref messages_if_ciphertexts_probabilities) =
            self.messages_if_ciphertexts_probabilities
        {
            writeln!(f, "P(M | C):\n{}", messages_if_ciphertexts_probabilities)?;
        }

        Ok(())
    }
}
