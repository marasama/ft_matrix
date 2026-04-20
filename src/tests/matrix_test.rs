#[allow(unused_imports)]
use super::*;
use crate::matrix::Matrix;
use crate::vector::Vector;

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

#[test]
fn multiply_test1() {
    let mut u = Matrix::from([[1., 0.], [0., 1.]]);
    let v = Vector::from([4., 2.]);
    assert_eq!((Vector::from([4., 2.])), u.mul_vec(v));
    // [4.]
    // [2.]
    let mut u = Matrix::from([[2., 0.], [0., 2.]]);
    let v = Vector::from([4., 2.]);
    assert_eq!(Vector::from([8., 4.]), u.mul_vec(v));
    // [8.]
    // [4.]
    let mut u = Matrix::from([[2., -2.], [-2., 2.]]);
    let v = Vector::from([4., 2.]);
    assert_eq!(Vector::from([4., -4.]), u.mul_vec(v));
    // [4.]
    // [-4.]
}

#[test]
fn multiply_test2() {
    let mut u = Matrix::from([[1., 0.], [0., 1.]]);
    let v = Matrix::from([[1., 0.], [0., 1.]]);
    println!("{}", u.mul_mat(v));
    // [1., 0.]
    // [0., 1.]
    let mut u = Matrix::from([[1., 0.], [0., 1.]]);
    let v = Matrix::from([[2., 1.], [4., 2.]]);
    println!("{}", u.mul_mat(v));
    // [2., 1.]
    // [4., 2.]
    let mut u = Matrix::from([[3., -5.], [6., 8.]]);
    let v = Matrix::from([[2., 1.], [4., 2.]]);
    println!("{}", u.mul_mat(v));
    // [-14., -7.]
    // [44., 22.]
}

#[test]
fn trace_test() {
    let mut u = Matrix::from([[1., 0.], [0., 1.]]);
    assert_eq!(2.0, u.trace());
    // 2.0
    let mut u = Matrix::from([[2., -5., 0.], [4., 3., 7.], [-2., 3., 4.]]);
    assert_eq!(9.0, u.trace());
    // 9.0
    let mut u = Matrix::from([[-2., -8., 4.], [1., -23., 4.], [0., 6., 4.]]);
    assert_eq!(-21.0, u.trace());
    // -21.0
}

#[test]
fn transpose_test() {
    let mut u = Matrix::from([[1., 2., 3.], [4., 5., 6.]]);
    println!("{}", &u);
    assert_eq!(Matrix::from([[1., 4.], [2., 5.], [3., 6.]]), u.transpose());
    // [1., 4.]
    // [2., 5.]
    // [3., 6.]
}

#[test]
fn row_echelon_test() {
    let mut u = Matrix::from([[1., 3., 1., 9.], [1., 1., -1., 1.], [3., 11., 5., 35.]]);

    println!("{:?}", u.row_echelon());
    // [1., 0.,-2.,-3.]
    // [0., 1., 1., 4.]
    // [0., 0., 0., 0.]

    let mut u = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
    let result_mat = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
    println!("{}", u.row_echelon());
    assert_eq!(result_mat, u.row_echelon());
    // [1.0, 0.0, 0.0]
    // [0.0, 1.0, 0.0]
    // [0.0, 0.0, 1.0]
    let mut u = Matrix::from([[1., 2.], [3., 4.]]);
    let result_mat = Matrix::from([[1., 0.], [0., 1.]]);
    println!("{}", u.row_echelon());
    assert_eq!(result_mat, u.row_echelon());
    // [1.0, 0.0]
    // [0.0, 1.0]
    let mut u = Matrix::from([[1., 2.], [2., 4.]]);
    let result_mat = Matrix::from([[1., 2.], [0., 0.]]);
    println!("{}", u.row_echelon());
    assert_eq!(result_mat, u.row_echelon());
    // [1.0, 2.0]
    // [0.0, 0.0]
    let mut u = Matrix::from([
        [8., 5., -2., 4., 28.],
        [4., 2.5, 20., 4., -4.],
        [8., 5., 1., 4., 17.],
    ]);
    let result_mat = Matrix::from([[1.0, 0.625, 0.0, 0.0, -12.1666667],
                                    [0.0, 0.0, 1.0, 0.0, -3.6666667],
                                    [0.0, 0.0, 0.0, 1.0, 29.5]]);
    println!("{}", u.row_echelon());
    assert_eq!(result_mat, u.row_echelon());
    // [1.0, 0.625, 0.0, 0.0, -12.1666667]
    // [0.0, 0.0, 1.0, 0.0, -3.6666667]
    // [0.0, 0.0, 0.0, 1.0, 29.5 ]
}

#[test]
fn determinant_test() {
    let mut u = Matrix::from([[1., -1.], [-1., 1.]]);
    println!("{}", u.determinant());
    // 0.0
    let mut u = Matrix::from([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]);
    println!("{}", u.determinant());
    assert_eq!(8.0, u.determinant());
    // 8.0
    let mut u = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]);
    println!("{}", u.determinant());
    assert_eq!(-174.00000000000003, u.determinant());
    // -174.0
    let mut u = Matrix::from([
        [8., 5., -2., 4.],
        [4., 2.5, 20., 4.],
        [8., 5., 1., 4.],
        [28., -4., 17., 1.],
    ]);
    println!("{}", u.determinant());
    assert_eq!(1032.0, u.determinant());
    // 1032
}

#[test]
fn rank_test() {
    let mut u = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
    println!("{}", u.rank());
    assert_eq!(3, u.rank());
    // 3
    let mut u = Matrix::from([[1., 2., 0., 0.], [2., 4., 0., 0.], [-1., 2., 1., 1.]]);
    println!("{}", u.rank());
    assert_eq!(2, u.rank());
    // 2
    let mut u = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.], [21., 18., 7.]]);
    println!("{}", u.rank());
    assert_eq!(3, u.rank());
    // 3
}

#[test]
fn inverse_test() {
    let mut u = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
    println!("{}", u.inverse());
    assert_eq!(Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]), u.inverse());
    // [1.0, 0.0, 0.0]
    // [0.0, 1.0, 0.0]
    // [0.0, 0.0, 1.0]
    let mut u = Matrix::from([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]);
    println!("{}", u.inverse());
    assert_eq!(Matrix::from([[0.5, 0., 0.], [0., 0.5, 0.], [0., 0., 0.5]]), u.inverse());
    // [0.5, 0.0, 0.0]
    // [0.0, 0.5, 0.0]
    // [0.0, 0.0, 0.5]
    let mut u = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]);
    let result_mat = Matrix::from([[0.649425287356, 0.097701149425, -0.655172413793],
                                        [-0.781609195402, -0.126436781609, 0.965517241379],
                                        [0.143678160920, 0.074712643678, -0.206896551724]]);
    println!("{:.12?}", u.inverse());
    println!("{:.12?}", result_mat);
    assert_eq!(result_mat, u.inverse());
    // [0.649425287, 0.097701149, -0.655172414]
    // [-0.781609195, -0.126436782, 0.965517241]
    // [0.143678161, 0.074712644, -0.206896552]
}
