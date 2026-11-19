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
cargo run -p symx-cli -- '1/x' --limit x:0:right
cargo run -p symx-cli -- 'exp(x)' --taylor x:0:5
cargo run -p symx-cli -- '2^10 + 3' --eval
```

## CLI 选项

- `--derive-by VAR` 求导
- `--integrate VAR` 积分
- `--limit VAR:POINT[:DIR]` 极限，POINT 支持 `0` / `+inf` / `-inf`，DIR 支持 `both` / `left` / `right`
- `--taylor VAR:POINT:N` 在 POINT 处展开 N 阶 Taylor
- `--solve VAR` 求解一元多项式方程（一次/二次/三次）
- `--eval` 数值求值
- `--factor` / `--expand` / `--collect VAR` 代数变换
- `--to latex|mathml` 切换输出格式
- `--version` 显示版本号
- 表达式中的 `**` 等价于 `^`，例如 `x**2` 与 `x^2` 相同

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
- CLI 增加批量表达式与脚本输入

Apache 2.0 © 2026 科技酱


## 本轮增量：7 × 5 轮更新

- 输出：LaTeX/MathML 对符号形式的无穷统一渲染
- 极限：有理式无穷极限保留最高次项系数，支持符号/数值无穷输入
- 表达式：任意负整数指数归一化为倒数
- 数值：支持 `pow/abs/min/max` 函数求值
- CLI：新增 `--latex-expr`、`--mathml-expr` 直接渲染表达式
- Web：新增极限运算、复制结果与重置示例

```bash
symx-cli --latex-expr 'x^2 + 1/2'
symx-cli --mathml-expr 'sqrt(x)'
symx-cli '2*x/x' --limit x:+inf
```

## 作者与社区

- 作者：科技酱（同一个世界，同一个互联网！）
- 主页：https://www.asoe.cn
- 技术文档：https://docs.asoe.cn
- GitHub：https://github.com/techjiang/
- 哔哩哔哩：https://space.bilibili.com/1768832152
- 邮箱：official@tech.asoe.cn
- 玲珑：https://forums.asoe.cn/
- QQ群：291974598 / 474819022 / 631167288

## 发布与可用性说明

- 版本：v1.0.0（最终维护版本）
- 依赖：Rust stable 工具链（核心/CLI）、Node.js 18+（Web 计算器）
- 快速体验：
  - Web：`cd web && npm install && npm run build && npm run preview`
  - CLI：`cargo run -p symx-cli -- 'x^2 - 2*x + 1' --factor`
  - 测试：`cargo test --workspace`（Rust）、`cd web && npm run build`（前端）
- 已知边界：v1.0.0 聚焦 M1/M2 可运行能力（表达式、化简、微积分基础、矩阵、数论、LaTeX/MathML、CLI/Web）；Risch 完整积分、SVD、PDE、PyO3/WASM 正式下载包仍在路线图。
- 仓库归档：源码与原版 Release 产物同步至 https://github.com/techjiang/SymEngine-X.git
