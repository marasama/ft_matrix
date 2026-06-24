use super::*;
use std::any::type_name_of_val;
use std::fmt::{Debug, Display};

impl<K, const R: usize, const C: usize> fmt::Display for Matrix<K, R, C>
where
    K: Display + Float + AddAssign + SubAssign,
    [(); R * C]:,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let precision = f.precision().unwrap_or(2);
        let max_len = self
            .data
            .iter()
            .map(|a| format!("{:.1$}", a, precision).len())
            .max()
            .unwrap_or(0);
        for r in 0..R {
            let _ = write!(f, "[ ");
            for c in 0..C - 1 {
                write!(
                    f,
                    "{:>pad$.prec$}, ",
                    self.row_col_val(r, c),
                    pad = max_len,
                    prec = precision
                )?;
            }
            write!(
                f,
                "{:>pad$.prec$}",
                self.row_col_val(r, C - 1),
                pad = max_len,
                prec = precision
            )?;
            let _ = writeln!(f, " ]");
        }
        Ok(())
    }
}

impl<K, const R: usize, const C: usize> fmt::Debug for Matrix<K, R, C>
where
    K: Display + Debug + Float + AddAssign + SubAssign,
    [(); R * C]:,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let precision = f.precision().unwrap_or(2);
        let _ = writeln!(f, "Type: {}", type_name_of_val(&self.data[0]));
        let max_len = self
            .data
            .iter()
            .map(|a| format!("{:.1$}", a, precision).len())
            .max()
            .unwrap_or(0);
        for r in 0..R {
            let _ = write!(f, "[ ");
            for c in 0..C - 1 {
                write!(
                    f,
                    "{:>pad$.prec$}, ",
                    self.row_col_val(r, c),
                    pad = max_len,
                    prec = precision
                )?;
            }
            write!(
                f,
                "{:>pad$.prec$}",
                self.row_col_val(r, C - 1),
                pad = max_len,
                prec = precision
            )?;
            let _ = writeln!(f, " ]");
        }
        Ok(())
    }
}
