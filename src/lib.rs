#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
pub mod commons;
pub mod matrix;
mod tests;
pub mod vector;

const VECTOR_EPS: f64 = 1e-3;
const MATRIX_EPS: f64 = 1e-3;
