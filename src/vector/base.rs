use super::*;
use crate::VECTOR_EPS;
use std::{io::Empty, ops::Mul};

impl<K> Vector<K>
where
    K: Float,
{
    pub fn new(new_data: Vec<K>) -> Vector<K> {
        Vector { data: new_data }
    }
    /// Returns the size of Vector
    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn sub(&mut self, other: &Vector<K>)
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

    pub fn sub_ref(&self, other: &Vector<K>) -> Vector<K>
    where
        K: Sub,
    {
        assert_eq!(
            self.data.len(),
            other.data.len(),
            "Error: Vector size mismatch!"
        );
        let mut new_data = vec![K::zero(); self.size()];
        for i in 0..self.size() {
            new_data[i] = self.data[i] - other.data[i];
        }
        Vector { data: new_data }

    }

    pub fn add(&mut self, other: &Vector<K>)
    where
        K: Add,
    {
        assert_eq!(
            self.data.len(),
            other.data.len(),
            "Error: Vector size mismatch!"
        );
        for i in 0..self.size() {
            self.data[i] = self.data[i] + other.data[i];
        }
    }
    
    pub fn add_ref(&self, other: &Vector<K>) -> Vector<K>
    where
        K: Add,
    {
        assert_eq!(
            self.data.len(),
            other.data.len(),
            "Error: Vector size mismatch!"
        );
        let mut new_data = vec![K::zero(); self.size()];
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

    pub fn empty() -> Vector<K> {
        Vector { data: vec![] }
    }
}

/// From Array
impl<K: Float, const N: usize> From<[K; N]> for Vector<K> {
    fn from(a: [K; N]) -> Vector<K> {
        Vector { data: Vec::from(a) }
    }
}

impl<K: Float> PartialEq<Vector<K>> for Vector<K> {
    fn eq(&self, other: &Vector<K>) -> bool {
        for (a, b) in self.data.iter().zip(&other.data) {
            if (*a - *b).abs() > K::from(VECTOR_EPS).unwrap_or(K::epsilon()) {
                return false;
            }
        }
        true
    }
}
