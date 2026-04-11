#[allow(unused_imports)]
use super::*;
use crate::vector::Vector;

#[test]
fn new_vector_from_vec() {
    let vec_array = vec![1.2, 4.3, 4.2, 5.1, 3.2];
    let a = Vector::new(vec_array.clone());
    assert_eq!(a.data, vec_array, "Error: Vector::new()");
}

#[test]
fn new_vector_from_array() {
    let array: [f64; 5] = [1.2, 4.3, 4.2, 5.1, 3.2];
    let vec_array: Vec<f64> = Vec::from(array);
    let a = Vector::from(array);
    assert_eq!(a.data, vec_array, "Error: Vector::from()");
}

#[test]
fn size_of_vector() {
    let a = Vector::from([1.2, 3.2, 4.1, 4.2]);
    assert_eq!(a.size(), 4, "Error: Vector::size()");
}

#[test]
fn sub_vector() {
    let a: Vector<f64> = Vector::from([1.2, 0.2, 0.4, 6.]);
    let mut b: Vector<f64> = Vector::from([2.4, 0.1, 0.2, 3.]);
    b.sub(&a);
}
