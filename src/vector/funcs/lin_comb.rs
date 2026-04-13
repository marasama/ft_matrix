use crate::vector::Vector;
use num_traits::Float;

pub fn linear_combination<K: Float>(u: &[Vector<K>], coefs: &[K]) -> Vector<K> {
    assert_eq!(
        u.len(),
        coefs.len(),
        "Error: Vector and coefficent size mismatch!"
    );
    let len_must = u[0].size();
    assert!(len_must > 0, "Error: Empty Vector in array!");
    for i in u.iter() {
        assert_eq!(len_must, i.size(), "Error: Size mismatch at Vector array!");
    }

    let mut new_data = vec![K::zero(); len_must];
    for (vect, coefficent) in u.iter().zip(coefs.iter()) {
        for (total, elem) in new_data.iter_mut().zip(vect.data.iter()) {
            *total = elem.mul_add(*coefficent, *total);
        }
    }
    Vector { data: new_data }
}
