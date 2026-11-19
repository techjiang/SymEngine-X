# SymEngine-X

SymEngine-X 是一个高性能符号数学计算引擎，提供 Rust 核心、CLI、Python/WASM 绑定与 Web 计算器。

当前实现 M1/M2 可运行基线：表达式树、解析、化简、求导、常见积分、极限、Taylor 级数、矩阵、数论与 LaTeX/MathML 输出。

## 快速开始

```bash
cargo test --workspace
cargo run -p symx-cli -- 'x^2 + 2*x + 1' --derive-by x --to latex
cargo run -p symx-cli -- 'x^2 + 2*x + 1' --collect x --to mathml
cargo run -p symx-cli -- 'x^2 - 2*x + 1' --factor
cargo run -p symx-cli -- 'x^2 - 5*x + 6' --solve x
cargo run -p symx-cli -- 'sin(x)^2' --integrate x
cargo run -p symx-cli -- '1 + 2/x' --limit x:+inf
cargo run -p symx-cli -- 'exp(x)' --taylor x:0:5
cargo run -p symx-cli -- '2^10 + 3' --eval
```

## CLI 选项

- `--derive-by VAR` 求导
- `--integrate VAR` 积分
- `--limit VAR:POINT` 极限，POINT 支持 `0` / `+inf` / `-inf`
- `--taylor VAR:POINT:N` 在 POINT 处展开 N 阶 Taylor
- `--solve VAR` 求解一元多项式方程（一次/二次/三次）
- `--eval` 数值求值
- `--factor` / `--expand` / `--collect VAR` 代数变换
- `--to latex|mathml` 切换输出格式

## 核心 API

```rust
use symx_core::prelude::*;

let expr = parse("x^2 + 2*x + 1").unwrap();
println!("{}", simplify(&expr));                         // 化简
println!("{}", derivative(&expr, "x"));                  // 求导
println!("{}", integrate(&expr, "x"));                   // 积分
println!("{}", taylor(&expr, "x", &Expr::zero(), 5));    // Taylor
println!("{:?}", symx_core::algebra::root::solve(&expr, "x"));
println!("{}", to_latex(&expr));                         // LaTeX
```

## 目录

- `crates/symx-core`：核心表达式与计算引擎
- `crates/symx-cli`：命令行计算器
- `crates/symx-python`：PyO3 绑定骨架
- `crates/symx-wasm`：WASM 绑定骨架
- `web`：React Web 计算器

## 路线图

- 完成 Risch 积分、SVD 分解、PDE 求解
- 完善 PyO3 与 wasm-bindgen 正式包
- Web 计算器接入 WASM 实时计算

Apache 2.0 © 2026 科技酱
