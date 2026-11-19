use crate::expr::Expr;

pub fn solve_linear_first_order(y_prime: &Expr, y: &str, x: &str) -> Expr {
    let integral = crate::calculus::integral::integrate(y_prime, x);
    Expr::add(integral, Expr::func("C", vec![Expr::symbol(y)]))
}
