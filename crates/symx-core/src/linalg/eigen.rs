use super::matrix::Matrix;

pub fn eigenvalues(m: &Matrix) -> Result<Vec<f64>, String> {
    if m.rows != m.cols || m.rows != 2 { return Err("eigenvalues currently support 2x2 matrices".into()); }
    let (a, b, c, d) = (m.get(0, 0), m.get(0, 1), m.get(1, 0), m.get(1, 1));
    let trace = a + d;
    let det = a * d - b * c;
    let disc = trace * trace - 4.0 * det;
    if disc < 0.0 { return Err("complex eigenvalues are not supported yet".into()); }
    let root = disc.sqrt();
    Ok(vec![(trace + root) / 2.0, (trace - root) / 2.0])
}
