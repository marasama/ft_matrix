use crate::matrix::Matrix;
use crate::vector::Vector;
use num_traits::Float;

impl<K: Float> Matrix<K> {
    fn mul_vec(&mut self, vec: Vector<K>) -> Vector<K> {
        vec
    }
    fn mul_mat(&mut self, mat: Matrix<K>) -> Matrix<K> {
        self.clone()
    }
}
