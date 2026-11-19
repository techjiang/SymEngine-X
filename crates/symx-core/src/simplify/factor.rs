use crate::expr::Expr;

pub fn factor(expr: &Expr) -> Expr {
    if let Expr::Add(items) = expr {
        let normalized = super::rules::simplify(expr);
        let items = match &normalized { Expr::Add(items) => items, _ => items };
        if items.len() == 3 {
            let terms: Vec<(Expr, Expr)> = items.iter().map(as_square_term).collect();
            // a^2 ± 2ab + b^2 => (a ± b)^2
            for variant in [1i64, -1] {
                for i in 0..3 {
                    for j in (i + 1)..3 {
                        let k = 3 - i - j;
                        let (a, _) = &terms[i];
                        let (b, _) = &terms[j];
                        let (mid, _) = &terms[k];
                        let target = super::rules::simplify(&Expr::mul(Expr::mul(Expr::integer(2 * variant), a.clone()), b.clone()));
                        if target == super::rules::simplify(mid) {
                            let sum = if variant > 0 { Expr::add(a.clone(), b.clone()) } else { Expr::add(a.clone(), Expr::neg(b.clone())) };
                            return Expr::mul(sum.clone(), sum);
                        }
                    }
                }
            }
        }
    }
    expr.clone()
}

fn as_square_term(expr: &Expr) -> (Expr, Expr) {
    match expr {
        Expr::Pow(base, exp) if **exp == Expr::Integer(2) => ((**base).clone(), expr.clone()),
        Expr::Neg(inner) => { let (base, original) = as_square_term(inner); (Expr::neg(base), original) }
        other => (other.clone(), other.clone()),
    }
}
