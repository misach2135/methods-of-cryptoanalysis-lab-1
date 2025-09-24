use std::ops::Deref;

use crate::tables::CipherTable;

pub(crate) fn calculate_c_m_distribution() -> [[f64; 20]; 20] {
    todo!()
}

pub(crate) fn calculate_ciphertexts_distribution(ciphertable: &CipherTable) -> [f64; 20] {
    let mut distribution = [0f64; 20];

    for row in ciphertable.deref().iter().skip(1) {
        for x in row.iter().skip(1) {
            distribution[*x as usize] += 1.0;
        }
    }

    distribution
        .into_iter()
        .map(|x| x / 20.0) // TODO: Calculate exact number of unique ciphertexts
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}
