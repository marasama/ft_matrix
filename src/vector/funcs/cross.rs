use crate::vector::Vector;
use num_traits::Float;

pub fn cross_product<K: Float>(u: &Vector<K>, v: &Vector<K>) -> Vector<K> {
    assert_eq!(
        u.size(),
        3,
        "Error: Vector size must be 3 for cross-product!"
    );
    assert_eq!(
        v.size(),
        3,
        "Error: Vector size must be 3 for cross-product!"
    );
    let new_data: Vec<K> = vec![
        ((u.data[1] * v.data[2]) - (u.data[2] * v.data[1])),
        ((u.data[2] * v.data[0]) - (u.data[0] * v.data[2])),
        ((u.data[0] * v.data[1]) - (u.data[1] * v.data[0])),
    ];
    Vector { data: new_data }
}
