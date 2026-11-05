use crate::matrix::Matrix;
use crate::vector::Vector;
use num_traits::Float;
use num_traits::MulAdd;
use std::ops::Sub;

pub fn lerp<V, K>(u: V, v: V, t: K) -> V
where
    V: MulAdd<K, V, Output = V> + Sub<Output = V> + Clone,
    K: Float,
{
    (v - u.clone()).mul_add(t, u)
}

impl<K: Float> MulAdd<K, Vector<K>> for Vector<K> {
    type Output = Self;
    fn mul_add(self, a: K, b: Vector<K>) -> Self::Output {
        assert_eq!(self.size(), b.size(), "Error: Size mismatch!");
        let scalar: K = a;
        Vector {
            data: self
                .data
                .iter()
                .zip(b.data.iter())
                .map(|(a, b)| a.mul_add(scalar, *b))
                .collect(),
        }
    }
}

impl<K: Float> MulAdd<K, Matrix<K>> for Matrix<K> {
    type Output = Self;
    fn mul_add(self, a: K, b: Matrix<K>) -> Self::Output {
        self.size_matcher(&b);
        let scalar: K = a;
        Matrix {
            data: self
                .data
                .iter()
                .zip(b.data.iter())
                .map(|(a, b)| a.mul_add(scalar, *b))
                .collect(),
            rows: self.rows,
            cols: self.cols,
        }
    }
}
