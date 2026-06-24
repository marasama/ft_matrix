use crate::vector::Vector;
use num_traits::Float;

impl<K: Float, const N: usize> Vector<K, N> {
    pub fn dot(&self, v: &Vector<K, N>) -> K {
        self.data
            .iter()
            .zip(v.data.iter())
            .fold(K::zero(), |acc, (a, b)| a.mul_add(*b, acc))
    }
}
