use crate::matrix::Matrix;
use crate::vector::Vector;
use num_traits::Float;
use num_traits::MulAdd;
use std::ops::Sub;

pub fn lerp<V>(u: V, v: V, t: f32) -> V
where
    V: MulAdd<f32, V, Output = V> + Sub<Output = V> + Clone,
{
    let u_clone = u.clone();
    (v - u_clone).mul_add(t, u)
}

impl<K: Float> MulAdd<f32, Vector<K>> for Vector<K> {
    type Output = Self;
    fn mul_add(self, a: f32, b: Vector<K>) -> Self::Output {
        assert_eq!(self.size(), b.size(), "Error: Size mismatch!");
        let scalar: K = K::from(a).expect("Error: Conversion error at f32 to K!");
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

impl<K: Float> MulAdd<f32, Matrix<K>> for Matrix<K> {
    type Output = Self;
    fn mul_add(self, a: f32, b: Matrix<K>) -> Self::Output {
        self.size_matcher(&b);
        let scalar: K = K::from(a).expect("Error: Conversion error at f32 to K!");
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
