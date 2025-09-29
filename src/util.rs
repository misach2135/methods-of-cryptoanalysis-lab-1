use core::fmt;
use std::{
    fmt::{Debug, Display},
    ops::{Add, Deref, Index},
};

use anyhow::bail;

#[derive(Debug)]
pub struct Matrix<T, const R: usize, const C: usize>([[T; C]; R]);

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
            .fold(S::default(), |acc, x| acc + x.clone().into())
    }

    pub fn column_sum<S>(&self, column: usize) -> S
    where
        T: Into<S>,
        S: Default + Add<Output = S>,
    {
        self.0
            .iter()
            .map(|x| x[column])
            .fold(S::default(), |acc, x| acc + x.clone().into())
    }
}

impl<T, const R: usize, const C: usize> Default for Matrix<T, R, C>
where
    T: Default + Clone + Copy,
{
    fn default() -> Self {
        Self([[T::default(); C]; R])
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

impl<T, const R: usize, const C: usize> Display for Matrix<T, R, C>
where
    T: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cell_width = 8;

        writeln!(
            f,
            "┌{}┐",
            (0..C)
                .map(|_| "─".repeat(cell_width))
                .collect::<Vec<_>>()
                .join("┬")
        )?;

        for r in 0..R {
            write!(f, "│")?;
            for c in 0..C {
                write!(
                    f,
                    "{:^width$}│",
                    format!("{:.4}", self.0[r][c]),
                    width = cell_width
                )?;
            }
            writeln!(f)?;

            if r < R - 1 {
                writeln!(
                    f,
                    "├{}┤",
                    (0..C)
                        .map(|_| "─".repeat(cell_width))
                        .collect::<Vec<_>>()
                        .join("┼")
                )?;
            }
        }

        writeln!(
            f,
            "└{}┘",
            (0..C)
                .map(|_| "─".repeat(cell_width))
                .collect::<Vec<_>>()
                .join("┴")
        )?;

        Ok(())
    }
}
