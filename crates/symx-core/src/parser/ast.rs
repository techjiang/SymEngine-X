use crate::expr::Expr;

#[derive(Clone, Debug)]
pub struct Ast { pub root: Expr }

impl Ast {
    pub fn new(root: Expr) -> Self { Self { root } }
}
