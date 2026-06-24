use crate::vector::Vector;
use num_traits::Float;

pub fn linear_combination<K: Float, const N: usize, const M: usize>(
    u: &[Vector<K, N>; M],
    coefs: &[K; M],
) -> Vector<K, N> {
    let mut new_data = [K::zero(); N];
    for (vect, coefficent) in u.iter().zip(coefs.iter()) {
        for (total, elem) in new_data.iter_mut().zip(vect.data.iter()) {
            *total = elem.mul_add(*coefficent, *total);
        }
    }
    Vector { data: new_data }
}
