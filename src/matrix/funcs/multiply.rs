use crate::matrix::Matrix;
use crate::vector::Vector;
use num_traits::Float;

impl<K: Float> Matrix<K> {
    pub fn mul_vec(&mut self, vec: Vector<K>) -> Vector<K> {
        assert_eq!(self.cols, vec.size());
        let mut new_val = vec![K::zero(); self.rows];
        for row_idx in 0..self.rows {
            for col_idx in 0..self.cols {
                new_val[row_idx] = self
                    .row_col_val(row_idx, col_idx)
                    .mul_add(vec.data[col_idx], new_val[row_idx]);
            }
        }
        Vector { data: new_val }
    }
    pub fn mul_mat(&mut self, mat: Matrix<K>) -> Matrix<K> {
        assert_eq!(
            self.cols, mat.rows,
            "Error: Need MxN and NxP size matrices for multiplication!"
        );
        let mut new_matrix = vec![K::zero(); self.cols * mat.rows];
        for m in 0..self.rows {
            for p in 0..mat.cols {
                let mut acc = K::zero();
                for n in 0..self.cols {
                    let a = self.row_col_val(m, n);
                    let b = mat.row_col_val(n, p);
                    acc = a.mul_add(b, acc);
                }
                new_matrix[m * mat.cols + p] = acc;
            }
        }
        Matrix {
            data: new_matrix,
            rows: self.rows,
            cols: mat.cols,
        }
    }
}
