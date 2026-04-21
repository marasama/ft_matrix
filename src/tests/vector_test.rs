use crate::vector::funcs::cosine::angle_cos;
use crate::vector::funcs::cross::cross_product;
use crate::vector::funcs::lin_comb::linear_combination;
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
fn is_equal() {
    let a: Vector<f64> = Vector::from([12., 32., 4.2, 412.2, 241.41]);
    let b: Vector<f64> = Vector::from([11., 32., 4.2, 412.2, 241.41]);
    assert!(a != b);
}

#[test]
fn vector_add() {
    let mut a = Vector::from([12., 24., 36.]);
    let b = Vector::from([12., 24., 36.]);
    let res = Vector::from([24., 48., 72.]);
    a.add(&b);
    assert_eq!(a, res);
}

#[test]
fn vector_sub() {
    let mut a = Vector::from([24., 48., 72.]);
    let b = Vector::from([12., 24., 36.]);
    let res = Vector::from([12., 24., 36.]);
    a.sub(&b);
    assert_eq!(a, res);
}

#[test]
fn vector_scl() {
    let mut a = Vector::from([24., 48., 72.]);
    let b = 0.5;
    let res = Vector::from([12., 24., 36.]);
    a.scl(b);
    assert_eq!(a, res);
}

#[test]
fn lin_comb() {
    let e1 = Vector::from([1., 0., 0.]);
    let e2 = Vector::from([0., 1., 0.]);
    let e3 = Vector::from([0., 0., 1.]);

    let v1 = Vector::from([1., 2., 3.]);
    let v2 = Vector::from([0., 10., -100.]);

    let res_e = linear_combination(&[e1, e2, e3], &[10., -2., 0.5]);
    let res_v = linear_combination(&[v1, v2], &[10., -2.]);

    assert_eq!(res_e.data, vec![10., -2., 0.5]);
    assert_eq!(res_v.data, vec![10., 0., 230.]);
}

#[test]
fn dot_test() {
    let u = Vector::from([0., 0.]);
    let v = Vector::from([1., 1.]);
    assert_eq!(0.0, u.dot(v));
    // 0.0
    let u = Vector::from([1., 1.]);
    let v = Vector::from([1., 1.]);
    assert_eq!(2.0, u.dot(v));
    // 2.0
    let u = Vector::from([-1., 6.]);
    let v = Vector::from([3., 2.]);
    assert_eq!(9.0, u.dot(v));
    // 9.0
}

#[test]
fn norm_test() {
    let mut u = Vector::from([0., 0., 0.]);
    assert_eq!(u.norm_1(), 0.0);
    assert_eq!(u.norm(), 0.0);
    assert_eq!(u.norm_inf(), 0.0);
    // 0.0, 0.0, 0.0
    let mut u = Vector::from([1., 2., 3.]);
    assert_eq!(u.norm_1(), 6.0);
    assert_eq!(u.norm(), 3.74165738);
    assert_eq!(u.norm_inf(), 3.0);
    // 6.0, 3.74165738, 3.0
    let mut u = Vector::from([-1., -2.]);
    assert_eq!(u.norm_1(), 3.0);
    assert_eq!(u.norm(), 2.236067977);
    assert_eq!(u.norm_inf(), 2.0);
    // 3.0, 2.236067977, 2.0
}

#[test]
fn cosine_test() {
    let u = Vector::from([1., 0.]);
    let v = Vector::from([1., 0.]);
    assert_eq!(1.0, angle_cos(&u, &v));
    // 1.0
    let u = Vector::from([1., 0.]);
    let v = Vector::from([0., 1.]);
    assert_eq!(0.0, angle_cos(&u, &v));
    // 0.0
    let u = Vector::from([-1., 1.]);
    let v = Vector::from([1., -1.]);
    assert_eq!(-1.0000001, angle_cos(&u, &v));
    // -1.0
    let u = Vector::from([2., 1.]);
    let v = Vector::from([4., 2.]);
    assert_eq!(1.0, angle_cos(&u, &v));
    // 1.0
    let u = Vector::from([1., 2., 3.]);
    let v = Vector::from([4., 5., 6.]);
    assert_eq!(0.974_631_8, angle_cos(&u, &v));
    // 0.974631846
}

#[test]
fn cross_product_test() {
    let u = Vector::from([0., 0., 1.]);
    let v = Vector::from([1., 0., 0.]);
    assert_eq!(Vector::from([0., 1., 0.]), cross_product(&u, &v));
    // [0.]
    // [1.]
    // [0.]
    let u = Vector::from([1., 2., 3.]);
    let v = Vector::from([4., 5., 6.]);
    assert_eq!(Vector::from([-3., 6., -3.]), cross_product(&u, &v));
    // [-3.]
    // [6.]
    // [-3.]
    let u = Vector::from([4., 2., -3.]);
    let v = Vector::from([-2., -5., 16.]);
    assert_eq!(Vector::from([17., -58., -16.]), cross_product(&u, &v));
    // [17.]
    // [-58.]
    // [-16.]
}
