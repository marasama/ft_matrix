use core::fmt;
use num_traits::Float;
use std::ops::{AddAssign, SubAssign};

#[derive(Clone)]
pub struct Matrix<K: Float> {
    pub data: Vec<K>,
    pub rows: usize,
    pub cols: usize,
}

mod base;

mod printing;
