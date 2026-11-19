use crate::expr::Expr;

pub fn taylor(expr: &Expr, var: &str, point: &Expr, order: usize) -> Expr {
    let mut acc = Expr::zero();
    let mut current = expr.clone();
    let mut factorial = 1i64;
    for n in 0..=order {
        if n > 0 {
            current = crate::simplify::simplify(&crate::calculus::derivative::derivative(&current, var));
            factorial *= n as i64;
        }
        let coeff = crate::calculus::integral::substitute(&current, var, point);
        let term = Expr::div(
            Expr::mul(coeff, Expr::pow(Expr::symbol(var), Expr::integer(n as i64))),
            Expr::integer(factorial),
        );
        acc = Expr::add(acc, term);
    }
    crate::simplify::simplify(&acc)
}
