# SymEngine-X

SymEngine-X 是一个高性能符号数学计算引擎，提供 Rust 核心、CLI、Python/WASM 绑定与 Web 计算器骨架。

当前版本实现的是 M1/M2 的可运行基线：表达式树、解析、化简、求导、常见积分、极限、Taylor 级数、矩阵、数论与 LaTeX 输出。

## 快速开始

```bash
cargo test --workspace
cargo run -p symx-cli -- 'x^2 + 2*x + 1' --derive-by x --to latex
```

## 目录

- `crates/symx-core`：核心表达式与计算引擎
- `crates/symx-cli`：命令行计算器
- `crates/symx-python`：PyO3 绑定骨架
- `crates/symx-wasm`：WASM 绑定骨架
- `web`：React Web 计算器骨架
