use crate::matrix::Matrix;
use num_traits::Float;

impl<K: Float> Matrix<K> {
    pub fn gaussian_elimination(&self) -> Matrix<K> {
        let mut new = self.clone();
        let mut piv_col = 0;
        let mut piv_row = 0;
        while piv_row < new.rows && piv_col < new.cols {
            if let Some(max_row) = (piv_row..self.rows)
                .max_by(|&a, &b| {
                    let val = |&r: &usize| new.data[r * self.cols + piv_col].abs();
                    val(&a).partial_cmp(&val(&b)).unwrap()
                })
                .filter(|&f| new.data[f * new.cols + piv_col].abs() > K::epsilon())
            {
                new.switch_rows(max_row, piv_row);
                let pivot_val = new.row_col_val(piv_row, piv_col);
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
            }
            piv_col += 1;
        }
        new
    }
    pub fn rank(&mut self) -> usize {
        let mut new = self.clone();
        let mut piv_col = 0;
        let mut piv_row = 0;
        while piv_row < new.rows && piv_col < new.cols {
            if let Some(max_row) = (piv_row..self.rows)
                .max_by(|&a, &b| {
                    let val = |&r: &usize| new.data[r * self.cols + piv_col].abs();
                    val(&a).partial_cmp(&val(&b)).unwrap()
                })
                .filter(|&f| new.data[f * new.cols + piv_col].abs() > K::epsilon())
            {
                new.switch_rows(max_row, piv_row);
                let pivot_val = new.row_col_val(piv_row, piv_col);
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
            }
            piv_col += 1;
        }
        piv_row
    }
}
