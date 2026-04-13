#[allow(unused_imports)]
use super::*;
use crate::matrix::Matrix;

#[test]
fn new_matrix_from_vec() {
    let vec_array = vec![1.2, 4.3, 4.2, 2.4, 8.6, 8.4, 3.6, 12.9, 12.6];
    let a = Matrix::new(vec_array.clone(), 3, 3);
    let array: [[f64; 3]; 3] = [[1.2, 4.3, 4.2], [2.4, 8.6, 8.4], [3.6, 12.9, 12.6]];
    let b = Matrix::from(array);
    assert_eq!(a, b);
}

#[test]
fn new_matrix_from_array() {
    let vec_array = vec![1.2, 4.3, 4.2, 2.4, 8.6, 8.4, 3.6, 12.9, 12.6];
    let a = Matrix::new(vec_array.clone(), 3, 3);
    let array: [[f64; 3]; 3] = [[1.2, 4.3, 4.2], [2.4, 8.6, 8.4], [3.6, 12.9, 12.6]];
    let b = Matrix::from(array);
    assert_eq!(a, b);
}

#[test]
fn size_of_matrix() {
    let a = Matrix::from([[1.2, 4.3, 4.2], [2.4, 8.6, 8.4], [3.6, 12.9, 12.6]]);
    a.print_size();
    assert_eq!(a.size(), (3, 3), "Error: Vector::size()");
}

#[test]
fn is_equal() {
    let a = Matrix::from([[1.2, 4.3, 4.2], [2.4, 8.6, 8.4], [3.6, 12.9, 12.6]]);
    let b = Matrix::from([[1.3, 4.3, 4.2], [2.4, 8.6, 8.4], [3.6, 12.9, 12.6]]);
    assert!(a != b);
}

#[test]
fn matrix_add() {
    let mut a = Matrix::from([[1.2, 2.4, 4.8], [2.4, 4.8, 9.6], [4.8, 9.6, 19.2]]);
    let b = Matrix::from([[1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0]]);
    let res = Matrix::from([[2.2, 3.4, 5.8], [3.4, 5.8, 10.6], [5.8, 10.6, 20.2]]);
    a.add(&b);
    assert_eq!(a, res);
}

#[test]
fn matrix_sub() {
    let mut a: Matrix<f32> = Matrix::from([[1.0, 2.0, 4.0], [2.0, 4.0, 8.0], [4.0, 8.0, 16.0]]);
    let b: Matrix<f32> = Matrix::from([[1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0]]);
    let res: Matrix<f32> = Matrix::from([[0., 1., 3.], [1., 3., 7.], [3., 7., 15.]]);
    a.sub(&b);
    assert_eq!(a, res);
}

#[test]
fn matrix_scl() {
    let mut a = Matrix::from([[1.2, 2.4, 4.8], [2.4, 4.8, 9.6], [4.8, 9.6, 19.2]]);
    let b = 0.5;
    let res = Matrix::from([[0.6, 1.2, 2.4], [1.2, 2.4, 4.8], [2.4, 4.8, 9.6]]);
    a.scl(b);
    assert_eq!(a, res);
}
