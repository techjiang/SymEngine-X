use symx_core::prelude::*;

#[test]
fn parses_and_simulates_basic_flow() {
    let x = Expr::symbol("x");
    let expr = Expr::mul(Expr::add(x.clone(), Expr::integer(2)), Expr::sub(x.clone(), Expr::integer(3)));
    let simplified = simplify(&expr);
    assert!(simplified.to_string().contains('x'));
    assert_eq!(evaluate(&Expr::add(Expr::integer(2), Expr::integer(3))), Some(5.0));
}

#[test]
fn derives_polynomial() {
    let expr = parse("x^3 + 2*x").unwrap();
    assert_eq!(simplify(&derivative(&expr, "x")).to_string(), "2 + 3 * (x)^2");
}

#[test]
fn integrates_and_outputs_latex() {
    let expr = parse("x^2 + sin(x)").unwrap();
    let integral = integrate(&expr, "x");
    assert!(integral.to_string().contains("cos"));
    assert!(to_latex(&expr).contains("x"));
}

#[test]
fn matrix_operations() {
    let m = Matrix::new(2, 2, &[1, 2, 3, 4]);
    assert_eq!(crate::matrix_det(&m), -2.0);
    let inv = m.inverse().unwrap();
    assert!((inv.get(0, 0) + 2.0).abs() < 1e-9);
}

fn matrix_det(m: &Matrix) -> f64 { symx_core::linalg::determinant::determinant(m).unwrap() }

#[test]
fn number_theory_and_limit() {
    assert_eq!(factorize(60), vec![2, 2, 3, 5]);
    assert_eq!(mod_pow(2, 10, 1000), 24);
    assert_eq!(mod_inverse(3, 11), Some(4));
    let expr = parse("1/x").unwrap();
    let lim = limit(&expr, "x", &Expr::zero(), Direction::Right);
    assert!(!lim.to_string().is_empty());
}
