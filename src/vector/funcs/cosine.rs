use num_traits::Float;

use crate::{vector::Vector, VECTOR_EPS};

pub fn angle_cos<K: Float, const N: usize>(u: &Vector<K, N>, v: &Vector<K, N>) -> Option<K> {
    let u_norm = u.norm();
    let v_norm = v.norm();
    let denom = u_norm * v_norm;

    if denom < K::from(VECTOR_EPS).unwrap() {
        return None;
    }
    let cos = u.dot(v) / (u_norm * v_norm);
    Some(cos.max(-K::one()).min(K::one()))
}
