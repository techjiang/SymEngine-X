use crate::expr::Expr;

pub fn collect(expr: &Expr, symbol: &str) -> Expr {
    let expanded = super::expand::expand(expr);
    let mut groups: Vec<(i64, Expr)> = Vec::new();
    if let Expr::Add(items) = &expanded {
        for item in items {
            let (coeff, power) = extract_coeff_power(item, symbol);
            if let Some(group) = groups.iter_mut().find(|(_, p)| p == &power) { group.0 += coeff; }
            else { groups.push((coeff, power)); }
        }
    } else {
        let (coeff, power) = extract_coeff_power(&expanded, symbol);
        groups.push((coeff, power));
    }
    groups.sort_by(|a, b| format!("{:?}", b.1).cmp(&format!("{:?}", a.1)));
    let mut acc = Expr::zero();
    for (coeff, power) in groups {
        if coeff == 0 { continue; }
        let term = if power == Expr::zero() {
            Expr::integer(coeff)
        } else if power == Expr::one() {
            Expr::mul(Expr::integer(coeff), Expr::symbol(symbol))
        } else {
            Expr::mul(Expr::integer(coeff), Expr::pow(Expr::symbol(symbol), power))
        };
        acc = Expr::add(acc, term);
    }
    super::rules::simplify(&acc)
}

fn extract_coeff_power(expr: &Expr, symbol: &str) -> (i64, Expr) {
    if let Expr::Mul(items) = expr {
        let mut coeff = 1;
        let mut power = Expr::zero();
        for item in items {
            match item {
                Expr::Integer(n) => coeff *= n,
                Expr::Symbol(s) if s == symbol => power = Expr::one(),
                Expr::Pow(base, exp) if **base == Expr::symbol(symbol) && **exp == Expr::Integer(1) => power = (**exp).clone(),
                Expr::Pow(base, exp) if **base == Expr::symbol(symbol) => power = (**exp).clone(),
                _ => {}
            }
        }
        (coeff, power)
    } else if expr == &Expr::symbol(symbol) {
        (1, Expr::one())
    } else if let Expr::Pow(base, exp) = expr {
        if **base == Expr::symbol(symbol) { (1, (**exp).clone()) } else { (1, Expr::zero()) }
    } else {
        (1, Expr::zero())
    }
}
