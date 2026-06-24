use crate::vector::Vector;
use num_traits::Float;

pub fn cross_product<K: Float>(u: &Vector<K, 3>, v: &Vector<K, 3>) -> Vector<K, 3> {
    Vector {
        data: [
            u.data[1].mul_add(v.data[2], -(u.data[2] * v.data[1])),
            u.data[2].mul_add(v.data[0], -(u.data[0] * v.data[2])),
            u.data[0].mul_add(v.data[1], -(u.data[1] * v.data[0])),
        ],
    }
}
