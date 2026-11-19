use crate::expr::Expr;

pub fn coefficients(expr: &Expr, var: &str) -> Vec<i64> {
    let collected = crate::simplify::collect::collect(expr, var);
    let mut max_deg = 0;
    collect_pairs(&collected, var, &mut max_deg);
    let mut coeffs = vec![0i64; max_deg + 1];
    for (coeff, deg) in pairs(&collected, var) { coeffs[deg] = coeff; }
    coeffs
}

fn collect_pairs(expr: &Expr, var: &str, max_deg: &mut usize) {
    for (_, deg) in pairs(expr, var) { *max_deg = (*max_deg).max(deg); }
}

fn pairs(expr: &Expr, var: &str) -> Vec<(i64, usize)> {
    let mut out = Vec::new();
    match expr {
        Expr::Add(items) => { for item in items { out.extend(pairs(item, var)); } }
        Expr::Integer(n) => out.push((*n, 0)),
        Expr::Rational(n, d) if *d != 0 => out.push((n / d, 0)),
        Expr::Symbol(s) if s == var => out.push((1, 1)),
        Expr::Mul(items) => {
            let mut coeff = 1i64;
            let mut deg = 0usize;
            for item in items {
                match item {
                    Expr::Integer(n) => coeff *= n,
                    Expr::Symbol(s) if s == var => deg += 1,
                    Expr::Pow(base, exp) if base.as_ref() == &Expr::symbol(var) => {
                        if let Expr::Integer(n) = exp.as_ref() { deg = (*n).max(0) as usize; }
                    }
                    _ => {}
                }
            }
            out.push((coeff, deg));
        }
        Expr::Pow(base, exp) if base.as_ref() == &Expr::symbol(var) => {
            if let Expr::Integer(n) = exp.as_ref() { out.push((1, (*n).max(0) as usize)); }
        }
        _ => {}
    }
    out
}
