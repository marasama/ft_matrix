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

impl<K: Float, const N: usize> MulAdd<K, Vector<K, N>> for Vector<K, N> {
    type Output = Self;
    fn mul_add(self, a: K, b: Vector<K, N>) -> Self::Output {
        let mut new_data = [K::zero(); N];
        for i in 0..N {
            new_data[i] = self.data[i].mul_add(a, b.data[i]);
        }
        Vector { data: new_data }
    }
}

impl<K: Float, const R: usize, const C: usize> MulAdd<K, Matrix<K, R, C>> for Matrix<K, R, C>
where
    [(); R * C]:,
{
    type Output = Self;
    fn mul_add(self, a: K, b: Matrix<K, R, C>) -> Self::Output {
        let mut new_data = [K::zero(); R * C];
        for i in 0..(R * C) {
            new_data[i] = self.data[i].mul_add(a, b.data[i]);
        }
        Matrix { data: new_data }
    }
}
