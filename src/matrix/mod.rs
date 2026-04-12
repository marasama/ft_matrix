use std::ops::{Add, AddAssign, Sub, SubAssign};

use num_traits::Float;

#[derive(Clone)]
pub struct Matrix<K: Float> {
    data: Vec<K>,
    pub rows: usize,
    pub cols: usize,
}

impl<K> Matrix<K>
where
    K: Float + Add + Sub + AddAssign + SubAssign,
{
    /// Size value function
    /// Returns (rows, cols) in usize
    pub fn size(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn print_size(&self) {
        println!("Rows: {} X Cols: {}", self.rows, self.cols);
    }

    pub fn size_matcher(&self, other: &Matrix<K>) {
        assert_eq!(self.data.len(), other.data.len(), "Error: Size mismatch");
        assert_eq!(self.rows, other.rows, "Error: Row size mismatch");
        assert_eq!(self.cols, other.cols, "Error: Columns size mismatch");
    }
}
