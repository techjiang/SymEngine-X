pub mod algebra;
pub mod calculus;
pub mod expr;
pub mod linalg;
pub mod number;
pub mod output;
pub mod parser;
pub mod simplify;

pub use expr::Expr;
pub use linalg::Matrix;

/// 引擎版本号。
pub const VERSION: &str = "1.0.0";

/// 解析中缀表达式。
///
/// # Examples
///
/// ```
/// use symx_core::parse;
/// let expr = parse("x^2 + 2*x + 1").unwrap();
/// assert!(expr.to_string().contains('x'));
/// ```
pub fn parse_checked(input: &str) -> Result<Expr, String> { parse(input) }


pub fn parse(input: &str) -> Result<Expr, String> { parser::parse(input) }
pub fn parse_latex(input: &str) -> Result<Expr, String> { parser::latex::parse(input) }

/// 对表达式求值，返回 None 表示表达式包含未绑定变量或非法运算。
pub fn numerical_eval(expr: &Expr) -> Option<f64> { number::float::evaluate(expr) }

/// 返回引擎版本号。
pub fn version() -> &'static str { VERSION }

pub mod prelude {
    pub use crate::calculus::derivative::{derivative, nth_derivative};
    pub use crate::calculus::integral::{definite_integral, integrate, substitute};
    pub use crate::calculus::limit::{limit, Direction};
    pub use crate::calculus::series::taylor;
    pub use crate::expr::Expr;
    pub use crate::linalg::Matrix;
    pub use crate::number::float::evaluate;
    pub use crate::simplify::collect::collect;
    pub use crate::simplify::factor::factor;
    pub use crate::simplify::expand::expand;
    pub use crate::output::mathml::to_mathml;
    pub use crate::number::number_theory::{factorize, is_prime, mod_inverse, mod_pow};
    pub use crate::output::latex::to_latex;
    pub use crate::parser::infix::parse;
    pub use crate::parser::latex::parse as parse_latex;
    pub use crate::simplify::{expand, factor, simplify};
}
