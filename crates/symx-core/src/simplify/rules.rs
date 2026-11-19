use crate::expr::Expr;

pub fn simplify(expr: &Expr) -> Expr {
    let expr = match expr {
        Expr::Add(items) => {
            let items: Vec<Expr> = items.iter().map(simplify).collect();
            combine_terms(&items, false)
        }
        Expr::Mul(items) => {
            let items: Vec<Expr> = items.iter().map(simplify).collect();
            combine_terms(&items, true)
        }
        Expr::Pow(base, exp) => {
            let base = simplify(base);
            let exp = simplify(exp);
            if exp == Expr::zero() { Expr::one() }
            else if exp == Expr::one() { base }
            else { Expr::pow(base, exp) }
        }
        Expr::Neg(inner) => Expr::neg(simplify(inner)),
        Expr::Function(name, args) => Expr::func(name.clone(), args.iter().map(simplify).collect()),
        other => other.clone(),
    };
    apply_identities(&expr)
}

fn apply_identities(expr: &Expr) -> Expr {
    match expr {
        Expr::Mul(items) if items.iter().any(|x| x == &Expr::zero()) => Expr::zero(),
        Expr::Mul(items) if items.len() == 2 && matches!((&items[0], &items[1]), (Expr::Integer(1), _) | (_, Expr::Integer(1))) => {
            if items[0] == Expr::one() { items[1].clone() } else { items[0].clone() }
        }
        _ => expr.clone(),
    }
}

fn combine_terms(items: &[Expr], mul: bool) -> Expr {
    let mut constants: Vec<f64> = Vec::new();
    let mut others: Vec<Expr> = Vec::new();
    for item in items {
        match item {
            Expr::Integer(n) if mul => constants.push(*n as f64),
            Expr::Rational(n, d) if mul => constants.push(*n as f64 / *d as f64),
            Expr::Float(n) if mul => constants.push(*n),
            Expr::Integer(n) if !mul => constants.push(*n as f64),
            Expr::Rational(n, d) if !mul => constants.push(*n as f64 / *d as f64),
            Expr::Float(n) if !mul => constants.push(*n),
            _ => others.push(item.clone()),
        }
    }
    let neutral = if mul { 1.0 } else { 0.0 };
    let constant = if constants.is_empty() { neutral } else { constants.into_iter().fold(neutral, |a, b| if mul { a * b } else { a + b }) };
    if mul && constant == 0.0 { return Expr::zero(); }
    if mul && constant != 1.0 { others.insert(0, float_or_int(constant)); }
    if !mul && constant != 0.0 { others.insert(0, float_or_int(constant)); }
    if mul {
        let mut acc = Expr::one();
        for item in others { acc = Expr::mul(acc, item); }
        acc
    } else {
        let mut acc = Expr::zero();
        for item in others { acc = Expr::add(acc, item); }
        acc
    }
}

fn float_or_int(n: f64) -> Expr {
    if n.fract() == 0.0 && n.abs() < i64::MAX as f64 { Expr::Integer(n as i64) } else { Expr::Float(n) }
}
