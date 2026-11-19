use super::matrix::Matrix;

pub fn lu(m: &Matrix) -> Result<(Matrix, Matrix), String> {
    if m.rows != m.cols { return Err("LU requires square matrix".into()); }
    let n = m.rows;
    let mut l = vec![0.0; n * n];
    let mut u = vec![0.0; n * n];
    for r in 0..n {
        for c in 0..n {
            let mut sum = m.get(r, c);
            for k in 0..r { sum -= l[r * n + k] * u[k * n + c]; }
            if r <= c { u[r * n + c] = sum; l[r * n + r] = 1.0; }
            else { l[r * n + c] = sum / u[c * n + c]; }
        }
    }
    Ok((Matrix::from_f64(n, n, l), Matrix::from_f64(n, n, u)))
}
