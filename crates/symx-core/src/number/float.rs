pub fn evaluate(expr: &crate::expr::Expr) -> Option<f64> {
    use crate::expr::Expr;
    match expr {
        Expr::Integer(n) => Some(*n as f64),
        Expr::Rational(n, d) if *d != 0 => Some(*n as f64 / *d as f64),
        Expr::Float(n) => Some(*n),
        Expr::Neg(a) => evaluate(a).map(|x| -x),
        Expr::Add(items) => items.iter().map(evaluate).sum::<Option<f64>>(),
        Expr::Mul(items) => items.iter().map(evaluate).reduce(|a, b| a.zip(b).map(|(x, y)| x * y)).flatten(),
        Expr::Pow(base, exp) => evaluate(base).zip(evaluate(exp)).map(|(b, e)| b.powf(e)),
        Expr::Function(name, args) => {
            let vals: Option<Vec<f64>> = args.iter().map(evaluate).collect();
            let vals = vals?;
            match (name.as_str(), vals.as_slice()) {
                ("sin", [x]) => Some(x.sin()),
                ("cos", [x]) => Some(x.cos()),
                ("tan", [x]) => Some(x.tan()),
                ("exp", [x]) => Some(x.exp()),
                ("ln" | "log", [x]) => Some(x.ln()),
                ("sqrt", [x]) => Some(x.sqrt()),
                _ => None,
            }
        }
        _ => None,
    }
}
