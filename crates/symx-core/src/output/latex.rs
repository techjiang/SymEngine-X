use crate::expr::Expr;

pub fn to_latex(expr: &Expr) -> String {
    match expr {
        Expr::Integer(n) => n.to_string(),
        Expr::Rational(n, d) => format!("\\frac{{{n}}}{{{d}}}"),
        Expr::Float(n) => crate::number::float::format(*n),
        Expr::Symbol(s) => escape_symbol(s),
        Expr::Neg(inner) => format!("-{}", wrap(inner)),
        Expr::Add(items) => items.iter().map(to_latex).collect::<Vec<_>>().join(" + "),
        Expr::Mul(items) => items.iter().map(|item| latex_factor(item)).collect::<Vec<_>>().join(" "),
        Expr::Pow(base, exp) => {
            let exponent = to_latex(exp);
            let body = if exponent.contains('-') || exponent.contains('+') {
                format!("{{{exponent}}}")
            } else { exponent };
            format!("{}^{{{body}}}", wrap(base))
        }
        Expr::Function(name, args) => {
            let args = args.iter().map(to_latex).collect::<Vec<_>>().join(", ");
            match name.as_str() {
                "sin" | "cos" | "tan" | "ln" | "log" | "exp" => format!("{name}({args})"),
                "sqrt" => format!("\\sqrt{{{args}}}"),
                _ => format!("\\operatorname{{{name}}}({args})"),
            }
        }
    }
}

fn escape_symbol(name: &str) -> String {
    match name {
        "+inf" => "\\infty".into(),
        "-inf" => "-\\infty".into(),
        other => other.to_string(),
    }
}

fn wrap(expr: &Expr) -> String {
    match expr {
        Expr::Integer(_) | Expr::Rational(_, _) | Expr::Float(_) | Expr::Symbol(_) | Expr::Function(_, _) => to_latex(expr),
        _ => format!("({})", to_latex(expr)),
    }
}

fn latex_factor(expr: &Expr) -> String {
    match expr {
        Expr::Add(_) | Expr::Neg(_) => format!("({})", to_latex(expr)),
        _ => to_latex(expr),
    }
}
