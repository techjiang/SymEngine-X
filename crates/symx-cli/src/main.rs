use symx_core::prelude::*;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() || args[0] == "--help" {
        println!("SymEngine-X CLI");
        println!("Usage: symx-cli <expression> [options]");
        println!("Options:");
        println!("  --derive-by VAR       对 VAR 求导");
        println!("  --integrate VAR       对 VAR 积分");
        println!("  --limit VAR:POINT:DIR 求极限，DIR 支持 both/left/right（默认 both）");
        println!("                        POINT 支持 0/+inf/-inf 或任意数值表达式");
        println!("  --taylor VAR:POINT:N  Taylor 展开");
        println!("  --solve VAR           求解一元多项式方程");
        println!("  --eval                数值求值");
        println!("  --factor / --expand / --collect VAR");
        println!("  --to latex|mathml     切换输出格式");
        println!("  --version             显示版本号");
        return;
    }
    if args[0] == "--version" {
        println!("SymEngine-X {}", symx_core::version());
        return;
    }
    let input = &args[0];
    let expr = match parse(input) {
        Ok(expr) => expr,
        Err(err) => {
            eprintln!("parse error: {err}");
            eprintln!("hint: 使用 --help 查看支持的表达式语法与选项");
            std::process::exit(1);
        }
    };
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
                let parts: Vec<&str> = arg.split(':').collect();
                if parts.len() >= 2 {
                    let (point_arg, direction) = if parts.len() >= 3 {
                        (parts[1], parts[2..].join(":"))
                    } else {
                        (parts[1], "both".to_string())
                    };
                    let point = parse(point_arg).unwrap_or(Expr::zero());
                    let direction = direction.parse().unwrap_or(Direction::Both);
                    result = limit(&result, parts[0], &point, direction);
                }
                i += 2;
            }
            "--taylor" => {
                let arg = args.get(i + 1).cloned().unwrap_or_else(|| "x:0:3".into());
                let parts: Vec<&str> = arg.split(':').collect();
                if parts.len() >= 2 {
                    let point = parse(parts[1]).unwrap_or(Expr::zero());
                    let order = parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(3);
                    result = taylor(&result, parts[0], &point, order);
                }
                i += 2;
            }
            "--solve" => {
                let var = args.get(i + 1).cloned().unwrap_or_else(|| "x".into());
                let roots = symx_core::algebra::root::solve(&result, &var);
                println!("{roots:?}");
                return;
            }
            "--eval" => {
                match symx_core::numerical_eval(&result) {
                    Some(value) => println!("{value}"),
                    None => { eprintln!("expression is not numerically evaluable"); std::process::exit(1); }
                }
                return;
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
