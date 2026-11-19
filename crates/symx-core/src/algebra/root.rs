use crate::expr::Expr;

pub fn solve_quadratic(a: f64, b: f64, c: f64) -> Vec<f64> {
    if a.abs() < 1e-12 {
        if b.abs() < 1e-12 { return vec![]; }
        return vec![-c / b];
    }
    let disc = b * b - 4.0 * a * c;
    if disc < 0.0 { return vec![]; }
    let root = disc.sqrt();
    vec![(-b + root) / (2.0 * a), (-b - root) / (2.0 * a)]
}

pub fn solve(expr: &Expr, var: &str) -> Vec<f64> {
    let coeffs = super::polynomial::coefficients(expr, var);
    let mut trimmed = coeffs;
    while trimmed.last() == Some(&0) { trimmed.pop(); }
    match trimmed.len() {
        0 | 1 => vec![],
        2 => vec![-trimmed[0] as f64 / trimmed[1] as f64],
        3 => solve_quadratic(trimmed[2] as f64, trimmed[1] as f64, trimmed[0] as f64),
        _ => vec![],
    }
}
