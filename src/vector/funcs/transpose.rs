use num_traits::Float;

use crate::matrix::Matrix;
use crate::vector::Vector;

impl<K: Float> Vector<K> {
    pub fn transpose(&self) -> Matrix<K> {
        let new_data: Vec<K> = self.data.clone();
        Matrix {
            data: new_data,
            rows: 1,
            cols: self.data.len(),
        }
    }
}
