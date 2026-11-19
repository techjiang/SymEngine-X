use super::node::Expr;

#[derive(Default, Debug)]
pub struct Arena { nodes: Vec<Expr> }

impl Arena {
    pub fn new() -> Self { Self { nodes: Vec::new() } }
    pub fn alloc(&mut self, expr: Expr) -> usize {
        let id = self.nodes.len();
        self.nodes.push(expr);
        id
    }
    pub fn get(&self, id: usize) -> Option<&Expr> { self.nodes.get(id) }
    pub fn len(&self) -> usize { self.nodes.len() }
    pub fn is_empty(&self) -> bool { self.nodes.is_empty() }
}
