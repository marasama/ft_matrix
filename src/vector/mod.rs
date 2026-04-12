use core::fmt;
use num_traits::Float;
use std::{
    any::type_name_of_val,
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Sub, SubAssign},
};

pub struct Vector<K: Float> {
    pub data: Vec<K>,
}

impl<K> fmt::Display for Vector<K>
where
    K: Display + Float,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let precision = f.precision().unwrap_or(2);
        // let max_len = format!("{:.prec$}", self.data.iter().max().unwrap_or(&K::zero()), prec = precision).len();
        let mut max_val: K = K::min_value();
        for i in &self.data {
            if *i > max_val {
                max_val = *i;
            }
        }
        let max_len = format!("{:.prec$}", max_val, prec = precision).len();
        for i in &self.data {
            writeln!(f, "[{:>pad$.prec$}]", i, pad = max_len, prec = precision)?;
        }
        Ok(())
    }
}

impl<K> fmt::Debug for Vector<K>
where
    K: Debug + Display + Float,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let precision = f.precision().unwrap_or(2);
        // let max_len = format!("{:.prec$}", self.data.iter().max().unwrap_or(&K::zero()), prec = precision).len();
        let _ = writeln!(f, "Type: {}", type_name_of_val(&self.data[0]));
        let mut max_val: K = K::min_value();
        for i in &self.data {
            if *i > max_val {
                max_val = *i;
            }
        }
        let max_len = format!("{:.prec$}", max_val, prec = precision).len();
        for i in &self.data {
            writeln!(f, "[{:>pad$.prec$}]", i, pad = max_len, prec = precision)?;
        }
        Ok(())
    }
}

impl<K> Vector<K>
where
    K: Add + AddAssign + Sub + SubAssign + Float,
{
    pub fn new(new_data: Vec<K>) -> Vector<K> {
        Vector {
            data: new_data.clone(),
        }
    }
    /// Returns the size of Vector
    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn sub(&mut self, other: &Vector<K>) {
        for i in 0..self.size() {
            self.data[i] -= other.data[i];
        }
    }

    pub fn add(&mut self, other: &Vector<K>) {
        for i in 0..self.size() {
            self.data[i] += other.data[i];
        }
    }

    pub fn scl(&mut self, other: K) {
        for i in 0..self.size() {
            self.data[i] = self.data[i] * other;
        }
    }
}

/// From Array
impl<K: Float, const N: usize> From<[K; N]> for Vector<K> {
    fn from(a: [K; N]) -> Vector<K> {
        Vector { data: Vec::from(a) }
    }
}
