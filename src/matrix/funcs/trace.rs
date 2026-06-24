use crate::matrix::Matrix;
use num_traits::Float;

impl<K: Float, const R: usize> Matrix<K, R, R>
where
    [(); R * R]:,
{
    /// Only square matrices have a trace!
    pub fn trace(&mut self) -> K {
        let mut trace_val = K::zero();
        for i in 0..R {
            trace_val = trace_val + self.data[i * R + i];
        }
        trace_val
    }
}
