use crate::matrix::Matrix;
use num_traits::Float;

impl<K: Float> Matrix<K> {
    pub fn determinant(&mut self) -> K {
        assert_eq!(self.rows, self.cols, "Error: Matrix must be square to calculate determinant!");
        let mut det = K::zero();

        for i in 0..self.rows {
            for j in 0..self.cols {

            }
        }
        det
    }
}
