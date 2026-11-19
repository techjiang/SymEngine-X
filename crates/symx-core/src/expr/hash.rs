use super::node::Expr;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn structural_hash(expr: &Expr) -> u64 {
    let mut hasher = DefaultHasher::new();
    format!("{expr:?}").hash(&mut hasher);
    hasher.finish()
}
