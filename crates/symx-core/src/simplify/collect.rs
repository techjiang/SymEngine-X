use crate::expr::Expr;

#[derive(Clone, Copy)]
enum Num { Int(i64), Rational(i64, i64), Float(f64) }

impl Num {
    fn as_f64(self) -> f64 {
        match self {
            Num::Int(n) => n as f64,
            Num::Rational(n, d) => n as f64 / d as f64,
            Num::Float(n) => n,
        }
    }
    fn add(self, other: Num) -> Num {
        match (self, other) {
            (Num::Int(a), Num::Int(b)) => Num::Int(a + b),
            (Num::Rational(n1, d1), Num::Rational(n2, d2)) => {
                let n = n1 * d2 + n2 * d1;
                let d = d1 * d2;
                let (n, d) = super::super::number::rational::reduce(n, d);
                if d == 1 { Num::Int(n) } else { Num::Rational(n, d) }
            }
            (a, b) => Num::Float(a.as_f64() + b.as_f64()),
        }
    }
    fn zero(self) -> bool { self.as_f64().abs() < 1e-12 }
    fn expr(self) -> Expr {
        match self {
            Num::Int(n) => Expr::integer(n),
            Num::Rational(n, d) => Expr::Rational(n, d),
            Num::Float(n) if n.fract() == 0.0 && n.abs() < i64::MAX as f64 => Expr::integer(n as i64),
            Num::Float(n) => Expr::Float(n),
        }
    }
}

pub fn collect(expr: &Expr, symbol: &str) -> Expr {
    let expanded = super::expand::expand(expr);
    let mut groups: Vec<(Num, Expr)> = Vec::new();
    if let Expr::Add(items) = &expanded {
        for item in items {
            let (coeff, power) = extract_coeff_power(item, symbol);
            if let Some(group) = groups.iter_mut().find(|(_, p)| p == &power) { group.0 = group.0.add(coeff); }
            else { groups.push((coeff, power)); }
        }
    } else {
        let (coeff, power) = extract_coeff_power(&expanded, symbol);
        groups.push((coeff, power));
    }
    groups.sort_by(|a, b| format!("{:?}", b.1).cmp(&format!("{:?}", a.1)));
    let mut acc = Expr::zero();
    for (coeff, power) in groups {
        if coeff.zero() { continue; }
        let coefficient = coeff.expr();
        let term = if power == Expr::zero() {
            coefficient
        } else if power == Expr::one() {
            Expr::mul(coefficient, Expr::symbol(symbol))
        } else {
            Expr::mul(coefficient, Expr::pow(Expr::symbol(symbol), power))
        };
        acc = Expr::add(acc, term);
    }
    super::rules::simplify(&acc)
}

fn extract_coeff_power(expr: &Expr, symbol: &str) -> (Num, Expr) {
    if let Expr::Mul(items) = expr {
        let mut coeff = Num::Int(1);
        let mut power = Expr::zero();
        for item in items {
            match item {
                Expr::Integer(n) => coeff = Num::Int(*n),
                Expr::Rational(n, d) if *d != 0 => coeff = Num::Rational(*n, *d),
                Expr::Float(n) => coeff = Num::Float(*n),
                Expr::Neg(inner) => { let (c, p) = extract_coeff_power(inner, symbol); coeff = Num::Float(-c.as_f64()); if p != Expr::zero() { power = p; } }
                Expr::Symbol(s) if s == symbol => power = Expr::one(),
                Expr::Pow(base, exp) if base.as_ref() == &Expr::symbol(symbol) => power = (**exp).clone(),
                _ => {}
            }
        }
        (coeff, power)
    } else if expr == &Expr::symbol(symbol) {
        (Num::Int(1), Expr::one())
    } else if let Expr::Pow(base, exp) = expr {
        if base.as_ref() == &Expr::symbol(symbol) { (Num::Int(1), (**exp).clone()) } else { (Num::Int(1), Expr::zero()) }
    } else if let Expr::Neg(inner) = expr {
        let (c, p) = extract_coeff_power(inner, symbol);
        (Num::Float(-c.as_f64()), p)
    } else {
        match expr {
            Expr::Integer(n) => (Num::Int(*n), Expr::zero()),
            Expr::Rational(n, d) if *d != 0 => (Num::Rational(*n, *d), Expr::zero()),
            Expr::Float(n) => (Num::Float(*n), Expr::zero()),
            _ => (Num::Int(1), Expr::zero()),
        }
    }
}
