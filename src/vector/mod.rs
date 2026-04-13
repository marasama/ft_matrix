use core::fmt;
use num_traits::Float;
use std::{
    fmt::{Debug, Display},
    ops::{Add, Sub},
};

#[derive(Clone)]
pub struct Vector<K: Float> {
    pub data: Vec<K>,
}

mod printing;

mod base;

pub mod funcs;
