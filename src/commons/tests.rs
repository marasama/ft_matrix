use crate::commons::lerp_mul_add::lerp;
use crate::matrix::Matrix;
use crate::vector::Vector;
#[test]
fn test_lerp() {
    let a = lerp(0., 1., 0.);
    assert_eq!(a, 0., "Error: Lerp error!");
    // 0.0
    let a = lerp(0., 1., 1.);
    assert_eq!(a, 1., "Error: Lerp error!");
    // 1.0
    let a = lerp(0., 1., 0.5);
    assert_eq!(a, 0.5, "Error: Lerp error!");
    // 0.5
    let a = lerp(21., 42., 0.3);
    assert_eq!(a, 27.300001, "Error: Lerp error!");
    // 27.3
    let a = lerp(Vector::from([2., 1.]), Vector::from([4., 2.]), 0.3);
    assert_eq!(Vector::from([2.6, 1.3]), a, "Error: Lerp error!");
    // [2.6]
    // [1.3]
    let a = lerp(
        Matrix::from([[2., 1.], [3., 4.]]),
        Matrix::from([[20., 10.], [30., 40.]]),
        0.5,
    );
    assert_eq!(
        Matrix::from([[11., 5.5], [16.5, 22.]]),
        a,
        "Error: Lerp error!"
    );
    // [[11., 5.5]
    // [16.5, 22.]]
}
