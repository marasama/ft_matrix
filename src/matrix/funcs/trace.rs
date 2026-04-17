use crate::matrix::Matrix;
use num_traits::Float;

impl<K: Float> Matrix<K> {
    pub fn trace(&mut self) -> K {
        assert_eq!(
            self.rows, self.cols,
            "Error: Only square matrices have a trace!"
        );
        let mut trace_val = K::zero();
        for i in 0..self.rows {
            trace_val = trace_val + self.row_col_val(i, i);
        }
        trace_val
    }
}
