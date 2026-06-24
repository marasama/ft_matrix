use crate::matrix::Matrix;
use num_traits::Float;

impl<K: Float, const R: usize> Matrix<K, R, R>
where
    [(); R * R]:,
{
    /// Matrix must be square to calculate determinant!"
    pub fn determinant_with_mat(&mut self) -> (K, Matrix<K, R, R>) {
        let mut new = self.clone();
        let mut det = K::one();
        let mut piv_col = 0;
        let mut piv_row = 0;
        // Kare matris olduğu için satır veya sütun farketmiyor
        while piv_row < R && piv_col < R {
            if let Some(max_row) = (piv_row..R)
                .max_by(|&a, &b| {
                    let val = |&r: &usize| new.data[r * R + piv_col].abs();
                    val(&a).partial_cmp(&val(&b)).unwrap()
                })
                .filter(|&f| new.data[f * R + piv_col].abs() > K::epsilon())
            {
                new.switch_rows(max_row, piv_row);
                if max_row != piv_row {
                    det = -det
                }
                let pivot_val = new.row_col_val(piv_row, piv_col);
                det = pivot_val * det;
                for i in 0..R {
                    new.data[piv_row * R + i] = new.data[piv_row * R + i] / pivot_val;
                }

                for i in 0..R {
                    if i == piv_row {
                        continue;
                    }

                    let factor = new.data[i * R + piv_col];
                    for j in 0..R {
                        let subtrahend = new.data[piv_row * R + j] * factor;
                        new.data[i * R + j] = new.data[i * R + j] - subtrahend;
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
    /// Matrix must be square to calculate determinant!"
    pub fn determinant(&mut self) -> K {
        let mut new = self.clone();
        let mut det = K::one();
        let mut piv_col = 0;
        let mut piv_row = 0;
        // Kare matris olduğu için satır veya sütun farketmiyor
        while piv_row < R && piv_col < R {
            if let Some(max_row) = (piv_row..R)
                .max_by(|&a, &b| {
                    let val = |&r: &usize| new.data[r * R + piv_col].abs();
                    val(&a).partial_cmp(&val(&b)).unwrap()
                })
                .filter(|&f| new.data[f * R + piv_col].abs() > K::epsilon())
            {
                new.switch_rows(max_row, piv_row);
                if max_row != piv_row {
                    det = -det
                }
                let pivot_val = new.row_col_val(piv_row, piv_col);
                det = pivot_val * det;
                for i in 0..R {
                    new.data[piv_row * R + i] = new.data[piv_row * R + i] / pivot_val;
                }

                for i in 0..R {
                    if i == piv_row {
                        continue;
                    }

                    let factor = new.data[i * R + piv_col];
                    for j in 0..R {
                        let subtrahend = new.data[piv_row * R + j] * factor;
                        new.data[i * R + j] = new.data[i * R + j] - subtrahend;
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
