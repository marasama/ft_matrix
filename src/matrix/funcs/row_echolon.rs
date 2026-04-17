use crate::matrix::Matrix;
use num_traits::Float;

impl<K: Float> Matrix<K> {
    pub fn switch_rows(&mut self, src: usize, dest: usize) {
        assert!(src < self.rows);
        assert!(dest < self.rows);
        if src == dest {
            return;
        }
        for i in 0..self.cols {
            self.data.swap(src * self.cols + i, dest * self.cols + i);
        }
    }
    pub fn row_echelon(&mut self) -> Matrix<K> {
        let mut new_matrix = self.clone();
        let mut pivot_row = 0;
        let mut pivot_col = 0;

        while pivot_row < new_matrix.rows && pivot_col < new_matrix.cols {
            // Pivot'un altındaki en büyük 0'dan farklı değeri alır
            if let Some(i) = (pivot_row..new_matrix.rows).max_by(|&a, &b| {
                let val = |r| new_matrix.row_col_val(r, pivot_col).abs();
                val(a).partial_cmp(&val(b)).unwrap()
            }).filter(|&r| new_matrix.row_col_val(r, pivot_col).abs() > K::epsilon()) {

                new_matrix.switch_rows(pivot_row, i);

                let pivot_val = new_matrix.data[pivot_row * new_matrix.cols + pivot_col];
                // Pivotu 1 yapmak için bütün satırı pivota bölüyoruz
                for j in 0..new_matrix.cols {
                    new_matrix.data[pivot_row * new_matrix.cols + j] =
                        new_matrix.data[pivot_row * new_matrix.cols + j] / pivot_val;
                }

                // Pivot satırı dışındaki bütün satırları 0'a eşitlemek için 
                // pivot satırı ile orantılayıp çıkartıyoruz
                for l in 0..new_matrix.rows {
                    if l == pivot_row {
                        continue;
                    }
                    let factor = new_matrix.data[l * new_matrix.cols + pivot_col];
                    for m in 0..new_matrix.cols {
                        let subtrahend = factor * new_matrix.data[pivot_row * new_matrix.cols + m];
                        new_matrix.data[l * new_matrix.cols + m] =
                            new_matrix.data[l * new_matrix.cols + m] - subtrahend;
                    }
                }
                pivot_row += 1;
            }
            pivot_col += 1;
        }
        new_matrix
    }
}
