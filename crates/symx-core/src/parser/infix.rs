//! 中缀表达式解析器，支持四则运算、幂、括号、函数调用、科学计数法与 `**` 幂写法。
use crate::expr::Expr;

#[derive(Clone, Debug, PartialEq)]
enum Token { Num(f64), Ident(String), Op(char), LParen, RParen, Comma }

pub fn parse(input: &str) -> Result<Expr, String> {
    let tokens = lex(input)?;
    let mut parser = Parser { tokens, pos: 0 };
    if tokens.is_empty() { return Err("empty expression".into()); }
    let expr = parser.parse_expr()?;
    if parser.pos != parser.tokens.len() { return Err(format!("unexpected token at {}", parser.pos)); }
    Ok(expr)
}

fn lex(input: &str) -> Result<Vec<Token>, String> {
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    let mut out = Vec::new();
    while i < chars.len() {
        let c = chars[i];
        if c.is_whitespace() { i += 1; continue; }
        match c {
            '(' => { out.push(Token::LParen); i += 1; }
            ')' => { out.push(Token::RParen); i += 1; }
            ',' => { out.push(Token::Comma); i += 1; }
            '+' | '-' | '*' | '/' | '^' => {
                if c == '*' && i + 1 < chars.len() && chars[i + 1] == '*' {
                    out.push(Token::Op('^'));
                    i += 2;
                } else {
                    out.push(Token::Op(c));
                    i += 1;
                }
            }
            '0'..='9' | '.' | 'a'..='z' | 'A'..='Z' | '_' => {
                let start = i;
                while i < chars.len() && (chars[i].is_ascii_alphanumeric() || chars[i] == '.' || chars[i] == '_') { i += 1; }
                let text: String = chars[start..i].iter().collect();
                let first = text.chars().next().unwrap_or(' ');
                if first.is_ascii_digit() || first == '.' {
                    let value = text.parse::<f64>().map_err(|_| "invalid number".to_string())?;
                    if !value.is_finite() { return Err("invalid number".into()); }
                    out.push(Token::Num(value));
                } else {
                    out.push(Token::Ident(text));
                }
            }
            _ => return Err(format!("unknown character: {c}")),
        }
    }
    Ok(out)
}

struct Parser { tokens: Vec<Token>, pos: usize }

impl Parser {
    fn peek(&self) -> Option<&Token> { self.tokens.get(self.pos) }
    fn eat(&mut self) -> Option<Token> { let t = self.tokens.get(self.pos).cloned(); if t.is_some() { self.pos += 1; } t }
    fn parse_expr(&mut self) -> Result<Expr, String> { self.parse_add() }

    fn parse_add(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_mul()?;
        while matches!(self.peek(), Some(Token::Op('+')) | Some(Token::Op('-'))) {
            let op = match self.eat() { Some(Token::Op(c)) => c, _ => unreachable!() };
            let right = self.parse_mul()?;
            left = if op == '+' { Expr::add(left, right) } else { Expr::sub(left, right) };
        }
        Ok(left)
    }

    fn parse_mul(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_power()?;
        while matches!(self.peek(), Some(Token::Op('*')) | Some(Token::Op('/'))) {
            let op = match self.eat() { Some(Token::Op(c)) => c, _ => unreachable!() };
            let right = self.parse_power()?;
            left = if op == '*' { Expr::mul(left, right) } else { Expr::div(left, right) };
        }
        Ok(left)
    }

    fn parse_power(&mut self) -> Result<Expr, String> {
        let base = self.parse_unary()?;
        if matches!(self.peek(), Some(Token::Op('^'))) {
            self.eat();
            let exp = self.parse_power()?;
            Ok(Expr::pow(base, exp))
        } else { Ok(base) }
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        if matches!(self.peek(), Some(Token::Op('-'))) {
            self.eat();
            return Ok(Expr::neg(self.parse_unary()?));
        }
        if matches!(self.peek(), Some(Token::Op('+'))) {
            self.eat();
            return self.parse_unary();
        }
        self.parse_atom()
    }

    fn parse_atom(&mut self) -> Result<Expr, String> {
        match self.eat() {
            Some(Token::Num(n)) => Ok(number(n)),
            Some(Token::Ident(name)) => {
                if matches!(self.peek(), Some(Token::LParen)) {
                    self.eat();
                    let mut args = Vec::new();
                    if !matches!(self.peek(), Some(Token::RParen)) {
                        loop {
                            args.push(self.parse_expr()?);
                            if matches!(self.peek(), Some(Token::Comma)) { self.eat(); continue; }
                            break;
                        }
                    }
                    if !matches!(self.peek(), Some(Token::RParen)) { return Err("missing )".into()); }
                    self.eat();
                    Ok(Expr::func(name, args))
                } else { Ok(Expr::symbol(name)) }
            }
            Some(Token::LParen) => {
                let expr = self.parse_expr()?;
                if !matches!(self.peek(), Some(Token::RParen)) { return Err("missing )".into()); }
                self.eat();
                Ok(expr)
            }
            other => Err(format!("unexpected token: {other:?}")),
        }
    }
}

fn number(n: f64) -> Expr {
    if n.fract() == 0.0 && n.abs() < i64::MAX as f64 { Expr::Integer(n as i64) } else { Expr::Float(n) }
}
