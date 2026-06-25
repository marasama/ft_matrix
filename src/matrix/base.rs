use crate::vector::Vector;
use crate::MATRIX_EPS;

use super::*;
use std::ops::{Add, Index, Sub};

impl<K: Float, const C: usize, const R: usize> From<[[K; C]; R]> for Matrix<K, R, C>
where
    [(); R * C]:,
{
    fn from(value: [[K; C]; R]) -> Self {
        let mut data = [K::zero(); R * C];
        data.copy_from_slice(value.as_flattened());
        Matrix { data }
    }
}

impl<K, const R: usize, const C: usize> Matrix<K, R, C>
where
    K: Float,
    [(); R * C]:,
{
    pub fn new(new_data: [K; R * C]) -> Self {
        Matrix { data: new_data }
    }

    pub fn zeros() -> Matrix<K, R, C> {
        Matrix {
            data: [K::zero(); R * C],
        }
    }
    /// Size value function
    /// Returns (rows, cols) in usize
    pub fn size(&self) -> (usize, usize) {
        (R, C)
    }

    pub fn print_size(&self) {
        println!("Rows: {} X Cols: {}", R, C);
    }

    /// Returns the value from row and column index
    pub fn row_col_val(&self, r: usize, c: usize) -> K {
        self.data[C * r + c]
    }

    pub fn add(&mut self, other: &Matrix<K, R, C>)
    where
        K: AddAssign,
    {
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a += *b;
        }
    }

    pub fn add_mat_ref(&self, other: &Matrix<K, R, C>) -> Matrix<K, R, C>
    where
        K: AddAssign,
    {
        let mut new_data = [K::zero(); R * C];
        for i in 0..(R * C) {
            new_data[i] = self.data[i] + other.data[i];
        }
        Matrix { data: new_data }
    }

    pub fn sub(&mut self, other: &Matrix<K, R, C>)
    where
        K: SubAssign,
    {
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a -= *b;
        }
    }

    pub fn sub_mat_ref(&self, other: &Matrix<K, R, C>) -> Matrix<K, R, C>
    where
        K: SubAssign,
    {
        let mut new_data = [K::zero(); R * C];
        for i in 0..(R * C) {
            new_data[i] = self.data[i] - other.data[i];
        }
        Matrix { data: new_data }
    }

    pub fn scl(&mut self, scale: K) {
        for a in self.data.iter_mut() {
            *a = *a * scale;
        }
    }

    pub fn scl_mat_ref(&self, scale: K) -> Matrix<K, R, C> {
        let mut new_data = [K::zero(); R * C];
        for i in 0..(R * C) {
            new_data[i] = self.data[i] * scale;
        }
        Matrix { data: new_data }
    }
}

impl<K: Float, const R: usize, const C: usize> PartialEq<Matrix<K, R, C>> for Matrix<K, R, C>
where
    [(); R * C]:,
{
    fn eq(&self, other: &Matrix<K, R, C>) -> bool {
        for (a, b) in self.data.iter().zip(&other.data) {
            if (*a - *b).abs() > K::from(MATRIX_EPS).unwrap_or(K::epsilon()) {
                return false;
            }
        }
        true
    }
}

impl<K: Float, const R: usize, const C: usize> Index<usize> for Matrix<K, R, C>
where
    [(); R * C]:,
{
    type Output = K;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<K: Float> Matrix<K, 0, 0> {
    pub fn empty() -> Self {
        Matrix { data: [] }
    }
}

impl<K: Float, const R: usize> Matrix<K, R, 1>
where
    [(); R * 1]:,
{
    pub fn to_vector(&self) -> Vector<K, R>
    where
        [(); R * 1]:,
    {
        let mut data = [K::zero(); R];
        data.copy_from_slice(&self.data);
        Vector { data }
    }
}

impl<K: Float, const R: usize, const C: usize> Add<&Matrix<K, R, C>> for Matrix<K, R, C>
where
    [(); R * C]:,
{
    type Output = Self;
    fn add(self, rhs: &Matrix<K, R, C>) -> Self::Output {
        let mut new_data = [K::zero(); R * C];
        for i in 0..(R * C) {
            new_data[i] = self.data[i] + rhs.data[i];
        }
        Matrix { data: new_data }
    }
}
impl<K: Float, const R: usize, const C: usize> Add<Matrix<K, R, C>> for Matrix<K, R, C>
where
    [(); R * C]:,
{
    type Output = Self;
    fn add(self, rhs: Matrix<K, R, C>) -> Self::Output {
        let mut new_data = [K::zero(); R * C];
        for i in 0..(R * C) {
            new_data[i] = self.data[i] + rhs.data[i];
        }
        Matrix { data: new_data }
    }
}
impl<K: Float, const R: usize, const C: usize> Sub<&Matrix<K, R, C>> for Matrix<K, R, C>
where
    [(); R * C]:,
{
    type Output = Self;
    fn sub(self, rhs: &Matrix<K, R, C>) -> Self::Output {
        let mut new_data = [K::zero(); R * C];
        for i in 0..(R * C) {
            new_data[i] = self.data[i] - rhs.data[i];
        }
        Matrix { data: new_data }
    }
}
impl<K: Float, const R: usize, const C: usize> Sub<Matrix<K, R, C>> for Matrix<K, R, C>
where
    [(); R * C]:,
{
    type Output = Self;
    fn sub(self, rhs: Matrix<K, R, C>) -> Self::Output {
        let mut new_data = [K::zero(); R * C];
        for i in 0..(R * C) {
            new_data[i] = self.data[i] - rhs.data[i];
        }
        Matrix { data: new_data }
    }
}
