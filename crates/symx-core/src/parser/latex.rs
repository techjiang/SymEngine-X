use super::infix;

pub fn parse(input: &str) -> Result<crate::expr::Expr, String> {
    let normalized = input
        .trim()
        .replace("\\left", "")
        .replace("\\right", "")
        .replace("\\cdot", "*")
        .replace("\\times", "*")
        .replace("\\frac", "/")
        .replace("^{", "^(")
        .replace("}{", ")/(") // 保留为一元函数括号，如 sqrt{(x)}
        .replace(r"\sqrt", "sqrt")
        .replace('{', "(")
        .replace('}', ")")
        .replace("\\pi", "(3.141592653589793)")
        .replace(' ', "");
    infix::parse(&normalized)
}
