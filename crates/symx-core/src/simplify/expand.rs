use crate::expr::Expr;

pub fn expand(expr: &Expr) -> Expr {
    match expr {
        Expr::Mul(items) => {
            let expanded: Vec<Expr> = items.iter().map(expand).collect();
            let mut acc = Expr::one();
            for item in expanded { acc = distribute_mul(acc, item); }
            super::rules::simplify(&acc)
        }
        Expr::Add(items) => {
            let mut acc = Expr::zero();
            for item in items { acc = Expr::add(acc, expand(item)); }
            acc
        }
        Expr::Pow(base, exp) => {
            if let Expr::Integer(n) = exp.as_ref() {
                if *n > 0 && *n <= 16 {
                    let b = expand(base);
                    let mut acc = Expr::one();
                    for _ in 0..*n { acc = distribute_mul(acc, b.clone()); }
                    return super::rules::simplify(&acc);
                }
            }
            expr.clone()
        }
        other => other.clone(),
    }
}

fn distribute_mul(a: Expr, b: Expr) -> Expr {
    match (a, b) {
        (Expr::Add(items), b) | (b, Expr::Add(items)) => {
            let mut acc = Expr::zero();
            for item in items { acc = Expr::add(acc, distribute_mul(b.clone(), item)); }
            acc
        }
        (a, b) => Expr::mul(a, b),
    }
}
