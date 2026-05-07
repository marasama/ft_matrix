use std::ops::AddAssign;

use crate::matrix::Matrix;
use num_traits::Float;

impl<K: Float + AddAssign> Matrix<K> {
    pub fn cholesky(&self) -> Self {
        assert_eq!(
            self.cols, self.rows,
            "Matrix must be square for Cholesky Decomposition!"
        );
        let mut new_data = vec![K::zero(); self.rows * self.cols];

        let divider = self.data[0].sqrt();

        for r in 0..self.rows {
            new_data[r * self.cols] = self.row_col_val(r, 0) / divider;
            for c in 0..=r {
                let mut sum: K = K::zero();
                for b in 0..c {
                    sum += new_data[r * self.cols + b] * new_data[c * self.cols + b];
                }

                if r == c {
                    new_data[r * self.cols + c] = (self.data[r * self.cols + c] - sum).sqrt();
                } else {
                    new_data[r * self.cols + c] = (K::one() / new_data[c * self.cols + c])
                        * (self.data[r * self.cols + c] - sum);
                }
            }
        }

        Matrix {
            data: new_data,
            rows: self.rows,
            cols: self.cols,
        }
    }
}
