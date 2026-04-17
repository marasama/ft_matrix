use crate::matrix::Matrix;
use num_traits::Float;

impl<K: Float> Matrix<K> {
    pub fn transpose(&mut self) -> Matrix<K> {
        let mut new_vec = vec![K::zero(); self.data.len()];
        for i in 0..self.rows {
            for j in 0..self.cols {
                new_vec[j * self.rows + i] = self.row_col_val(i, j);
            }
        }
        Matrix {
            data: new_vec,
            rows: self.cols,
            cols: self.rows,
        }
    }
}
