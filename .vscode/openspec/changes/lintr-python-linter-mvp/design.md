# Design: Lintr Python Linter MVP

## 架構決策

- 決策：採 Cargo workspace 與五個 library crate，因為 parser、config、rules 與 public API 的變更節奏不同，需要清楚邊界。
- 決策：`lintr-core` 只放共用型別與 `Rule<C>` 抽象，不引用 parser 型別，因為 core 應維持最底層依賴。
- 決策：Python-specific context 放在 `lintr-parser`，規則實作 `Rule<PythonLintContext>`，因為 rule 需要 AST/source helper，但 engine 不應依賴具體 rule。
- 決策：public API 對 parse failure 使用 typed error，不把語法錯誤混入 style/bug/security diagnostic，因為 parse error 代表規則無法安全執行。
- 決策：diagnostic ordering 固定由 engine 統一排序，因為 registry 順序與 rule traversal 細節不應影響 public API 穩定性。
- 決策：TDD 與 SOLID 寫入知識庫與專案規範，因為這是後續每個 rule 與 crate 變更的共同開發契約。

## 資料流

1. 呼叫者建立或載入 `Config`。
2. `lint(source, config)` 呼叫 parser 建立 `PythonLintContext`。
3. parse 成功後，engine 從 registry 取得 rules，依 config 過濾 category/code 與 settings。
4. 每條 rule 以 `Rule<PythonLintContext>` 執行，回傳 diagnostics。
5. engine 合併 diagnostics 並依 range、rule code 做穩定排序。
6. parse 失敗時，API 回傳 typed parse error，規則不執行。
7. `lint_file(path, config)` 先讀檔；IO 失敗回傳 typed IO error，讀檔成功後沿用 `lint(source, config)`。

## 涉及檔案

- `Cargo.toml`：新增 workspace 設定與共用 dependencies。
- `crates/lintr-core/Cargo.toml`、`crates/lintr-core/src/lib.rs`：核心型別、`Rule<C>`、錯誤型別與單元測試。
- `crates/lintr-parser/Cargo.toml`、`crates/lintr-parser/src/lib.rs`：`rustpython-parser` wrapper、`ParsedPythonFile`、`PythonLintContext` 與 parser tests。
- `crates/lintr-config/Cargo.toml`、`crates/lintr-config/src/lib.rs`：TOML config、default settings 與 config tests。
- `crates/lintr-rules/Cargo.toml`、`crates/lintr-rules/src/`：style/bug/security rules、registry 與 rule tests。
- `crates/lintr/Cargo.toml`、`crates/lintr/src/lib.rs`、`crates/lintr/tests/`：public API、re-export 與 integration tests。
- `.vscode/copilot-instructions.md`、`.vscode/knowledge/INDEX.md`、`.vscode/knowledge/modules/lintr/quickref.md`、`.vscode/knowledge/changelog/2026-05.md`：流程規範與知識庫。

## TDD 與 SOLID 落地

- 每個 public behavior 先有 failing test，再做最小實作。
- 每條 rule 至少包含正向、負向、邊界測試。
- core tests 需覆蓋 diagnostic/range/rule trait object 可替換性。
- parser tests 需覆蓋合法 source、非法 source、source 保留與 range helper。
- config tests 需覆蓋 default、`lintr.toml`、`pyproject.toml [tool.lintr]`、rule filtering。
- integration tests 需覆蓋 `lint()`、`lint_file()`、diagnostic ordering 與 config filtering。
- SOLID review gate：新增 rule 不改 engine 主流程；修改 core trait 前確認不造成 crate cycle。
