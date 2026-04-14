use crate::vector::Vector;
use num_traits::Float;

impl<K: Float> Vector<K> {
    pub fn dot(&self, v: Vector<K>) -> K {
        assert_eq!(self.size(), v.size(), "error: size mismatch!");
        let mut sum = K::zero();
        for (a, b) in self.data.iter().zip(v.data.iter()) {
            sum = a.mul_add(*b, sum);
        }
        sum
    }
    pub fn dot_ref(&self, v: &Vector<K>) -> K {
        assert_eq!(self.size(), v.size(), "error: size mismatch!");
        let mut sum = K::zero();
        for (a, b) in self.data.iter().zip(v.data.iter()) {
            sum = a.mul_add(*b, sum);
        }
        sum
    }
}
