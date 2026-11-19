use crate::expr::Expr;

pub fn integrate(expr: &Expr, var: &str) -> Expr {
    let expr = crate::simplify::simplify(expr);
    let inner = match &expr {
        Expr::Add(items) => {
            let mut acc = Expr::zero();
            for item in items { acc = Expr::add(acc, integrate(item, var)); }
            return acc;
        }
        Expr::Mul(items) => {
            let mut scalar = None;
            let mut rest = Vec::new();
            for item in items {
                if matches!(item, Expr::Integer(_) | Expr::Rational(_, _) | Expr::Float(_)) { scalar = Some(item.clone()); }
                else { rest.push(item.clone()); }
            }
            if let Some(s) = scalar {
                if rest.len() == 1 {
                    return Expr::mul(s, integrate(&rest[0], var));
                }
            }
            &expr
        }
        other => other,
    };

    let result = match inner {
        Expr::Integer(n) => Expr::mul(Expr::integer(*n), Expr::symbol(var)),
        Expr::Rational(n, d) => Expr::mul(Expr::Rational(*n, *d), Expr::symbol(var)),
        Expr::Float(n) => Expr::mul(Expr::Float(*n), Expr::symbol(var)),
        Expr::Symbol(s) if s == var => Expr::div(Expr::pow(Expr::symbol(var), Expr::integer(2)), Expr::integer(2)),
        Expr::Symbol(_) => Expr::mul(inner.clone(), Expr::symbol(var)),
        Expr::Pow(base, exp) if base.as_ref() == &Expr::symbol(var) => {
            if let Expr::Integer(n) = exp.as_ref() {
                if *n == -1 { Expr::func("ln", vec![Expr::symbol(var)]) }
                else { Expr::div(Expr::pow(Expr::symbol(var), Expr::integer(n + 1)), Expr::integer(n + 1)) }
            } else { Expr::func("integrate", vec![inner.clone(), Expr::symbol(var)]) }
        }
        Expr::Function(name, args) if args.len() == 1 && args[0] == Expr::symbol(var) => match name.as_str() {
            "sin" => Expr::neg(Expr::func("cos", vec![Expr::symbol(var)])),
            "cos" => Expr::func("sin", vec![Expr::symbol(var)]),
            "exp" => Expr::func("exp", vec![Expr::symbol(var)]),
            "ln" | "log" => Expr::sub(
                Expr::mul(Expr::symbol(var), Expr::func("ln", vec![Expr::symbol(var)])),
                Expr::symbol(var),
            ),
            _ => Expr::func("integrate", vec![inner.clone(), Expr::symbol(var)]),
        },
        _ => Expr::func("integrate", vec![inner.clone(), Expr::symbol(var)]),
    };
    crate::simplify::simplify(&result)
}

pub fn definite_integral(expr: &Expr, var: &str, lower: &Expr, upper: &Expr) -> Expr {
    let antiderivative = integrate(expr, var);
    let at_upper = substitute(&antiderivative, var, upper);
    let at_lower = substitute(&antiderivative, var, lower);
    crate::simplify::simplify(&Expr::sub(at_upper, at_lower))
}

pub fn substitute(expr: &Expr, var: &str, value: &Expr) -> Expr {
    match expr {
        Expr::Symbol(s) if s == var => value.clone(),
        Expr::Add(items) | Expr::Mul(items) | Expr::Function(_, items) => {
            let items: Vec<Expr> = items.iter().map(|x| substitute(x, var, value)).collect();
            match expr {
                Expr::Add(_) => crate::simplify::simplify(&Expr::Add(items)),
                Expr::Mul(_) => crate::simplify::simplify(&Expr::Mul(items)),
                Expr::Function(name, _) => Expr::func(name.clone(), items),
                _ => unreachable!(),
            }
        }
        Expr::Pow(a, b) => Expr::pow(substitute(a, var, value), substitute(b, var, value)),
        Expr::Neg(a) => Expr::neg(substitute(a, var, value)),
        other => other.clone(),
    }
}
