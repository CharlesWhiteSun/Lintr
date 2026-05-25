# Research: Lintr Python Linter MVP

## 知識庫查閱結果

- 任務類型：新功能 / 行為契約建立，因此先查 `.vscode/openspec/specs/INDEX.md`。目前尚無已完成規格。
- start-check：`module=lintr`、`file=.vscode/copilot-instructions.md`、`query="TDD SOLID Lintr Python linter MVP"`。
- required reads：`knowledge/agent/INDEX.md`、`knowledge/INDEX.md`、`knowledge/modules/lintr/quickref.md`、`knowledge/traps/topics/INDEX.md`。
- `knowledge/modules/lintr/quickref.md` 尚不存在，本 change 會新增。
- 相關 topics：`command-preflight`、`powershell-encoding`、`tool-search-visibility`。

## 相關陷阱清單

- 無直接 trap 命中。
- `command-preflight`：0 traps；本任務已在執行 terminal 與批次 edit 前使用 repair-preflight。
- `powershell-encoding`：0 traps；知識庫檔案不可用 PowerShell `Set-Content` 類命令改寫。
- `tool-search-visibility`：0 traps；`.vscode` 與 knowledge 路徑以直接讀檔與列目錄確認。

## 假設確認

- 已確認 Lintr MVP 使用 Rust 實作，分析目標語言為 Python source。
- 已確認第一版交付形式為 Library crate，不先做 CLI。
- MVP 包含 Cargo workspace、core/parser/config/rules/root library 五個 crate、TOML config、8 條內建規則、public API 與完整測試。
- MVP 不包含 CLI、LSP、autofix、plugin system、type inference、跨檔 import graph 或完整 dataflow analysis。
- Parser 先以 `rustpython-parser` 作為候選；最終版本與 AST/location API 需由 parser crate 測試驗證後鎖定。

## 待探索範圍

- `rustpython-parser` 的實際 parse API、AST node location 與 tokenizer 能力。
- `E003 MissingWhitespaceAroundOperator` 是否能以 tokenizer 穩定實作，避免誤判字串與註解。
- Parse error 對 public API 的具體 Rust 型別命名會在 Phase 2/3 TDD 時細化，但行為契約須先固定為 typed error。
