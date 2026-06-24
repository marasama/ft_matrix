use core::fmt;
use num_traits::Float;
use std::ops::{AddAssign, SubAssign};

#[derive(Clone)]
pub struct Matrix<K: Float, const R: usize, const C: usize>
where
    [(); R * C]:,
{
    pub data: [K; R * C],
}

mod base;

mod printing;

pub mod funcs;
