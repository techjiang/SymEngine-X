use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f64>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize, data: &[i64]) -> Self {
        Self { rows, cols, data: data.iter().map(|x| *x as f64).collect() }
    }
    pub fn from_f64(rows: usize, cols: usize, data: Vec<f64>) -> Self { Self { rows, cols, data } }
    pub fn get(&self, r: usize, c: usize) -> f64 { self.data[r * self.cols + c] }
    pub fn transpose(&self) -> Self {
        let mut out = vec![0.0; self.rows * self.cols];
        for r in 0..self.rows { for c in 0..self.cols { out[c * self.rows + r] = self.get(r, c); } }
        Self::from_f64(self.cols, self.rows, out)
    }
    pub fn add(&self, other: &Self) -> Result<Self, String> {
        self.check_shape(other)?;
        Ok(Self::from_f64(self.rows, self.cols, self.data.iter().zip(&other.data).map(|(a, b)| a + b).collect()))
    }
    pub fn sub(&self, other: &Self) -> Result<Self, String> {
        self.check_shape(other)?;
        Ok(Self::from_f64(self.rows, self.cols, self.data.iter().zip(&other.data).map(|(a, b)| a - b).collect()))
    }
    pub fn mul(&self, other: &Self) -> Result<Self, String> {
        if self.cols != other.rows { return Err("matrix shape mismatch".into()); }
        let mut out = vec![0.0; self.rows * other.cols];
        for r in 0..self.rows {
            for c in 0..other.cols {
                let mut sum = 0.0;
                for k in 0..self.cols { sum += self.get(r, k) * other.get(k, c); }
                out[r * other.cols + c] = sum;
            }
        }
        Ok(Self::from_f64(self.rows, other.cols, out))
    }
    fn check_shape(&self, other: &Self) -> Result<(), String> {
        if self.rows != other.rows || self.cols != other.cols { Err("matrix shape mismatch".into()) } else { Ok(()) }
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "[")?;
        for r in 0..self.rows {
            write!(f, "  ")?;
            for c in 0..self.cols { write!(f, "{} ", self.get(r, c))?; }
            writeln!(f)?;
        }
        write!(f, "]")
    }
}
