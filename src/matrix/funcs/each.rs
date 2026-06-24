use std::ops::{AddAssign, SubAssign};

use crate::matrix::Matrix;
use crate::vector::Vector;
use num_traits::Float;

impl<K: Float + SubAssign + AddAssign, const R: usize, const C: usize> Matrix<K, R, C>
where
    [(); R * C]:,
{
    pub fn sub_each_col(&self, v: &Vector<K, R>) -> Matrix<K, R, C> {
        let mut new_data = self.data.clone();
        for r in 0..R {
            let val = v.data[r];
            for c in 0..C {
                new_data[r * C + c] -= val;
            }
        }
        Matrix { data: new_data }
    }
    pub fn sub_each_row(&self, v: &Vector<K, C>) -> Matrix<K, R, C> {
        let mut new_data = self.data.clone();
        for r in 0..R {
            for c in 0..C {
                new_data[r * C + c] -= v.data[c];
            }
        }
        Matrix { data: new_data }
    }
    pub fn add_each_col(&self, v: &Vector<K, R>) -> Matrix<K, R, C> {
        let mut new_data = self.data.clone();
        for r in 0..R {
            let val = v.data[r];
            for c in 0..C {
                new_data[r * C + c] += val;
            }
        }
        Matrix { data: new_data }
    }
    pub fn add_each_row(&self, v: &Vector<K, C>) -> Matrix<K, R, C> {
        let mut new_data = self.data.clone();
        for r in 0..R {
            for c in 0..C {
                new_data[r * C + c] += v.data[c];
            }
        }
        Matrix { data: new_data }
    }
}
