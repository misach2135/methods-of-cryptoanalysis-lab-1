use core::fmt;
use std::{fmt::Display, ops::Deref};

#[derive(Debug)]
pub struct Matrix<T, const R: usize, const C: usize>([[T; C]; R]);

impl<T, const R: usize, const C: usize> Matrix<T, R, C>
where
    T: Default + Clone + Copy,
{
    pub fn new() -> Self {
        Self([[T::default(); C]; R])
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
