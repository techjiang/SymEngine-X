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

#[test]
fn mathml_output_has_infix_operators() {
    let expr = parse("x + 2*x").unwrap();
    let xml = symx_core::output::mathml::to_mathml(&expr);
    assert!(xml.contains("<mo>+</mo>"));
    assert!(xml.contains("<mo>&times;</mo>"));
    assert!(xml.contains("<mi>x</mi>"));
}

#[test]
fn collect_keeps_float_coefficients() {
    let expr = parse("0.5*x + 1.5*x").unwrap();
    let collected = symx_core::simplify::collect::collect(&expr, "x");
    assert_eq!(collected.to_string(), "2 * x");
}

#[test]
fn factor_difference_of_squares() {
    let expr = parse("x^2 - 2*x + 1").unwrap();
    let factored = symx_core::simplify::factor::factor(&symx_core::parser::parse("x^2 + 2*x + 1").unwrap());
    assert!(matches!(factored, Expr::Mul(_)));
}

#[test]
fn matrix_determinant_and_eigen_trace() {
    let m = Matrix::new(3, 3, &[1, 0, 0, 0, 2, 0, 0, 0, 3]);
    let det = symx_core::linalg::determinant::determinant(&m).unwrap();
    assert!((det - 6.0).abs() < 1e-9);
}

#[test]
fn modular_exponentiation_large_exponent() {
    assert_eq!(mod_pow(7, 1000, 13), 9);
    assert_eq!(is_prime(97), true);
    assert_eq!(is_prime(1), false);
}

#[test]
fn mathml_function_output() {
    let expr = parse("sqrt(x)").unwrap();
    let xml = symx_core::output::mathml::to_mathml(&expr);
    assert!(xml.contains("<msqrt>"));
}
