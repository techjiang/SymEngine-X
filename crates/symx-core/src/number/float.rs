use crate::expr::Expr;

/// 将浮点表达式求值；除数为 0 或结果非有限数时返回 None。
pub fn evaluate(expr: &Expr) -> Option<f64> {
    let value = eval(expr)?;
    if value.is_finite() { Some(value) } else { None }
}

fn eval(expr: &Expr) -> Option<f64> {
    match expr {
        Expr::Integer(n) => Some(*n as f64),
        Expr::Rational(n, d) if *d != 0 => Some(*n as f64 / *d as f64),
        Expr::Rational(_, 0) => None,
        Expr::Float(n) => Some(*n),
        Expr::Add(items) => items.iter().try_fold(0.0, |acc, item| Some(acc + eval(item)?)),
        Expr::Mul(items) => items.iter().try_fold(1.0, |acc, item| Some(acc * eval(item)?)),
        Expr::Neg(inner) => eval(inner).map(|n| -n),
        Expr::Pow(base, exp) => {
            let b = eval(base)?;
            let e = eval(exp)?;
            let value = b.powf(e);
            if value.is_finite() { Some(value) } else { None }
        }
        Expr::Function(name, args) => match (name.as_str(), args.as_slice()) {
            ("sin", [x]) => eval(x).map(f64::sin),
            ("cos", [x]) => eval(x).map(f64::cos),
            ("tan", [x]) => eval(x).map(f64::tan),
            ("exp", [x]) => eval(x).map(f64::exp),
            ("ln" | "log", [x]) => eval(x).and_then(|n| if n > 0.0 { Some(n.ln()) } else { None }),
            ("sqrt", [x]) => eval(x).and_then(|n| if n >= 0.0 { Some(n.sqrt()) } else { None }),
            _ => None,
        },
        Expr::Symbol(_) => None,
    }
}

/// 输出用于 LaTeX/MathML 的稳定浮点文本，避免集成级浮点噪声。
pub fn format(n: f64) -> String {
    if n == 0.0 { return "0".into(); }
    let rounded = (n * 1e12).round() / 1e12;
    let text = format!("{rounded}");
    text.trim_end_matches('0').trim_end_matches('.').to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_non_finite_values() {
        assert_eq!(evaluate(&Expr::div(Expr::one(), Expr::zero())), None);
        assert_eq!(evaluate(&Expr::func("ln", vec![Expr::integer(-1)])), None);
    }
    #[test]
    fn stable_format() {
        assert_eq!(format(1.0 + 1e-15), "1");
        assert_eq!(format(1.25), "1.25");
    }
}
