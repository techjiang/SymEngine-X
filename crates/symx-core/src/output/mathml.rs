use crate::expr::Expr;

pub fn to_mathml(expr: &Expr) -> String {
    match expr {
        Expr::Integer(n) => format!("<mn>{n}</mn>"),
        Expr::Rational(n, d) => format!("<mfrac><mn>{n}</mn><mn>{d}</mn></mfrac>"),
        Expr::Float(n) => format!("<mn>{n}</mn>"),
        Expr::Symbol(s) => format!("<mi>{}</mi>", escape(s)),
        Expr::Neg(inner) => format!("<mrow><mo>-</mo>{}</mrow>", to_mathml(inner)),
        Expr::Add(items) => infix_row(items, "+"),
        Expr::Mul(items) => infix_row(items, "&times;"),
        Expr::Pow(base, exp) => format!("<msup>{}<mrow>{}</mrow></msup>", wrap(base), to_mathml(exp)),
        Expr::Function(name, args) => {
            let inner = infix_row(args, ",");
            match name.as_str() {
                "sqrt" if args.len() == 1 => format!("<msqrt>{}</msqrt>", to_mathml(&args[0])),
                _ => format!("<mrow><mi>{}</mi><mo>&#8288;(</mo>{}<mo>)</mo></mrow>", escape(name), inner),
            }
        }
    }
}

fn infix_row(items: &[Expr], op: &str) -> String {
    let mut parts = Vec::new();
    for (i, item) in items.iter().enumerate() {
        if i > 0 { parts.push(format!("<mo>{op}</mo>")); }
        parts.push(to_mathml(item));
    }
    format!("<mrow>{}</mrow>", parts.concat())
}

fn wrap(expr: &Expr) -> String {
    match expr {
        Expr::Integer(_) | Expr::Rational(_, _) | Expr::Float(_) | Expr::Symbol(_) => to_mathml(expr),
        _ => format!("<mrow>{}</mrow>", to_mathml(expr)),
    }
}

fn escape(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}
