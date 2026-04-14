use num_traits::{Float, NumCast};

use crate::vector::Vector;

pub fn angle_cos<K: Float>(u: &Vector<K>, v: &Vector<K>) -> f32 {
    assert_eq!(u.size(), v.size(), "Error: Size mismatch!");
    let u_norm: f32 = u.norm_ref();
    let v_norm: f32 = v.norm_ref();
    let dot_uv: f32 = NumCast::from(u.dot_ref(v)).unwrap();
    assert_ne!(u_norm, 0.0f32, "Error: Zero vector input!");
    assert_ne!(v_norm, 0.0f32, "Error: Zero vector input!");
    dot_uv / (u_norm * v_norm)
}
