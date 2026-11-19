use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Integer(i64),
    Rational(i64, i64),
    Float(f64),
    Symbol(String),
    Add(Vec<Expr>),
    Mul(Vec<Expr>),
    Pow(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
    Function(String, Vec<Expr>),
}

impl Expr {
    pub fn integer(n: i64) -> Self { Expr::Integer(n) }
    pub fn zero() -> Self { Expr::Integer(0) }
    pub fn one() -> Self { Expr::Integer(1) }
    pub fn symbol(name: impl Into<String>) -> Self { Expr::Symbol(name.into()) }

    pub fn add(a: Expr, b: Expr) -> Self {
        let mut items = Vec::new();
        push_flatten(&mut items, a, false);
        push_flatten(&mut items, b, false);
        norm_add(items)
    }

    pub fn mul(a: Expr, b: Expr) -> Self {
        let mut items = Vec::new();
        push_flatten(&mut items, a, true);
        push_flatten(&mut items, b, true);
        norm_mul(items)
    }

    pub fn div(a: Expr, b: Expr) -> Self {
        Expr::mul(a, Expr::pow(b, Expr::integer(-1)))
    }

    pub fn sub(a: Expr, b: Expr) -> Self { Expr::add(a, Expr::neg(b)) }
    pub fn neg(a: Expr) -> Self {
        match a {
            Expr::Integer(n) => Expr::Integer(-n),
            Expr::Rational(n, d) => Expr::Rational(-n, d),
            Expr::Float(n) => Expr::Float(-n),
            Expr::Neg(inner) => *inner,
            other => Expr::Neg(Box::new(other)),
        }
    }
    pub fn pow(base: Expr, exp: Expr) -> Self {
        match (&base, &exp) {
            (_, Expr::Integer(0)) => Expr::one(),
            (_, Expr::Integer(1)) => base,
            (Expr::Integer(0), Expr::Integer(n)) if *n > 0 => Expr::zero(),
            (Expr::Integer(1), _) => Expr::one(),
            (Expr::Pow(inner, inner_exp), Expr::Integer(n)) => Expr::pow((**inner).clone(), Expr::mul((**inner_exp).clone(), Expr::integer(*n))),
            _ => Expr::Pow(Box::new(base), Box::new(exp)),
        }
    }
    pub fn func(name: impl Into<String>, args: Vec<Expr>) -> Self {
        Expr::Function(name.into(), args)
    }
}

fn push_flatten(out: &mut Vec<Expr>, value: Expr, is_mul: bool) {
    match value {
        Expr::Add(items) if !is_mul => out.extend(items),
        Expr::Mul(items) if is_mul => out.extend(items),
        other => out.push(other),
    }
}

fn norm_add(mut items: Vec<Expr>) -> Expr {
    items.retain(|x| x != &Expr::zero());
    if items.is_empty() { return Expr::zero(); }
    if items.len() == 1 { return items.remove(0); }
    items.sort_by(|a, b| expr_key(a).cmp(&expr_key(b)));
    Expr::Add(items)
}

fn norm_mul(mut items: Vec<Expr>) -> Expr {
    let mut coeff = Expr::one();
    let mut rest = Vec::new();
    for item in items.drain(..) {
        match item {
            Expr::Rational(n, d) => coeff = mul_rational(coeff, Expr::Rational(n, d)),
            Expr::Integer(n) => coeff = mul_rational(coeff, Expr::Integer(n)),
            Expr::Float(n) => coeff = mul_rational(coeff, Expr::Float(n)),
            Expr::Neg(inner) => {
                coeff = mul_rational(coeff, Expr::Integer(-1));
                rest.push(*inner);
            }
            other => rest.push(other),
        }
    }
    if matches!(coeff, Expr::Integer(0)) { return Expr::zero(); }
    if coeff != Expr::one() { rest.insert(0, coeff); }
    if rest.is_empty() { return Expr::one(); }
    if rest.len() == 1 { return rest.remove(0); }
    rest.sort_by(|a, b| expr_key(a).cmp(&expr_key(b)));
    Expr::Mul(rest)
}

fn mul_rational(a: Expr, b: Expr) -> Expr {
    match (a, b) {
        (Expr::Integer(a), Expr::Integer(b)) => Expr::Integer(a * b),
        (Expr::Integer(a), Expr::Rational(b, d)) | (Expr::Rational(b, d), Expr::Integer(a)) => reduce_rational(a * b, d),
        (Expr::Rational(a, b), Expr::Rational(c, d)) => reduce_rational(a * c, b * d),
        (Expr::Float(a), Expr::Float(b)) => Expr::Float(a * b),
        (Expr::Float(a), Expr::Integer(b)) | (Expr::Integer(b), Expr::Float(a)) => Expr::Float(a * b as f64),
        (Expr::Rational(n, d), Expr::Float(x)) | (Expr::Float(x), Expr::Rational(n, d)) => Expr::Float(n as f64 / d as f64 * x),
        _ => Expr::Integer(1),
    }
}

fn reduce_rational(n: i64, d: i64) -> Expr {
    if d == 0 { return Expr::Rational(n, d); }
    let g = gcd(n.abs(), d.abs()).max(1);
    let mut n = n / g;
    let mut d = d / g;
    if d < 0 { n = -n; d = -d; }
    if d == 1 { Expr::Integer(n) } else { Expr::Rational(n, d) }
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 { let t = a % b; a = b; b = t; }
    a
}

fn expr_key(e: &Expr) -> String { format!("{e:?}") }

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Integer(n) => write!(f, "{n}"),
            Expr::Rational(n, d) => write!(f, "{n}/{d}"),
            Expr::Float(n) => write!(f, "{n}"),
            Expr::Symbol(s) => write!(f, "{s}"),
            Expr::Neg(e) => write!(f, "-({e})"),
            Expr::Add(items) => {
                for (i, item) in items.iter().enumerate() {
                    if i > 0 { write!(f, " + ")?; }
                    write!(f, "{item}")?;
                }
                Ok(())
            }
            Expr::Mul(items) => {
                for (i, item) in items.iter().enumerate() {
                    if i > 0 { write!(f, " * ")?; }
                    write!(f, "{item}")?;
                }
                Ok(())
            }
            Expr::Pow(base, exp) => write!(f, "({base})^{exp}"),
            Expr::Function(name, args) => {
                write!(f, "{name}(")?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{arg}")?;
                }
                write!(f, ")")
            }
        }
    }
}
