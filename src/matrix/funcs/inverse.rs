use crate::matrix::Matrix;
use num_traits::Float;

/// Returns a square identity matrix at given size
pub fn identity_matrix<K: Float, const R: usize>() -> Matrix<K, R, R>
where
    [(); R * R]:,
{
    let mut id_data = [K::zero(); R * R];
    for i in 0..R {
        id_data[i * R + i] = K::one();
    }
    Matrix { data: id_data }
}

impl<K: Float, const R: usize> Matrix<K, R, R>
where
    [(); R * R]:,
{
    // Gauss elemesiyle aynı sadece uyguladığın
    // eleme işlemlerinin aynısını birim matrise de
    // uyguluyorsun
    pub fn inverse(&self) -> Option<Matrix<K, R, R>> {
        let mut new = self.clone();
        let mut idt_mat: Matrix<K, R, R> = identity_matrix::<K, R>();
        for piv in 0..R {
            let max_row = (piv..R)
                .max_by(|&a, &b| {
                    let val = |&r: &usize| new.data[r * R + piv].abs();
                    val(&a).partial_cmp(&val(&b)).unwrap()
                })
                .filter(|&f| new.data[f * R + piv].abs() > K::epsilon())?;
            // Orijinal Matris
            new.switch_rows(max_row, piv);
            // Birim Matris
            idt_mat.switch_rows(max_row, piv);

            let piv_val = new.row_col_val(piv, piv);
            for j in 0..R {
                // Orijinal Matris
                new.data[piv * R + j] = new.data[piv * R + j] / piv_val;
                // Birim Matris
                idt_mat.data[piv * R + j] = idt_mat.data[piv * R + j] / piv_val;
            }

            for i in 0..R {
                if i == piv {
                    continue;
                }
                // Oran her iki matris içinde sabit
                let factor = new.data[i * R + piv];

                for j in 0..R {
                    // Orijinal Matris
                    new.data[i * R + j] = new.data[i * R + j] - new.data[piv * R + j] * factor;
                    // Birim Matris
                    idt_mat.data[i * R + j] =
                        idt_mat.data[i * R + j] - idt_mat.data[piv * R + j] * factor;
                }
            }
        }
        Some(idt_mat)
    }
}
