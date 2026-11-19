use crate::expr::Expr;

pub fn factor(expr: &Expr) -> Expr {
    if let Expr::Add(items) = expr {
        if items.len() == 3 {
            let terms: Vec<Option<(Expr, Expr)>> = items.iter().map(as_square_term).collect();
            if let [Some((a, _)), Some((mid, _)), Some((c, _))] = &terms[..] {
                let two_ac = Expr::mul(Expr::mul(Expr::integer(2), a.clone()), c.clone());
                if super::rules::simplify(&two_ac) == super::rules::simplify(mid) {
                    return Expr::mul(Expr::add(a.clone(), c.clone()), Expr::add(a.clone(), c.clone()));
                }
            }
        }
    }
    expr.clone()
}

fn as_square_term(expr: &Expr) -> Option<(Expr, Expr)> {
    match expr {
        Expr::Pow(base, exp) if **exp == Expr::Integer(2) => Some(((**base).clone(), expr.clone())),
        other => Some((other.clone(), other.clone())),
    }
}
