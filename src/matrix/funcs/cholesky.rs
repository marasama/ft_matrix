use std::ops::AddAssign;

use crate::matrix::Matrix;
use num_traits::Float;

impl<K, const R: usize> Matrix<K, R, R>
where
    K: Float + AddAssign,
    [(); R * R]:,
{
    /// Matrix must be square for Cholesky Decomposition
    pub fn cholesky(&self) -> Self {
        let mut new_data = [K::zero(); R * R];

        for r in 0..R {
            for c in 0..=r {
                let mut sum: K = K::zero();
                for b in 0..c {
                    sum += new_data[r * R + b] * new_data[c * R + b];
                }

                if r == c {
                    new_data[r * R + c] = (self.data[r * R + c] - sum).sqrt();
                } else {
                    new_data[r * R + c] =
                        (K::one() / new_data[c * R + c]) * (self.data[r * R + c] - sum);
                }
            }
        }

        Matrix { data: new_data }
    }
}
