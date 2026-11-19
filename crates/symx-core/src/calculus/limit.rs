use crate::expr::Expr;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction { Both, Left, Right, PositiveInfinity, NegativeInfinity }

impl std::str::FromStr for Direction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "both" | "0" => Ok(Direction::Both),
            "left" | "-" => Ok(Direction::Left),
            "right" | "+" => Ok(Direction::Right),
            "+inf" | "inf" => Ok(Direction::PositiveInfinity),
            "-inf" => Ok(Direction::NegativeInfinity),
            _ => Err(format!("unknown direction: {s}")),
        }
    }
}

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
    if !contains_var(expr, var) { return crate::simplify::simplify(expr); }
    if let Some(value) = rational_infinity_limit(expr, var) { return value; }
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

fn rational_infinity_limit(expr: &Expr, var: &str) -> Option<Expr> {
    if let Expr::Mul(items) = expr {
        if items.len() == 2 {
            let numerator = &items[0];
            let denominator = &items[1];
            if let Expr::Pow(den_base, den_exp) = denominator {
                if den_base.as_ref() != &Expr::symbol(var) { return None; }
                if let Expr::Integer(den_n) = den_exp.as_ref() {
                    if let Expr::Pow(num_base, num_exp) = numerator {
                        if num_base.as_ref() != &Expr::symbol(var) { return None; }
                        if let Expr::Integer(num_n) = num_exp.as_ref() {
                            if num_n == den_n { return Some(Expr::one()); }
                            if num_n < den_n { return Some(Expr::zero()); }
                        }
                    }
                }
            }
        }
    }
    None
}

fn as_i64(expr: &Expr) -> Option<i64> { if let Expr::Integer(n) = expr { Some(*n) } else { None } }

fn contains_var(expr: &Expr, var: &str) -> bool {
    match expr {
        Expr::Symbol(s) => s == var,
        Expr::Add(items) | Expr::Mul(items) | Expr::Function(_, items) => items.iter().any(|e| contains_var(e, var)),
        Expr::Neg(e) => contains_var(e, var),
        Expr::Pow(a, b) => contains_var(a, var) || contains_var(b, var),
        _ => false,
    }
}
