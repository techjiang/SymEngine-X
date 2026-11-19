use symx_core::prelude::*;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() || args[0] == "--help" {
        println!("SymEngine-X CLI");
        println!("Usage: symx-cli <expression> [--derive-by VAR] [--integrate VAR] [--limit VAR:POINT] [--factor] [--expand] [--collect VAR] [--to latex|mathml]");
        return;
    }
    let input = &args[0];
    let expr = match parse(input) { Ok(expr) => expr, Err(err) => { eprintln!("parse error: {err}"); std::process::exit(1); } };
    let mut result = expr;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--derive-by" => {
                let var = args.get(i + 1).cloned().unwrap_or_else(|| "x".into());
                result = simplify(&derivative(&result, &var));
                i += 2;
            }
            "--integrate" => {
                let var = args.get(i + 1).cloned().unwrap_or_else(|| "x".into());
                result = integrate(&result, &var);
                i += 2;
            }
            "--limit" => {
                let arg = args.get(i + 1).cloned().unwrap_or_default();
                let parts: Vec<&str> = arg.splitn(2, ':').collect();
                if parts.len() == 2 {
                    let point = parse(parts[1]).unwrap_or(Expr::zero());
                    result = limit(&result, parts[0], &point, Direction::Both);
                }
                i += 2;
            }
            "--factor" => { result = symx_core::simplify::factor::factor(&result); i += 1; }
            "--expand" => { result = symx_core::simplify::expand::expand(&result); i += 1; }
            "--collect" => {
                let var = args.get(i + 1).cloned().unwrap_or_else(|| "x".into());
                result = symx_core::simplify::collect::collect(&result, &var);
                i += 2;
            }
            "--to" => {
                match args.get(i + 1).map(String::as_str) {
                    Some("latex") => { println!("{}", to_latex(&result)); return; }
                    Some("mathml") => { println!("{}", symx_core::output::mathml::to_mathml(&result)); return; }
                    _ => {}
                }
                i += 2;
            }
            _ => i += 1,
        }
    }
    println!("{}", simplify(&result));
}
