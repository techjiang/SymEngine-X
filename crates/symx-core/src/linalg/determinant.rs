use super::matrix::Matrix;

pub fn determinant(m: &Matrix) -> Result<f64, String> {
    if m.rows != m.cols { return Err("determinant requires square matrix".into()); }
    let mut data = m.data.clone();
    let n = m.rows;
    let mut det = 1.0;
    for col in 0..n {
        let mut pivot = col;
        for r in col + 1..n {
            if data[r * n + col].abs() > data[pivot * n + col].abs() { pivot = r; }
        }
        if (data[pivot * n + col]).abs() < 1e-12 { return Ok(0.0); }
        if pivot != col {
            for c in 0..n { data.swap(col * n + c, pivot * n + c); }
            det = -det;
        }
        let pivot_val = data[col * n + col];
        det *= pivot_val;
        for c in 0..n { data[col * n + c] /= pivot_val; }
        for r in 0..n {
            if r == col { continue; }
            let factor = data[r * n + col];
            for c in 0..n { data[r * n + c] -= factor * data[col * n + c]; }
        }
    }
    Ok(det)
}
