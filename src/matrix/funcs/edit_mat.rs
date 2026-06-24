use crate::matrix::Matrix;
use num_traits::Float;

impl<K: Float, const R: usize, const C: usize> Matrix<K, R, C>
where
    [(); R * C]:,
{
    // Creates a new matrix from the selected rows
    pub fn get_rows<const M: usize>(&self, rows: &[usize; M]) -> Matrix<K, M, C>
    where
        [(); M * C]:,
    {
        let mut new_data = [K::zero(); M * C];

        for (i, &r) in rows.iter().enumerate() {
            assert!(
                r < R,
                "Selected rows must be smaller than the row size of the main matrix!"
            );
            let src_start = r * C;
            let src_end = src_start + C;
            let src_slice = &self.data[src_start..src_end];

            let dest_start = i * C;
            let dest_end = dest_start + C;
            let dest_slice = &mut new_data[dest_start..dest_end];

            dest_slice.copy_from_slice(src_slice);
        }

        Matrix { data: new_data }
    }
    // Creates a new matrix from the selected rows
    pub fn get_cols<const M: usize>(&self, cols: &[usize; M]) -> Matrix<K, R, M>
    where
        [(); R * M]:,
    {
        let mut new_data = [K::zero(); R * M];

        for (i, &c) in cols.iter().enumerate() {
            assert!(
                c < C,
                "Selected columns must be smaller than the column size of the main matrix!"
            );
            for r in 0..R {
                new_data[r * M + i] = self.data[r * C + c];
            }
        }

        Matrix { data: new_data }
    }
}
