use crate::expr::Expr;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction { Both, Left, Right, PositiveInfinity, NegativeInfinity }

pub fn limit(expr: &Expr, var: &str, point: &Expr, direction: Direction) -> Expr {
    match direction {
        Direction::PositiveInfinity => limit_at_infinity(expr, var, true),
        Direction::NegativeInfinity => limit_at_infinity(expr, var, false),
        _ => {
            let direct = crate::calculus::integral::substitute(expr, var, point);
            let invalid = matches!(&direct, Expr::Rational(_, 0))
                || matches!(&direct, Expr::Float(f) if f.is_infinite() || f.is_nan());
            if !invalid {
                return crate::simplify::simplify(&direct);
            }
            let one = Expr::Rational(1, 1_000_000);
            let h = match direction {
                Direction::Left => Expr::neg(one),
                Direction::Right => one,
                _ => one,
            };
            let shifted = crate::calculus::integral::substitute(expr, var, &Expr::add(point.clone(), h));
            crate::simplify::simplify(&shifted)
        }
    }
}

fn limit_at_infinity(expr: &Expr, var: &str, positive: bool) -> Expr {
    match expr {
        Expr::Integer(_) | Expr::Rational(_, _) | Expr::Float(_) => expr.clone(),
        Expr::Symbol(s) if s == var => Expr::func(if positive { "+inf" } else { "-inf" }, vec![]),
        Expr::Pow(base, exp) if base.as_ref() == &Expr::symbol(var) => match (as_i64(exp), positive) {
            (Some(n), _) if n > 0 => Expr::func(if positive { "+inf" } else if n % 2 == 0 { "+inf" } else { "-inf" }, vec![]),
            (Some(n), _) if n < 0 => Expr::zero(),
            _ => expr.clone(),
        },
        Expr::Mul(items) => {
            let mut acc = Expr::one();
            for item in items { acc = Expr::mul(acc, limit_at_infinity(item, var, positive)); }
            acc
        }
        Expr::Add(items) => {
            let mut acc = Expr::zero();
            for item in items { acc = Expr::add(acc, limit_at_infinity(item, var, positive)); }
            acc
        }
        _ => crate::simplify::simplify(expr),
    }
}

fn as_i64(expr: &Expr) -> Option<i64> { if let Expr::Integer(n) = expr { Some(*n) } else { None } }
