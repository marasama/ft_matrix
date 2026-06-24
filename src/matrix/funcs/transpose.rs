use crate::matrix::Matrix;
use num_traits::Float;

impl<K: Float, const R: usize, const C: usize> Matrix<K, R, C>
where
    [(); R * C]:,
    [(); C * R]:,
{
    pub fn transpose(&self) -> Matrix<K, C, R> {
        let mut new_vec = [K::zero(); C * R];
        for i in 0..R {
            for j in 0..C {
                new_vec[j * R + i] = self.row_col_val(i, j);
            }
        }
        Matrix { data: new_vec }
    }
}
