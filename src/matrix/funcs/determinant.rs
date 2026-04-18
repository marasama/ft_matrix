use crate::matrix::Matrix;
use num_traits::Float;

impl<K: Float> Matrix<K> {
    pub fn determinant_with_mat(&mut self) -> (K, Matrix<K>) {
        assert_eq!(
            self.rows, self.cols,
            "Error: Matrix must be square to calculate determinant!"
        );
        let mut new = self.clone();
        let mut det = K::one();
        let mut piv_col = 0;
        let mut piv_row = 0;
        // Kare matris olduğu için satır veya sütun farketmiyor
        while piv_row < new.rows && piv_col < new.cols {
            if let Some(max_row) = (piv_row..self.rows)
                .max_by(|&a, &b| {
                    let val = |&r: &usize| new.data[r * self.cols + piv_col].abs();
                    val(&a).partial_cmp(&val(&b)).unwrap()
                })
                .filter(|&f| new.data[f * new.cols + piv_col].abs() > K::epsilon())
            {
                new.switch_rows(max_row, piv_row);
                if max_row != piv_row {
                    det = -det
                }
                let pivot_val = new.row_col_val(piv_row, piv_col);
                det = pivot_val * det;
                for i in 0..new.cols {
                    new.data[piv_row * new.cols + i] = new.data[piv_row * new.cols + i] / pivot_val;
                }

                for i in 0..new.rows {
                    if i == piv_row {
                        continue;
                    }

                    let factor = new.data[i * new.cols + piv_col];
                    for j in 0..new.cols {
                        let subtrahend = new.data[piv_row * new.cols + j] * factor;
                        new.data[i * new.cols + j] = new.data[i * new.cols + j] - subtrahend;
                    }
                }
                piv_row += 1;
            } else {
                return (K::zero(), new);
            }
            piv_col += 1;
        }
        (det, new)
    }
    pub fn determinant(&mut self) -> K {
        assert_eq!(
            self.rows, self.cols,
            "Error: Matrix must be square to calculate determinant!"
        );
        let mut new = self.clone();
        let mut det = K::one();
        let mut piv_col = 0;
        let mut piv_row = 0;
        // Kare matris olduğu için satır veya sütun farketmiyor
        while piv_row < new.rows && piv_col < new.cols {
            if let Some(max_row) = (piv_row..self.rows)
                .max_by(|&a, &b| {
                    let val = |&r: &usize| new.data[r * self.cols + piv_col].abs();
                    val(&a).partial_cmp(&val(&b)).unwrap()
                })
                .filter(|&f| new.data[f * new.cols + piv_col].abs() > K::epsilon())
            {
                new.switch_rows(max_row, piv_row);
                if max_row != piv_row {
                    det = -det
                }
                let pivot_val = new.row_col_val(piv_row, piv_col);
                det = pivot_val * det;
                for i in 0..new.cols {
                    new.data[piv_row * new.cols + i] = new.data[piv_row * new.cols + i] / pivot_val;
                }

                for i in 0..new.rows {
                    if i == piv_row {
                        continue;
                    }

                    let factor = new.data[i * new.cols + piv_col];
                    for j in 0..new.cols {
                        let subtrahend = new.data[piv_row * new.cols + j] * factor;
                        new.data[i * new.cols + j] = new.data[i * new.cols + j] - subtrahend;
                    }
                }
                piv_row += 1;
            } else {
                return K::zero();
            }
            piv_col += 1;
        }
        det
    }
}
