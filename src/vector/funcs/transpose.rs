use std::usize;

use num_traits::Float;

use crate::matrix::Matrix;
use crate::vector::Vector;

impl<K: Float, const N: usize> Vector<K, N>
where
    [(); 1 * N]:,
{
    pub fn transpose(&self) -> Matrix<K, 1, N> {
        let mut data = [K::zero(); 1 * N];
        data.copy_from_slice(&self.data);
        Matrix { data }
    }
}
