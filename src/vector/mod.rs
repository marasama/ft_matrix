use core::fmt;
use num_traits::Float;
use std::{
    fmt::{Debug, Display},
    ops::{Add, Sub},
};

#[derive(Clone)]
pub struct Vector<K: Float, const N: usize> {
    pub data: [K; N],
}

mod printing;

mod base;

pub mod funcs;
