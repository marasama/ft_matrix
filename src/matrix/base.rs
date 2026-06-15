use crate::MATRIX_EPS;

use super::*;
use std::ops::Index;

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

    pub fn add_mat_ref(&self, other: &Matrix<K>) -> Matrix<K>
    where
        K: AddAssign,
    {
        self.size_matcher(other);
        let mut new_data = vec![K::zero(); self.rows * self.cols];
        for i in 0..(self.rows * self.cols) {
            new_data[i] = self.data[i] + other.data[i];
        }
        Matrix {
            data: new_data,
            rows: self.rows,
            cols: self.cols,
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

    pub fn sub_mat_ref(&self, other: &Matrix<K>) -> Matrix<K>
    where
        K: SubAssign,
    {
        self.size_matcher(other);
        let mut new_data = vec![K::zero(); self.rows * self.cols];
        for i in 0..(self.rows * self.cols) {
            new_data[i] = self.data[i] - other.data[i];
        }
        Matrix {
            data: new_data,
            rows: self.rows,
            cols: self.cols,
        }
    }

    pub fn scl(&mut self, scale: K) {
        for a in self.data.iter_mut() {
            *a = *a * scale;
        }
    }

    pub fn scl_mat_ref(&self, scale: K) -> Matrix<K> {
        let mut new_data = vec![K::zero(); self.rows * self.cols];
        for i in 0..(self.rows * self.cols) {
            new_data[i] = self.data[i] * scale;
        }
        Matrix {
            data: new_data,
            rows: self.rows,
            cols: self.cols,
        }
    }

    pub fn empty() -> Matrix<K> {
        Matrix {
            data: vec![],
            rows: 0,
            cols: 0,
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
