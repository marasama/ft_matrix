use std::ops::SubAssign;

use crate::matrix::Matrix;
use crate::vector::Vector;
use num_traits::Float;

impl<K: Float + SubAssign> Matrix<K> {
    pub fn sub_each_col(&self, v: &Vector<K>) -> Matrix<K> {
        assert_eq!(
            self.rows,
            v.size(),
            "Row sizes must be equal to substract the vector from the each column of the matrix!"
        );
        let mut new_data = self.data.clone();
        for r in 0..self.rows {
            let val = v.data[r];
            for c in 0..self.cols {
                new_data[r * self.cols + c] -= val;
            }
        }
        Matrix {
            data: new_data,
            rows: self.rows,
            cols: self.cols,
        }
    }
    pub fn sub_each_row(&self, v: &Vector<K>) -> Matrix<K> {
        assert_eq!(
            self.cols,
            v.size(),
            "Columns sizes must be equal to substract the vector from the each row of the matrix!"
        );
        let mut new_data = self.data.clone();
        for r in 0..self.rows {
            for c in 0..self.cols {
                new_data[r * self.cols + c] -= v.data[c];
            }
        }
        Matrix {
            data: new_data,
            rows: self.rows,
            cols: self.cols,
        }
    }
}
