use crate::matrix::Matrix;
use num_traits::Float;

impl<K: Float> Matrix<K> {
    // Creates a new matrix from the selected rows
    pub fn get_rows(&self, rows: &[usize]) -> Matrix<K> {
        for r in rows {
            assert!(
                *r < self.rows,
                "Selected rows must be smaller than the row size of the main matrix!"
            );
        }

        let mut new_data = vec![K::zero(); rows.len() * self.cols];

        for (i, &r) in rows.iter().enumerate() {
            let src_start = r * self.cols;
            let src_end = src_start + self.cols;
            let src_slice = &self.data[src_start..src_end];

            let dest_start = i * self.cols;
            let dest_end = dest_start + self.cols;
            let dest_slice = &mut new_data[dest_start..dest_end];

            dest_slice.copy_from_slice(src_slice);
        }

        Matrix {
            data: new_data,
            rows: rows.len(),
            cols: self.cols,
        }
    }
    // Creates a new matrix from the selected rows
    pub fn get_cols(&self, cols: &[usize]) -> Matrix<K> {
        for &c in cols {
            assert!(
                c < self.cols,
                "Selected columns must be smaller than the column size of the main matrix!"
            );
        }

        let mut new_data = vec![K::zero(); cols.len() * self.rows];

        for (i, &c) in cols.iter().enumerate() {
            for r in 0..self.rows {
                new_data[r * cols.len() + i] = self.data[r * self.cols + c];
            }
        }

        Matrix {
            data: new_data,
            rows: self.rows,
            cols: cols.len(),
        }
    }
}
