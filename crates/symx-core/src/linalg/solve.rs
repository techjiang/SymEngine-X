use super::matrix::Matrix;

impl Matrix {
    pub fn inverse(&self) -> Result<Matrix, String> {
        if self.rows != self.cols { return Err("inverse requires square matrix".into()); }
        let n = self.rows;
        let mut aug = vec![0.0; n * 2 * n];
        for r in 0..n {
            for c in 0..n { aug[r * 2 * n + c] = self.get(r, c); }
            aug[r * 2 * n + n + r] = 1.0;
        }
        for col in 0..n {
            let mut pivot = col;
            for r in col + 1..n {
                if aug[r * 2 * n + col].abs() > aug[pivot * 2 * n + col].abs() { pivot = r; }
            }
            if aug[pivot * 2 * n + col].abs() < 1e-12 { return Err("matrix is singular".into()); }
            if pivot != col {
                for c in 0..2 * n { aug.swap(col * 2 * n + c, pivot * 2 * n + c); }
            }
            let p = aug[col * 2 * n + col];
            for c in 0..2 * n { aug[col * 2 * n + c] /= p; }
            for r in 0..n {
                if r == col { continue; }
                let factor = aug[r * 2 * n + col];
                for c in 0..2 * n { aug[r * 2 * n + c] -= factor * aug[col * 2 * n + c]; }
            }
        }
        let mut inv = vec![0.0; n * n];
        for r in 0..n { for c in 0..n { inv[r * n + c] = aug[r * 2 * n + n + c]; } }
        Ok(Matrix::from_f64(n, n, inv))
    }
}
