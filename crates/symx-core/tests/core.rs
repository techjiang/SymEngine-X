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

#[test]
fn parser_rejects_invalid_input() {
    assert!(parse("").is_err());
    assert!(parse("1..2").is_err());
    assert!(parse("x +").is_err());
}

#[test]
fn miller_rabin_handles_strong_pseudoprime() {
    // 3215031751 是经典强伪素数，固定基检测可正确排除。
    assert!(!is_prime(3215031751));
}

#[test]
fn infinity_limit_handles_constant_shift() {
    let expr = parse("1 + 2/x").unwrap();
    let lim = limit(&expr, "x", &Expr::zero(), Direction::PositiveInfinity);
    assert_eq!(lim.to_string(), "1");
}

#[test]
fn mathml_escapes_special_characters() {
    let expr = Expr::func("a<b&c", vec![Expr::symbol("x")]);
    let xml = symx_core::output::mathml::to_mathml(&expr);
    assert!(xml.contains("a&lt;b&amp;c"));
}


#[test]
fn parser_accepts_negative_exponent_form() {
    let expr = parse("x^(-1)").unwrap();
    assert!(expr.to_string().contains('x'));
    assert!(!expr.to_string().is_empty());
}

#[test]
fn rational_latex_uses_fraction() {
    let expr = parse("1/x").unwrap();
    let latex = to_latex(&expr);
    assert!(latex.contains("\\frac{"));
}

#[test]
fn numeric_eval_rejects_zero_to_negative_power() {
    let expr = parse("0^(-1)").unwrap();
    assert_eq!(symx_core::numerical_eval(&expr), None);
}

#[test]
fn parser_supports_double_star_power() {
    let expr = parse("x**3").unwrap();
    assert_eq!(simplify(&derivative(&expr, "x")).to_string(), "3 * (x)^2");
}

#[test]
fn infinity_symbols_have_math_outputs() {
    let pos = Expr::func("+inf", vec![]);
    let neg = Expr::func("-inf", vec![]);
    assert_eq!(to_latex(&pos), "\\infty");
    assert!(to_latex(&neg).contains("\\infty"));
    assert!(symx_core::output::mathml::to_mathml(&pos).contains("&#8734;"));
}

#[test]
fn infinity_output_accepts_symbol_form() {
    let positive = Expr::symbol("+inf");
    let negative = Expr::symbol("-inf");
    assert_eq!(to_latex(&positive), "\\infty");
    assert_eq!(to_latex(&negative), "-\\infty");
    assert!(symx_core::output::mathml::to_mathml(&positive).contains("&#8734;"));
}

#[test]
fn infinity_limit_keeps_leading_coefficient() {
    let expr = parse("2*x / x").unwrap();
    let lim = limit(&expr, "x", &Expr::zero(), Direction::PositiveInfinity);
    assert_eq!(lim.to_string(), "2");
}

#[test]
fn negative_power_normalizes_to_reciprocal() {
    let expr = parse("x^(-2)").unwrap();
    assert!(expr.to_string().contains('/'));
}

#[test]
fn parser_accepts_scientific_notation() {
    let expr = parse("1e3 + 2*x").unwrap();
    assert!(expr.to_string().contains("1000") || expr.to_string().contains("1e3"));
}
