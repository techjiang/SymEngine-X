use crate::expr::Expr;

/// 在点 point 处展开 order 阶 Taylor 多项式。
/// 使用 (var - point)^n 作为基，point 非 0 时结果也正确。
pub fn taylor(expr: &Expr, var: &str, point: &Expr, order: usize) -> Expr {
    let mut acc = Expr::zero();
    let mut current = expr.clone();
    let mut factorial = 1i64;
    for n in 0..=order {
        if n > 0 {
            current = crate::simplify::simplify(&crate::calculus::derivative::derivative(&current, var));
            factorial = factorial.checked_mul(n as i64).unwrap_or(i64::MAX);
        }
        let coeff = crate::calculus::integral::substitute(&current, var, point);
        if coeff == Expr::zero() { continue; }
        let shift = Expr::sub(Expr::symbol(var), point.clone());
        let term = Expr::div(
            Expr::mul(coeff, Expr::pow(shift.clone(), Expr::integer(n as i64))),
            Expr::integer(factorial),
        );
        acc = Expr::add(acc, term);
    }
    crate::simplify::simplify(&acc)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    #[test]
    fn taylor_of_polynomial_around_one() {
        let expr = parse("x^2").unwrap();
        let series = taylor(&expr, "x", &Expr::one(), 2);
        let value = crate::number::float::evaluate(&crate::calculus::integral::substitute(&series, "x", &Expr::integer(3)));
        assert_eq!(value, Some(9.0));
    }
}
