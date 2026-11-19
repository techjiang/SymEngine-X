use crate::expr::Expr;

/// 求解 ax^2 + bx + c = 0，使用数值稳定的求根公式。
pub fn solve_quadratic(a: f64, b: f64, c: f64) -> Vec<f64> {
    if a.abs() < 1e-12 {
        if b.abs() < 1e-12 { return vec![]; }
        return vec![-c / b];
    }
    let disc = b * b - 4.0 * a * c;
    if disc < -1e-12 { return vec![]; }
    let disc = disc.max(0.0).sqrt();
    // 避免 b 与 sqrt(disc) 接近时发生灾难性抵消。
    let q = -0.5 * (b + b.signum() * disc);
    if q.abs() < 1e-12 { return vec![-b / (2.0 * a)]; }
    let mut roots = vec![q / a, c / q];
    roots.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    roots
}

/// 求解一元三次方程，返回所有实根（含重根）。
pub fn solve_cubic(a: f64, b: f64, c: f64, d: f64) -> Vec<f64> {
    if a.abs() < 1e-12 { return solve_quadratic(b, c, d); }
    // 归一化并化为缺少二次项的 depressed cubic: t^3 + p t + q = 0
    let (bb, cc, dd) = (b / a, c / a, d / a);
    let shift = -bb / 3.0;
    let p = cc - bb * bb / 3.0;
    let q = 2.0 * bb.powi(3) / 27.0 - bb * cc / 3.0 + dd;
    let disc = (q / 2.0).powi(2) + (p / 3.0).powi(3);

    let mut roots: Vec<f64> = if disc > 1e-12 {
        let sqrt_disc = disc.sqrt();
        let u = (-q / 2.0 + sqrt_disc).cbrt();
        let v = (-q / 2.0 - sqrt_disc).cbrt();
        vec![shift + u + v]
    } else if disc >= -1e-12 {
        let u = (-q / 2.0).cbrt();
        vec![shift + 2.0 * u, shift - u]
    } else {
        let r = 2.0 * (-p / 3.0).sqrt();
        let phi = (-q / (2.0 * (-p / 3.0).powf(1.5))).clamp(-1.0, 1.0).acos() / 3.0;
        (0..3).map(|k| shift + r * (phi + 2.0 * std::f64::consts::PI * k as f64 / 3.0).cos()).collect()
    };
    roots.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    // 合并数值重复根
    roots.dedup_by(|a, b| (*a - *b).abs() < 1e-8);
    roots
}

/// 从表达式中提取一元多项式系数并求解。
pub fn solve(expr: &Expr, var: &str) -> Vec<f64> {
    let coeffs = super::polynomial::coefficients(expr, var);
    let mut trimmed = coeffs;
    while trimmed.last() == Some(&0) { trimmed.pop(); }
    match trimmed.len() {
        0 | 1 => vec![],
        2 => vec![-trimmed[0] as f64 / trimmed[1] as f64],
        3 => solve_quadratic(trimmed[2] as f64, trimmed[1] as f64, trimmed[0] as f64),
        4 => solve_cubic(trimmed[3] as f64, trimmed[2] as f64, trimmed[1] as f64, trimmed[0] as f64),
        _ => vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn quadratic_avoids_cancellation() {
        let roots = solve_quadratic(1.0, 1e8, 1.0);
        assert_eq!(roots.len(), 2);
        assert!((roots[0] + 1e8).abs() < 1.0);
        assert!((roots[1] + 1e-8).abs() < 1e-12);
    }
    #[test]
    fn cubic_has_three_roots() {
        let roots = solve_cubic(1.0, -6.0, 11.0, -6.0);
        assert_eq!(roots.len(), 3);
        for expected in [1.0, 2.0, 3.0] {
            assert!(roots.iter().any(|r| (r - expected).abs() < 1e-8));
        }
    }
}
