use crate::matrix::Matrix;
use num_traits::Float;

impl<K: Float> Matrix<K> {
    pub fn determinant(&mut self) -> K {
        assert_eq!(
            self.rows, self.cols,
            "Error: Matrix must be square to calculate determinant!"
        );
        let mut new = self.clone();
        let mut det = K::one();
        // Kare matris olduğu için satır veya sütun farketmiyor
        for piv_col in 0..new.cols {
            if let Some(max_row) = ((piv_col)..new.rows)
                .max_by(|&a, &b| {
                    let val = |&r| new.row_col_val(r, piv_col).abs();
                    val(&a).partial_cmp(&val(&b)).unwrap()
                })
                .filter(|&f| new.row_col_val(f, piv_col).abs() > K::epsilon())
            {
                new.switch_rows(max_row, piv_col);
                if max_row != piv_col {
                    det = -det
                }
                det = det * new.row_col_val(piv_col, piv_col);
                for i in (piv_col + 1)..new.cols {
                    new.data[piv_col * new.cols + i] =
                        new.data[piv_col * new.cols + i] / new.data[piv_col * new.cols + piv_col];
                }
                new.data[piv_col * new.cols + piv_col] = K::one();
                for i in 0..new.rows {
                    if i != piv_col && new.row_col_val(i, piv_col).abs() > K::epsilon() {
                        for j in (piv_col + 1)..new.cols {
                            new.data[i * new.cols + j] = (-new.data[piv_col * new.cols + j])
                                .mul_add(
                                    new.data[i * new.cols + piv_col],
                                    new.data[i * new.cols + j],
                                );
                        }
                    }
                }
            } else {
                return K::zero();
            }
        }
        det
    }
}
