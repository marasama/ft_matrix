use crate::MATRIX_EPS;

use super::*;
use std::ops::{Add, Index, Mul, Sub};

impl<K: Float, const C: usize, const R: usize> From<[[K; C]; R]> for Matrix<K> {
    fn from(value: [[K; C]; R]) -> Self {
        let new_vec: Vec<K> = value.into_iter().flatten().collect();
        Matrix {
            data: new_vec.to_owned(),
            rows: R,
            cols: C,
        }
    }
}

impl<K> Add<Self> for Matrix<K>
where
    K: Float,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self.size_matcher(&rhs);
        Matrix {
            data: self
                .data
                .iter()
                .zip(rhs.data.iter())
                .map(|(a, b)| *a + *b)
                .collect(),
            rows: self.rows,
            cols: self.cols,
        }
    }
}

impl<K> Sub<Self> for Matrix<K>
where
    K: Float,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self.size_matcher(&rhs);
        Matrix {
            data: self
                .data
                .iter()
                .zip(rhs.data.iter())
                .map(|(a, b)| *a - *b)
                .collect(),
            rows: self.rows,
            cols: self.cols,
        }
    }
}

impl<K> Mul<Self> for Matrix<K>
where
    K: Mul + Float,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        self.size_matcher(&rhs);
        Matrix {
            data: self
                .data
                .iter()
                .zip(rhs.data.iter())
                .map(|(a, b)| *a * *b)
                .collect(),
            rows: self.rows,
            cols: self.cols,
        }
    }
}

impl<K> Matrix<K>
where
    K: Float,
{
    pub fn new(new_data: Vec<K>, r: usize, c: usize) -> Self {
        assert!(
            new_data.len() == r * c,
            "Error: Row and column size mismatch"
        );
        Matrix {
            data: new_data,
            rows: r,
            cols: c,
        }
    }
    /// Size value function
    /// Returns (rows, cols) in usize
    pub fn size(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn print_size(&self) {
        println!("Rows: {} X Cols: {}", self.rows, self.cols);
    }

    pub fn size_matcher(&self, other: &Matrix<K>) {
        assert_eq!(self.data.len(), other.data.len(), "Error: Size mismatch");
        assert_eq!(self.rows, other.rows, "Error: Row size mismatch");
        assert_eq!(self.cols, other.cols, "Error: Columns size mismatch");
    }

    pub fn size_checker(&self, other: &Matrix<K>) -> bool {
        if self.data.len() != other.data.len() {
            return false;
        }
        if self.rows != other.rows {
            return false;
        }
        if self.cols != other.cols {
            return false;
        }
        true
    }

    /// Returns the value from row and column index
    pub fn row_col_val(&self, r: usize, c: usize) -> K {
        assert!(self.rows > r);
        assert!(self.cols > c);
        self.data[self.cols * r + c]
    }

    pub fn add(&mut self, other: &Matrix<K>)
    where
        K: AddAssign,
    {
        self.size_matcher(other);
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a += *b;
        }
    }

    pub fn sub(&mut self, other: &Matrix<K>)
    where
        K: SubAssign,
    {
        self.size_matcher(other);
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a -= *b;
        }
    }

    pub fn scl(&mut self, scale: K) {
        for a in self.data.iter_mut() {
            *a = *a * scale;
        }
    }
}

impl<K: Float> PartialEq<Matrix<K>> for Matrix<K> {
    fn eq(&self, other: &Matrix<K>) -> bool {
        for (a, b) in self.data.iter().zip(&other.data) {
            if (*a - *b).abs() > K::from(MATRIX_EPS).unwrap_or(K::epsilon()) {
                return false;
            }
        }
        true
    }
}

impl<K: Float> Index<usize> for Matrix<K> {
    type Output = K;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}
