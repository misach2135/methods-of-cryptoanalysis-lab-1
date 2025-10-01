use std::{fmt::Display, ops::Deref, path::Path};

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

    fn get_union_probability_of_m_and_k(&self, m_id: u32, k_id: u32) -> f64 {
        self.messages_distribution[m_id as usize] * self.keys_distribution[k_id as usize]
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

        Ok(())
    }
}

pub struct EvaluatedProbabilities {
    context: LabContext,
    ciphertexts_probabilities: Matrix<f64, 1, 20>,
    m_and_c_probabilities: Matrix<f64, 20, 20>,
    m_if_c_probabilities: Matrix<f64, 20, 20>,
}

impl EvaluatedProbabilities {
    pub fn eval(context: LabContext) -> Self {
        let mut ciphertexts_probabilities = [0f64; 20];
        let mut m_and_c_probabilities = [[0f64; 20]; 20];
        let mut m_if_c_probabilities: [[f64; 20]; 20] = [[0f64; 20]; 20];

        for (k_id, row) in context.cipthertable.deref().iter().enumerate() {
            for (m_id, c_id) in row.iter().enumerate() {
                let key_and_m_prob =
                    context.get_union_probability_of_m_and_k(m_id as u32, k_id as u32);

                ciphertexts_probabilities[*c_id as usize] += key_and_m_prob;
                m_and_c_probabilities[m_id][*c_id as usize] += key_and_m_prob;
            }
        }

        for (m_id, row) in m_and_c_probabilities.iter().enumerate() {
            for (c_id, m_and_c_prob) in row.iter().enumerate() {
                m_if_c_probabilities[m_id][c_id] = m_and_c_prob / ciphertexts_probabilities[c_id];
            }
        }

        Self {
            context,
            ciphertexts_probabilities: ciphertexts_probabilities.into(),
            m_and_c_probabilities: m_and_c_probabilities.into(),
            m_if_c_probabilities: m_if_c_probabilities.into(),
        }
    }
}

impl Display for EvaluatedProbabilities {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Context:\n{}", self.context)?;

        writeln!(f, "P(C):\n{}", self.ciphertexts_probabilities)?;
        writeln!(f, "P(M, C):\n{}", self.m_and_c_probabilities)?;
        writeln!(f, "P(M | C):\n{}", self.m_if_c_probabilities)?;

        Ok(())
    }
}

pub fn deterministic_decision_matrix(ctx: &EvaluatedProbabilities) -> Matrix<u32, 1, 20> {
    let mut res = Matrix::default();

    for i in 0..20 {
        res[i] = ctx
            .m_if_c_probabilities
            .iter()
            .map(|x| x[i])
            .enumerate()
            .max_by(|a, b| a.1.total_cmp(&b.1))
            .map(|x| x.0 as u32)
            .unwrap();
    }

    res
}

pub fn stochastic_decision_matrix(ctx: &EvaluatedProbabilities) -> Matrix<f64, 20, 20> {
    let mut res = Matrix::default();

    res
}
