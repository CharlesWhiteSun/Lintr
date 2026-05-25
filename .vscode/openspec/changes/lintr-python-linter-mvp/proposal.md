# Proposal: Lintr Python Linter MVP

## 為何需要此功能

Lintr 需要先建立可測試、可擴充的 Rust library 基礎，才能穩定承載 Python 靜態分析規則。此 change 先鎖定 MVP 行為契約與 TDD/SOLID 開發規範，避免後續 rule、parser 與 config 互相耦合。

## 功能範圍

In scope：

- 建立 Cargo workspace 與五個 library crate：`lintr-core`、`lintr-parser`、`lintr-config`、`lintr-rules`、`lintr`。
- 提供 `lint(source, config)` 與 `lint_file(path, config)` public API。
- 封裝 Python parser，對合法 source 產生可供 rules 使用的 context，對非法 source 回傳 typed parse error。
- 支援 TOML config：default、`lintr.toml`、`pyproject.toml [tool.lintr]`、rule code/category filtering、rule settings。
- 實作 8 條內建規則：`E001`、`E002`、`E003`、`B001`、`B002`、`B003`、`S001`、`S002`。
- 以 TDD 實作每個 crate 與 rule，並以 SOLID 維持 crate 邊界與 rule 抽象。

Out of scope：

- CLI binary、LSP、autofix、plugin system、type inference、跨檔 import graph、完整 dataflow analysis。
- 對 Python 套件環境、import resolution 或 runtime semantics 做深度推論。

## 涉及能力（Capabilities）

- `lintr-library-api`：public lint API、config filtering、diagnostic ordering、parse / IO error 行為。
- `lintr-development-process`：TDD 與 SOLID 的強制開發流程、測試層級與架構守門。

## 影響範圍與主要檔案

- `Cargo.toml`：workspace members 與共用 dependencies。
- `crates/lintr-core/`：核心型別與 rule abstraction。
- `crates/lintr-parser/`：Python parser wrapper 與 lint context。
- `crates/lintr-config/`：TOML config parsing 與 rule settings。
- `crates/lintr-rules/`：內建 rules 與 registry。
- `crates/lintr/`：public API 與 integration tests。
- `.vscode/openspec/changes/lintr-python-linter-mvp/`：本 change 的 proposal、design、tasks、specs。
- `.vscode/knowledge/` 與 `.vscode/copilot-instructions.md`：TDD/SOLID 流程規範與模組 quickref。

## 已知風險

- `rustpython-parser` 版本與 AST/location API 可能與預期不同，需要由 parser tests 先驗證。
- `E003` 若用純字串掃描容易誤判字串與註解，應優先採 tokenizer 或保守 scope。
- 若 `lintr-core` 反向依賴 parser/rules/config，會造成 crate cycle 與 SOLID 破壞；必須由 `Rule<C>` 泛型 context 避免。
