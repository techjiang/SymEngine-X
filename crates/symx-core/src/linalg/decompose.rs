use super::matrix::Matrix;

/// 带部分主元选取的 LU 分解，返回 (L, U, 行交换索引)。
pub fn lu(m: &Matrix) -> Result<(Matrix, Matrix, Vec<usize>), String> {
    if m.rows != m.cols { return Err("LU requires square matrix".into()); }
    let n = m.rows;
    let mut a = vec![0.0; n * n];
    for r in 0..n { for c in 0..n { a[r * n + c] = m.get(r, c); } }
    let mut l = vec![0.0; n * n];
    let mut pivots: Vec<usize> = (0..n).collect();

    for k in 0..n {
        // 选取当前列绝对值最大的行作为主元。
        let mut pivot = k;
        for r in (k + 1)..n {
            if a[r * n + k].abs() > a[pivot * n + k].abs() { pivot = r; }
        }
        if a[pivot * n + k].abs() < 1e-12 { return Err("matrix is singular".into()); }
        if pivot != k {
            for c in 0..n { a.swap(k * n + c, pivot * n + c); }
            pivots.swap(k, pivot);
            for c in 0..k { l.swap(k * n + c, pivot * n + c); }
        }
        l[k * n + k] = 1.0;
        for r in (k + 1)..n {
            l[r * n + k] = a[r * n + k] / a[k * n + k];
            for c in k..n { a[r * n + c] -= l[r * n + k] * a[k * n + c]; }
        }
    }
    let lower = Matrix::from_f64(n, n, l);
    let upper = Matrix::from_f64(n, n, a);
    Ok((lower, upper, pivots))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lu_handles_zero_leading_pivot() {
        let m = Matrix::new(2, 2, &[0.0, 1.0, 1.0, 1.0]);
        let (l, u, pivots) = lu(&m).unwrap();
        assert_eq!(pivots, vec![1, 0]);
        assert!((l.get(1, 0) - 0.0).abs() < 1e-12);
        assert!((u.get(0, 0) - 1.0).abs() < 1e-12);
    }
    #[test]
    fn singular_matrix_is_rejected() {
        let m = Matrix::new(2, 2, &[1.0, 2.0, 2.0, 4.0]);
        assert!(lu(&m).is_err());
    }
}
