# SymEngine-X

SymEngine-X 是一个高性能符号数学计算引擎，提供 Rust 核心、CLI、Python/WASM 绑定与 Web 计算器。

当前实现 M1/M2 可运行基线：表达式树、解析、化简、求导、常见积分、极限、Taylor 级数、矩阵、数论与 LaTeX/MathML 输出。

## 快速开始

```bash
cargo test --workspace
cargo run -p symx-cli -- 'x^2 + 2*x + 1' --derive-by x --to latex
cargo run -p symx-cli -- 'x^2 + 2*x + 1' --collect x --to mathml
```

## CLI 选项

- `--derive-by VAR` 求导
- `--integrate VAR` 积分
- `--limit VAR:POINT` 极限
- `--factor` / `--expand` / `--collect VAR` 代数变换
- `--to latex|mathml` 切换输出格式

## 目录

- `crates/symx-core`：核心表达式与计算引擎
- `crates/symx-cli`：命令行计算器
- `crates/symx-python`：PyO3 绑定骨架
- `crates/symx-wasm`：WASM 绑定骨架
- `web`：React Web 计算器

Apache 2.0 © 2026 科技酱
