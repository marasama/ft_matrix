use num_traits::Float;

use crate::vector::Vector;

impl<K: Float> Vector<K> {
    pub fn norm_1(&mut self) -> f32 {
        self.data
            .iter()
            .map(|a| a.abs())
            .fold(K::zero(), |acc, x| acc + x)
            .to_f32()
            .unwrap()
    }
    pub fn norm(&mut self) -> f32 {
        self.data
            .iter()
            .map(|a| *a * *a)
            .fold(K::zero(), |acc, x| x + acc)
            .powf(K::from(0.5).unwrap())
            .to_f32()
            .unwrap()
    }
    pub fn norm_ref(&self) -> f32 {
        self.data
            .iter()
            .map(|a| *a * *a)
            .fold(K::zero(), |acc, x| x + acc)
            .powf(K::from(0.5).unwrap())
            .to_f32()
            .unwrap()
    }
    pub fn norm_inf(&mut self) -> f32 {
        self.data
            .iter()
            .map(|x| x.abs())
            .fold(K::zero(), |acc, x| if x > acc { x } else { acc })
            .to_f32()
            .unwrap()
    }
}
