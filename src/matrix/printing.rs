use super::*;
use std::any::type_name_of_val;
use std::fmt::{Debug, Display};

impl<K> fmt::Display for Matrix<K>
where
    K: Display + Float + AddAssign + SubAssign,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let precision = f.precision().unwrap_or(2);
        let max_len = self
            .data
            .iter()
            .map(|a| format!("{:.1$}", a, precision).len())
            .max()
            .unwrap_or(0);
        for r in 0..self.rows {
            let _ = write!(f, "[ ");
            for c in 0..self.cols - 1 {
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
                self.row_col_val(r, self.cols - 1),
                pad = max_len,
                prec = precision
            )?;
            let _ = writeln!(f, " ]");
        }
        Ok(())
    }
}

impl<K> fmt::Debug for Matrix<K>
where
    K: Display + Debug + Float + AddAssign + SubAssign,
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
        for r in 0..self.rows {
            let _ = write!(f, "[ ");
            for c in 0..self.cols - 1 {
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
                self.row_col_val(r, self.cols - 1),
                pad = max_len,
                prec = precision
            )?;
            let _ = writeln!(f, " ]");
        }
        Ok(())
    }
}
