use crate::matrix::Matrix;
use num_traits::Float;

impl<K: Float, const R: usize, const C: usize> Matrix<K, R, C>
where
    [(); R * C]:,
{
    pub fn switch_rows(&mut self, src: usize, dest: usize) {
        assert!(src < R);
        assert!(dest < R);
        if src == dest {
            return;
        }
        for i in 0..C {
            self.data.swap(src * C + i, dest * C + i);
        }
    }
    pub fn row_echelon(&mut self) -> Matrix<K, R, C> {
        let mut new_matrix = self.clone();
        let mut pivot_row = 0;
        let mut pivot_col = 0;

        while pivot_row < R && pivot_col < C {
            // Pivot'un altındaki en büyük 0'dan farklı değeri alır
            if let Some(i) = (pivot_row..R)
                .max_by(|&a, &b| {
                    let val = |r| new_matrix.row_col_val(r, pivot_col).abs();
                    val(a).partial_cmp(&val(b)).unwrap()
                })
                .filter(|&r| new_matrix.data[r * C + pivot_col].abs() > K::epsilon())
            {
                new_matrix.switch_rows(pivot_row, i);

                let pivot_val = new_matrix.data[pivot_row * C + pivot_col];
                // Pivotu 1 yapmak için bütün satırı pivota bölüyoruz
                for j in 0..C {
                    new_matrix.data[pivot_row * C + j] =
                        new_matrix.data[pivot_row * C + j] / pivot_val;
                }

                // Pivot satırı dışındaki bütün satırları 0'a eşitlemek için
                // pivot satırı ile orantılayıp çıkartıyoruz
                for l in 0..R {
                    if l == pivot_row {
                        continue;
                    }
                    let factor = new_matrix.data[l * C + pivot_col];
                    for m in 0..C {
                        let subtrahend = factor * new_matrix.data[pivot_row * C + m];
                        new_matrix.data[l * C + m] = new_matrix.data[l * C + m] - subtrahend;
                    }
                }
                pivot_row += 1;
            }
            pivot_col += 1;
        }
        new_matrix
    }
}
