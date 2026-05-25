# Tasks: Lintr Python Linter MVP

## 1. 規格與知識庫前置

- [x] 1.1 建立 OpenSpec research/proposal/design/tasks/specs artifacts
- [x] 1.2 更新 `.vscode/copilot-instructions.md` 的 Rust 技術棧、架構、TDD 與 SOLID 規範
- [x] 1.3 更新 `.vscode/knowledge/INDEX.md` 的 Quick Context 與 lintr 模組導航
- [x] 1.4 新增 `.vscode/knowledge/modules/lintr/quickref.md`
- [x] 1.5 更新 `.vscode/knowledge/changelog/2026-05.md`

## 2. 程式碼修改

- [ ] 2.1 新增根 `Cargo.toml` workspace 與共用 dependencies
- [ ] 2.2 新增 `lintr-core` crate，先寫 core 型別與 `Rule<C>` 測試，再實作最小核心抽象
- [ ] 2.3 新增 `lintr-parser` crate，先寫 parser wrapper 測試，再封裝 Python parse context
- [ ] 2.4 新增 `lintr-config` crate，先寫 TOML/default/filtering 測試，再實作 config API
- [ ] 2.5 新增 `lintr-rules` crate，依 TDD 實作 style、bug、security rules 與 registry
- [ ] 2.6 新增 `lintr` root library crate，先寫 public API integration tests，再實作 `lint()` 與 `lint_file()`

## 3. 測試驗證

- [ ] 3.1 執行 `cargo fmt --all -- --check`
- [ ] 3.2 執行 `cargo clippy --workspace --all-targets -- -D warnings`
- [ ] 3.3 執行 `cargo test --workspace`
- [ ] 3.4 執行 `cargo build --workspace`
- [ ] 3.5 驗證 `lint()` 對 E001/B002/S001 fixture 產生穩定診斷
- [ ] 3.6 驗證 config 停用 rule 後不再回報該 diagnostic

## 4. 知識庫更新

- [ ] 4.1 執行 kb.mjs new-trap（如有新陷阱）
- [ ] 4.2 更新 modules/lintr/quickref.md（如 crate 邊界或 API contract 有變更）
- [ ] 4.3 更新 changelog/2026-05.md
- [ ] 4.4 執行 `node .vscode/knowledge/scripts/kb.mjs rebuild`
- [ ] 4.5 執行 `node .vscode/knowledge/scripts/kb.mjs finish-check`，確認 0 errors
