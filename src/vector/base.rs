use super::*;
use crate::{matrix::Matrix, VECTOR_EPS};

impl<K, const N: usize> Vector<K, N>
where
    K: Float,
{
    pub fn new(new_data: [K; N]) -> Vector<K, N> {
        Vector { data: new_data }
    }
    /// Returns the size of Vector
    pub fn size(&self) -> usize {
        N
    }

    pub fn sub(&mut self, other: &Vector<K, N>)
    where
        K: Sub,
    {
        assert_eq!(
            self.data.len(),
            other.data.len(),
            "Error: Vector size mismatch!"
        );
        for i in 0..self.size() {
            self.data[i] = self.data[i] - other.data[i];
        }
    }

    pub fn sub_vec_ref(&self, other: &Vector<K, N>) -> Vector<K, N>
    where
        K: Sub,
    {
        assert_eq!(N, other.data.len(), "Error: Vector size mismatch!");
        let mut new_data = [K::zero(); N];
        for i in 0..self.size() {
            new_data[i] = self.data[i] - other.data[i];
        }
        Vector { data: new_data }
    }

    pub fn add(&mut self, other: &Vector<K, N>)
    where
        K: Add,
    {
        for i in 0..self.size() {
            self.data[i] = self.data[i] + other.data[i];
        }
    }

    pub fn add_vec_ref(&self, other: &Vector<K, N>) -> Vector<K, N>
    where
        K: Add,
    {
        let mut new_data = [K::zero(); N];
        for i in 0..self.size() {
            new_data[i] = self.data[i] + other.data[i];
        }
        Vector { data: new_data }
    }

    pub fn scl(&mut self, other: K) {
        for i in 0..self.size() {
            self.data[i] = self.data[i] * other;
        }
    }

    pub fn scl_vec_ref(&mut self, other: K) -> Vector<K, N> {
        let mut new_data = [K::zero(); N];
        for i in 0..self.size() {
            new_data[i] = self.data[i] * other;
        }
        Vector { data: new_data }
    }

    pub fn empty() -> Vector<K, 0> {
        Vector {
            data: [K::zero(); 0],
        }
    }

    pub fn to_matrix(&self) -> Matrix<K, N, 1>
    where
        [(); N * 1]:,
    {
        let mut new_data = [K::zero(); N * 1];
        new_data.copy_from_slice(&self.data);
        Matrix { data: new_data }
    }
}

/// From Array
impl<K: Float, const N: usize> From<[K; N]> for Vector<K, N> {
    fn from(a: [K; N]) -> Vector<K, N> {
        Vector { data: a }
    }
}

impl<K: Float, const N: usize> PartialEq<Vector<K, N>> for Vector<K, N> {
    fn eq(&self, other: &Vector<K, N>) -> bool {
        for (a, b) in self.data.iter().zip(&other.data) {
            if (*a - *b).abs() > K::from(VECTOR_EPS).unwrap_or(K::epsilon()) {
                return false;
            }
        }
        true
    }
}

impl<K: Float, const N: usize> Add<Vector<K, N>> for Vector<K, N> {
    type Output = Self;
    fn add(self, rhs: Vector<K, N>) -> Self::Output {
        let mut new_data = [K::zero(); N];
        for i in 0..self.size() {
            new_data[i] = self.data[i] + rhs.data[i];
        }
        Vector { data: new_data }
    }
}

impl<K: Float, const N: usize> Add<&Vector<K, N>> for Vector<K, N> {
    type Output = Self;
    fn add(self, rhs: &Vector<K, N>) -> Self::Output {
        let mut new_data = [K::zero(); N];
        for i in 0..self.size() {
            new_data[i] = self.data[i] + rhs.data[i];
        }
        Vector { data: new_data }
    }
}

impl<K: Float, const N: usize> Sub<Vector<K, N>> for Vector<K, N> {
    type Output = Self;
    fn sub(self, rhs: Vector<K, N>) -> Self::Output {
        let mut new_data = [K::zero(); N];
        for i in 0..self.size() {
            new_data[i] = self.data[i] - rhs.data[i];
        }
        Vector { data: new_data }
    }
}

impl<K: Float, const N: usize> Sub<&Vector<K, N>> for Vector<K, N> {
    type Output = Self;
    fn sub(self, rhs: &Vector<K, N>) -> Self::Output {
        let mut new_data = [K::zero(); N];
        for i in 0..self.size() {
            new_data[i] = self.data[i] - rhs.data[i];
        }
        Vector { data: new_data }
    }
}
