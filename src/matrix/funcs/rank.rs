use crate::{matrix::Matrix, MATRIX_EPS};
use num_traits::Float;

impl<K: Float, const R: usize, const C: usize> Matrix<K, R, C>
where
    [(); R * C]:,
{
    pub fn gaussian_elimination(&self) -> Matrix<K, R, C> {
        let mut new = self.clone();
        let mut piv_col = 0;
        let mut piv_row = 0;
        while piv_row < R && piv_col < C {
            if let Some(max_row) = (piv_row..R)
                .max_by(|&a, &b| {
                    let val = |&r: &usize| new.data[r * C + piv_col].abs();
                    val(&a).partial_cmp(&val(&b)).unwrap()
                })
                .filter(|&f| new.data[f * C + piv_col].abs() > K::epsilon())
            {
                new.switch_rows(max_row, piv_row);
                let pivot_val = new.data[piv_row * C + piv_col];
                for i in 0..C {
                    new.data[piv_row * C + i] = new.data[piv_row * C + i] / pivot_val;
                }

                for i in 0..R {
                    if i == piv_row {
                        continue;
                    }

                    let factor = new.data[i * C + piv_col];
                    for j in 0..C {
                        let subtrahend = new.data[piv_row * C + j] * factor;
                        new.data[i * C + j] = new.data[i * C + j] - subtrahend;
                    }
                }
                piv_row += 1;
            }
            piv_col += 1;
        }
        new
    }
    pub fn rank(&self) -> usize {
        let reduced = self.gaussian_elimination();
        reduced
            .data
            .chunks(C)
            .filter(|row| row.iter().any(|&x| x.abs() > K::from(MATRIX_EPS).unwrap()))
            .count()
    }
}
