use crate::expr::Expr;

pub fn to_mathml(expr: &Expr) -> String {
    match expr {
        Expr::Integer(n) => format!("<mn>{n}</mn>"),
        Expr::Float(n) => format!("<mn>{n}</mn>"),
        Expr::Symbol(s) => format!("<mi>{s}</mi>"),
        Expr::Add(items) | Expr::Mul(items) => {
            let op = if matches!(expr, Expr::Add(_)) { "+" } else { "&times;" };
            let body = items.iter().map(to_mathml).collect::<Vec<_>>().join("");
            format!("<mrow>{body}<mo>{op}</mo></mrow>")
        }
        other => format!("<mi>{}</mi>", crate::output::string::to_string(other)),
    }
}
