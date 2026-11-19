use crate::expr::Expr;

pub fn derivative(expr: &Expr, var: &str) -> Expr {
    match expr {
        Expr::Integer(_) | Expr::Rational(_, _) | Expr::Float(_) => Expr::zero(),
        Expr::Symbol(name) => if name == var { Expr::one() } else { Expr::zero() },
        Expr::Add(items) => {
            let mut acc = Expr::zero();
            for item in items { acc = Expr::add(acc, derivative(item, var)); }
            simplify(&acc)
        }
        Expr::Mul(items) => {
            let mut acc = Expr::zero();
            for (i, item) in items.iter().enumerate() {
                let mut term = derivative(item, var);
                for (j, other) in items.iter().enumerate() {
                    if i == j { continue; }
                    term = Expr::mul(term, other.clone());
                }
                acc = Expr::add(acc, term);
            }
            simplify(&acc)
        }
        Expr::Neg(inner) => Expr::neg(derivative(inner, var)),
        Expr::Pow(base, exp) => {
            let base = simplify(base);
            let exp = simplify(exp);
            if !contains_var(&exp, var) {
                let n = as_i64(&exp).unwrap_or(1);
                let new_exp = Expr::pow(base.clone(), Expr::integer(n - 1));
                return simplify(&Expr::mul(Expr::mul(exp.clone(), new_exp), derivative(&base, var)));
            }
            if base == Expr::symbol(var) {
                return expr.clone();
            }
            let inner = derivative(&base, var);
            if inner == Expr::zero() { return Expr::zero(); }
            Expr::mul(expr.clone(), Expr::mul(inner, Expr::div(Expr::one(), base.clone())))
        }
        Expr::Function(name, args) if args.len() == 1 => {
            let arg = &args[0];
            let inner = derivative(arg, var);
            if inner == Expr::zero() { return Expr::zero(); }
            let outer = match name.as_str() {
                "sin" => Expr::func("cos", vec![arg.clone()]),
                "cos" => Expr::neg(Expr::func("sin", vec![arg.clone()])),
                "exp" => Expr::func("exp", vec![arg.clone()]),
                "ln" | "log" => Expr::div(Expr::one(), arg.clone()),
                "sqrt" => Expr::div(
                    Expr::one(),
                    Expr::mul(Expr::integer(2), Expr::func("sqrt", vec![arg.clone()])),
                ),
                _ => Expr::func("derivative", vec![Expr::symbol(name), arg.clone()]),
            };
            simplify(&Expr::mul(outer, inner))
        }
        _ => Expr::zero(),
    }
}

pub fn nth_derivative(expr: &Expr, var: &str, n: usize) -> Expr {
    let mut current = expr.clone();
    for _ in 0..n { current = simplify(&derivative(&current, var)); }
    current
}

fn simplify(expr: &Expr) -> Expr { crate::simplify::simplify(expr) }

fn contains_var(expr: &Expr, var: &str) -> bool {
    match expr {
        Expr::Symbol(s) => s == var,
        Expr::Add(items) | Expr::Mul(items) | Expr::Function(_, items) => items.iter().any(|e| contains_var(e, var)),
        Expr::Neg(e) => contains_var(e, var),
        Expr::Pow(a, b) => contains_var(a, var) || contains_var(b, var),
        _ => false,
    }
}

fn as_i64(expr: &Expr) -> Option<i64> {
    match expr { Expr::Integer(n) => Some(*n), _ => None }
}
