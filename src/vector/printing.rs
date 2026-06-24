use super::*;
use std::any::type_name_of_val;

impl<K, const N: usize> fmt::Display for Vector<K, N>
where
    K: Display + Float,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let precision = f.precision().unwrap_or(20);
        let max_len = self
            .data
            .iter()
            .map(|a| format!("{:.1$}", a, precision).len())
            .max()
            .unwrap_or(0);
        for i in &self.data {
            writeln!(f, "[{:>pad$.prec$}]", i, pad = max_len, prec = precision)?;
        }
        Ok(())
    }
}

impl<K, const N: usize> fmt::Debug for Vector<K, N>
where
    K: Debug + Display + Float,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let precision = f.precision().unwrap_or(20);
        let _ = writeln!(f, "Type: {}", type_name_of_val(&self.data[0]));
        let max_len = self
            .data
            .iter()
            .map(|a| format!("{:.1$}", a, precision).len())
            .max()
            .unwrap_or(0);
        for i in &self.data {
            writeln!(f, "[{:>pad$.prec$}]", i, pad = max_len, prec = precision)?;
        }
        Ok(())
    }
}
