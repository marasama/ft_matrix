use std::ops::Mul;

use crate::matrix::Matrix;
use crate::vector::Vector;
use num_traits::Float;

impl<K: Float, const M: usize, const N: usize> Matrix<K, M, N>
where
    [(); M * N]:,
{
    pub fn mul_vec(&self, vec: &Vector<K, N>) -> Vector<K, M> {
        let mut new_val = [K::zero(); M];
        for row_idx in 0..M {
            for col_idx in 0..N {
                new_val[row_idx] = self
                    .row_col_val(row_idx, col_idx)
                    .mul_add(vec.data[col_idx], new_val[row_idx]);
            }
        }
        Vector { data: new_val }
    }
    pub fn mul_mat<const P: usize>(&self, mat: &Matrix<K, N, P>) -> Matrix<K, M, P>
    where
        [(); M * P]:,
        [(); N * P]:,
    {
        let mut new_matrix = [K::zero(); M * P];
        for m in 0..M {
            for p in 0..P {
                let mut acc = K::zero();
                for n in 0..N {
                    let a = self.data[m * N + n];
                    let b = mat.data[n * P + p];
                    acc = a.mul_add(b, acc);
                }
                new_matrix[m * P + p] = acc;
            }
        }
        Matrix { data: new_matrix }
    }
}

impl<K: Float, const M: usize, const N: usize> Mul<&Vector<K, N>> for Matrix<K, M, N>
where
    [(); M * N]:,
{
    type Output = Vector<K, M>;

    fn mul(self, rhs: &Vector<K, N>) -> Self::Output {
        self.mul_vec(&rhs)
    }
}

impl<K: Float, const M: usize, const N: usize, const P: usize> Mul<&Matrix<K, N, P>>
    for Matrix<K, M, N>
where
    [(); M * N]:,
    [(); N * P]:,
    [(); M * P]:,
{
    type Output = Matrix<K, M, P>;

    fn mul(self, rhs: &Matrix<K, N, P>) -> Self::Output {
        self.mul_mat(&rhs)
    }
}
