use crate::matrix::Matrix;
use num_traits::Float;

impl<K: Float> Matrix<K> {
    fn switch_rows(&mut self, src: usize, dest: usize) {
        assert!(src < self.rows);
        assert!(dest < self.rows);
        if src == dest {
            return;
        }
        for i in 0..self.cols {
            self.data.swap(src * self.cols + i, dest * self.cols + i);
        }
    }
    fn ch_row_col_val(&mut self, r: usize, c: usize, new_val: K) {
        assert!(self.rows > r);
        assert!(self.cols > c);
        self.data[self.cols * r + c] = new_val;
    }
    //    pub fn row_echelon(&mut self) -> Matrix<K> {
    //        let mut new_matrix = self.clone();
    //        let mut pivot_row: usize = 0;
    //        let mut pivot_col: usize = 0;
    //
    //        while pivot_row < new_matrix.rows && pivot_col < new_matrix.cols {
    //            let mut col_max = K::zero();
    //            let mut col_max_idx: usize = 0;
    //            // Aynı sütundaki en yüksek değeri pivot olarak belirliyoruz
    //            for i in pivot_row..new_matrix.rows {
    //                if new_matrix.row_col_val(i, pivot_col).abs() > col_max {
    //                    col_max = new_matrix.row_col_val(i, pivot_col).abs();
    //                    col_max_idx = i;
    //                }
    //            }
    //            // Sütundaki en yüksek değer 0 ise pivot yoktur, sonraki sütuna bakıyoruz
    //            if col_max < K::epsilon() {
    //                pivot_col += 1;
    //            } else {
    //                new_matrix.switch_rows(pivot_row, col_max_idx);
    //                // Altta kalan her satır için sıfırlama işlemi yapılıyor
    //
    //                for i in pivot_row + 1..new_matrix.rows {
    //                    let row_ratio = new_matrix.row_col_val(i, pivot_col)
    //                        / new_matrix.row_col_val(pivot_row, pivot_col);
    //                    //new_matrix.ch_row_col_val(i, pivot_col, K::zero());
    //                    for k in pivot_col + 1..new_matrix.cols {
    //                        new_matrix.data[i * new_matrix.cols + k] = new_matrix.data
    //                            [i * new_matrix.cols + k]
    //                            - new_matrix.data[pivot_row * new_matrix.cols + k] * row_ratio;
    //                    }
    //                }
    //                pivot_row += 1;
    //                pivot_col += 1;
    //            }
    //        }
    //        new_matrix
    //    }
    //}

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
