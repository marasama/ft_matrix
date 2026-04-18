use crate::matrix::Matrix;
use num_traits::Float;

/// Returns a square identity matrix at given size
pub fn identity_matrix<K: Float>(r_c: usize) -> Matrix<K> {
    let mut id_data = vec![K::zero(); r_c * r_c];
    for i in 0..r_c {
        id_data[i * r_c + i] = K::one();
    }
    Matrix {
        data: id_data,
        rows: r_c,
        cols: r_c,
    }
}

impl<K: Float> Matrix<K> {
    // Gauss elemesiyle aynı sadece uyguladığın
    // eleme işlemlerinin aynısını birim matrise de
    // uyguluyorsun
    pub fn inverse(&mut self) -> Matrix<K> {
        assert_eq!(
            self.rows, self.cols,
            "Error: Matrix must be NxN size to calculate its inverse!"
        );
        let mut new = self.clone();
        let mut piv_row = 0;
        let mut piv_col = 0;
        let mut idt_mat: Matrix<K> = identity_matrix(self.rows);
        while piv_row < new.rows && piv_col < new.cols {
            if let Some(max_row) = (piv_row..new.rows)
                .max_by(|&a, &b| {
                    let val = |&r: &usize| new.data[r * new.cols + piv_col].abs();
                    val(&a).partial_cmp(&val(&b)).unwrap()
                })
                .filter(|&f| new.data[f * new.cols + piv_col].abs() > K::epsilon())
            {
                // Orijinal Matris
                new.switch_rows(max_row, piv_row);
                // Birim Matris
                idt_mat.switch_rows(max_row, piv_row);

                let piv_val = new.row_col_val(piv_row, piv_col);
                for i in 0..new.cols {
                    // Orijinal Matris
                    new.data[piv_row * new.cols + i] = new.data[piv_row * new.cols + i] / piv_val;
                    // Birim Matris
                    idt_mat.data[piv_row * new.cols + i] =
                        idt_mat.data[piv_row * new.cols + i] / piv_val;
                }

                for i in 0..new.rows {
                    if i == piv_row {
                        continue;
                    }

                    // Oran her iki matris içinde sabit
                    let factor = new.data[i * new.cols + piv_col];

                    for j in 0..new.cols {
                        let ori_subtrahend = new.data[piv_row * new.cols + j] * factor;
                        let idt_subtrahend = idt_mat.data[piv_row * new.cols + j] * factor;
                        // Orijinal Matris
                        new.data[i * new.cols + j] = new.data[i * new.cols + j] - ori_subtrahend;
                        // Birim Matris
                        idt_mat.data[i * new.cols + j] =
                            idt_mat.data[i * new.cols + j] - idt_subtrahend;
                    }
                }
                piv_row += 1;
            }
            piv_col += 1;
        }
        idt_mat
    }
}
