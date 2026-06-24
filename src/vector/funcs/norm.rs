use num_traits::Float;

use crate::vector::Vector;

impl<K: Float, const N: usize> Vector<K, N> {
    pub fn norm_1(&self) -> K {
        self.data
            .iter()
            .map(|a| a.abs())
            .fold(K::zero(), |acc, x| acc + x)
    }
    pub fn norm(&self) -> K {
        self.data
            .iter()
            .map(|a| *a * *a)
            .fold(K::zero(), |acc, x| x + acc)
            .powf(K::from(0.5).unwrap())
    }
    pub fn norm_inf(&mut self) -> K {
        self.data
            .iter()
            .map(|x| x.abs())
            .fold(K::zero(), |acc, x| if x > acc { x } else { acc })
    }
}
