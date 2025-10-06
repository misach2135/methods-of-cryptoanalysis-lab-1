use core::fmt;
use std::{
    fmt::{Debug, Display},
    ops::{Add, Deref, DerefMut, Index, IndexMut},
};

use anyhow::bail;

#[derive(Debug, Clone)]
pub struct Matrix<T, const R: usize, const C: usize>([[T; C]; R]);

impl<T, const R: usize, const C: usize> Matrix<T, R, C>
where
    T: Default + Copy,
{
    pub fn transpose(self) -> Matrix<T, C, R> {
        let mut m = Matrix::default();

        for (i, row) in self.0.into_iter().enumerate() {
            for (j, element) in row.into_iter().enumerate() {
                m.0[j][i] = element;
            }
        }

        m
    }
}

impl<T, const R: usize, const C: usize> Matrix<T, R, C>
where
    T: Clone + Copy,
{
    pub fn row_sum<S>(&self, row: usize) -> S
    where
        T: Into<S>,
        S: Default + Add<Output = S>,
    {
        self.0[row]
            .iter()
            .fold(S::default(), |acc, x| acc + (*x).into())
    }

    pub fn column_sum<S>(&self, column: usize) -> S
    where
        T: Into<S>,
        S: Default + Add<Output = S>,
    {
        self.0
            .iter()
            .map(|x| x[column])
            .fold(S::default(), |acc, x| acc + x.into())
    }
}

impl<T, const R: usize, const C: usize> Default for Matrix<T, R, C>
where
    T: Default,
{
    fn default() -> Self {
        Self(std::array::from_fn(|_| {
            std::array::from_fn(|_| T::default())
        }))
    }
}

impl<T, const R: usize, const C: usize> TryFrom<Vec<[T; C]>> for Matrix<T, R, C>
where
    T: Debug,
{
    type Error = anyhow::Error;

    fn try_from(value: Vec<[T; C]>) -> Result<Self, Self::Error> {
        if value.len() != R {
            bail!("Vector of rows cannot be converted to static Matrix as its rows count mismatch")
        }

        let temp: [[T; C]; R] = value.try_into().unwrap();

        Ok(Self(temp))
    }
}

impl<T, const R: usize, const C: usize> Index<(usize, usize)> for Matrix<T, R, C> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.0[index.0][index.1]
    }
}

impl<T, const C: usize> Index<usize> for Matrix<T, 1, C> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[0][index]
    }
}

impl<T, const C: usize> IndexMut<usize> for Matrix<T, 1, C> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[0][index]
    }
}

impl<T, const R: usize, const C: usize> IndexMut<(usize, usize)> for Matrix<T, R, C> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.0[index.0][index.1]
    }
}

impl<T, const R: usize, const C: usize> From<[[T; C]; R]> for Matrix<T, R, C> {
    fn from(value: [[T; C]; R]) -> Self {
        Self(value)
    }
}

impl<T, const C: usize> From<[T; C]> for Matrix<T, 1, C> {
    fn from(value: [T; C]) -> Self {
        Self([value])
    }
}

impl<T, const R: usize, const C: usize> Deref for Matrix<T, R, C> {
    type Target = [[T; C]; R];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const R: usize, const C: usize> DerefMut for Matrix<T, R, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T, const R: usize, const C: usize> Display for Matrix<T, R, C>
where
    T: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for r in 0..R {
            for c in 0..C {
                write!(f, "{:.4}", self.0[r][c])?;
                if c < C - 1 {
                    write!(f, ",")?;
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}
