# Lintr 模組 Quickref

## 目標

- 用 Rust 打造 Python 靜態程式碼分析器 / Linter。
- MVP 交付 Library crate，不包含 CLI、LSP、autofix、plugin system、type inference、跨檔 import graph。
- OpenSpec change：`.vscode/openspec/changes/lintr-python-linter-mvp/`。

## 執行環境

- 第一階段以 Ubuntu 24.04 WSL 2 + VS Code Remote - WSL 作為主要命令執行環境；Windows 端 workspace 僅作短期 rollback。
- Repo 長期放在 `/home/charles/www/Lintr`；不從 WSL 長期操作 `/mnt/d/www/Lintr`。
- RTK 僅壓縮高噪音 shell 輸出；`kb.mjs`、`opsx` / `openspec`、安裝/下載、env/log 類命令初期保留 canonical/raw。

## Crate 邊界

| Crate | 責任 |
|------|------|
| `lintr-core` | 共用型別、diagnostic、severity/category、range、`Rule<C>`、錯誤型別 |
| `lintr-parser` | Python parser wrapper、`ParsedPythonFile`、`PythonLintContext`、source/range helper |
| `lintr-config` | TOML config、default settings、rule code/category filtering |
| `lintr-rules` | style / bug / security 內建規則與 registry |
| `lintr` | public API、re-export、engine orchestration、integration tests |

## SOLID 守門

- `lintr-core` 不可依賴 parser/config/rules/root crate。
- 新增 rule 時，優先只新增 rule module 與 registry entry，不改 engine 主流程。
- Rule 透過 `Rule<PythonLintContext>` 或等價抽象執行，不讓 engine 依賴具體 rule 型別。
- Config 描述 rule 啟停與 settings，不知道具體 rule 實作。

## TDD 流程

1. Red：先寫失敗測試，鎖定行為。
2. Green：寫最小實作讓測試通過。
3. Refactor：測試通過後整理抽象與邊界。
4. 完成前跑 fmt、clippy、test、build；無法執行時必須回報原因。

## Public API Contract

- `lint(source, config)`：分析 Python source；parse failure 回 typed parse error，不執行 rules。
- `lint_file(path, config)`：讀檔後沿用 `lint`；IO failure 回 typed IO error。
- Diagnostics 穩定排序：start offset、end offset、rule code。
- Config filtering 在 rule 執行前套用。

## MVP Rules

- Style：`E001 LineTooLong`、`E002 TrailingWhitespace`、`E003 MissingWhitespaceAroundOperator`。
- Bug：`B001 MutableDefaultArgument`、`B002 CompareToNoneWithEq`、`B003 UnreachableCode`。
- Security：`S001 UseOfEval`、`S002 HardcodedPassword`。

## 驗證指令

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo build --workspace`
- `./opsx list`（WSL / Linux shell）
- `node .vscode/knowledge/scripts/kb.mjs rebuild`
- `node .vscode/knowledge/scripts/kb.mjs finish-check`
