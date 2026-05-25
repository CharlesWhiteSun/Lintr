# GitHub Copilot 專案作業規範

> 此檔案為本專案的**單一規範真實來源**（Single Source of Truth），所有 prompt 檔案引用本檔而非重複定義。

---

## 一、技術棧

| 項目 | 版本 / 說明 |
|------|------------|
| 語言 | Rust（Cargo workspace；以 library crates 為主） |
| 分析目標 | Python source code（MVP） |
| 主要架構 | Rust static analyzer / linter library；`crates/lintr` 聚合 public API |
| Parser | `rustpython-parser`（由 `lintr-parser` 封裝；版本以 parser 測試驗證後鎖定） |
| 設定格式 | TOML：`lintr.toml` 與 `pyproject.toml [tool.lintr]` |
| 資料庫 | 無 |
| 測試框架 | Rust `#[test]`、crate integration tests、`cargo test --workspace`、`cargo clippy` |

---

## 一之一、執行環境與 RTK 政策

- 第一階段以 **WSL 2 + VS Code Remote - WSL** 作為主要命令執行環境；AI Chat terminal、Rust/Cargo、Node、OpenSpec 與 RTK 應在同一個 WSL shell 中運作。
- Phase 1 基準環境為 `Ubuntu-24.04`（WSL 2）、預設使用者 `charles`、Linux workspace `/home/charles/www/Lintr`；Windows 端 `D:\www\Lintr` 僅作短期 rollback 來源，不作長期雙寫。
- Phase 2 工具鏈基準：`rustc 1.95.0` / `cargo 1.95.0`（rustup minimal，`~/.cargo/bin`）、`node v24.16.0` / `npm 11.13.0` 與 OpenSpec `1.3.1`（`~/.local/bin`）、`ripgrep 15.1.0`（`~/.local/bin`）、`git 2.43.0`、`sqlite3 3.45.1`、`gcc 13.3.0`、GNU Make `4.3`、`pkg-config 1.8.1`。目前尚無 root `Cargo.toml`，Cargo workspace 驗證延後到 Lintr MVP scaffold 後執行。
- RTK 是「命令輸出壓縮層」，不是知識庫來源；知識查閱仍以 `.vscode/knowledge`、OpenSpec 與本檔為準。
- 初期排除 RTK 自動 rewrite 的命令：`kb.mjs`、`opsx` / `openspec`、安裝/下載命令、`env` / log / 可能含 secrets 的輸出。
- 若 RTK 壓縮摘要不足以判斷錯誤，必須改用 canonical command 或 RTK verbose/raw fallback，不得只憑摘要重試或下結論。

---

## 二、專案架構與命名慣例

本專案先以「Rust 實作、分析 Python source、交付 Library crate」作為 Lintr MVP。第一版不包含 CLI、LSP、autofix、plugin system、type inference 或跨檔 import graph。

### Cargo workspace 分層

| Crate | 責任 | 依賴方向 |
|------|------|----------|
| `lintr-core` | 共用型別、`Diagnostic`、`Severity`、`Category`、`TextRange`、`Rule<C>` trait、錯誤型別 | 不依賴其他本地 crate |
| `lintr-parser` | Python source parsing、`ParsedPythonFile`、`PythonLintContext`、source/range helper | 依賴 `lintr-core` |
| `lintr-config` | TOML config 載入、default config、rule settings 查詢 | 依賴 `lintr-core` |
| `lintr-rules` | 內建 style / bug / security rules 與 registry | 依賴 `lintr-core`、`lintr-parser` |
| `lintr` | 對外 public API、re-export、lint orchestration | 依賴 core/parser/config/rules |

### SOLID 開發規範

- Single Responsibility：每個 crate、module、rule 僅負責一個清楚原因；每條規則獨立檔案或子模組。
- Open/Closed：新增規則應透過 registry 或 factory 擴充，不改 engine 核心流程。
- Liskov Substitution：所有規則以 `Rule<PythonLintContext>` 代換使用，不依賴具體規則型別。
- Interface Segregation：`Rule<C>` trait 僅保留 code、category、settings、check 等必要行為；避免胖介面。
- Dependency Inversion：高層 orchestration 依賴 `Rule<C>` 抽象與 config lookup；`lintr-core` 不可反向依賴 parser/rules/config。

### 命名慣例

- Crate 使用 `lintr-*` kebab-case；Rust module、function 使用 snake_case；type / trait 使用 PascalCase。
- Rule code 使用固定前綴：`E###` style、`B###` bug、`S###` security。
- Test 名稱描述行為，例如 `reports_line_too_long_when_limit_is_exceeded`。

---

## 三、知識庫管理協議（單一真實來源）

### 知識庫結構 v3

`.vscode/knowledge/`（建議加入 `.gitignore`）：

    .vscode/knowledge/
    ├── INDEX.md                          ← <80 行純導航（第 1 層，AI 啟動必讀）
    ├── changelog/
    │   └── YYYY-MM.md                    ← 變更歷程（按月封存，唯一來源）
    ├── agent/                            ← Agent 操作守門與 runtime repair 閉環
    │   ├── INDEX.md                      ← preflight / failure record / retry rule 入口
    │   └── generated/                    ← repair guard facets（由 rebuild 產生）
    ├── modules/                          ← 第 2 層：模組強制讀
    │   └── {module}/quickref.md          ← <150 行；其餘細節分檔
    ├── traps/
    │   ├── topics-taxonomy.yml           ← 主題分類學白名單（人工維護）
    │   ├── topics/
    │   │   ├── INDEX.md                  ← 第 3 層：主題目錄（自動產生）
    │   │   └── {slug}.md                 ← 第 4 層：主題集群（AUTO 區自動，防呆原則手動）
    │   ├── trap-NNN.md                   ← 第 5 層：陷阱細節，YAML frontmatter
    │   ├── index.jsonl                   ← 機器可讀大表（自動）
    │   ├── by-module.json                ← facet：模組 → trap id 列表
    │   ├── by-tag.json                   ← facet：tag → trap id 列表
    │   ├── by-topic.json                 ← facet：主題 → trap id 列表
    │   ├── by-file.json                  ← facet：原始碼檔 → trap id 列表（修 bug 必查）
    │   ├── by-symptom.json               ← facet：症狀短語 → trap id 列表
    │   └── fts.db                        ← SQLite FTS5 全文檢索（自動，gitignore）
    └── scripts/
        └── kb.mjs                        ← Node CLI（rebuild/new-trap/taxonomy/facets/topics/audit/search/health/...）

> 詳情見 `.vscode/knowledge/INDEX.md`。

### 任務開始前（OpenSpec + 知識庫雙軌啟動）

#### Step 0 — OpenSpec 意圖確認（新功能 / 規格變更必做）

若任務屬於**新功能**、**規格變更**或**行為契約調整**：

1. 以 `/opsx:explore` 進入探索模式，與使用者釐清需求邊界與設計假設
2. 以 `/opsx:propose` 建立 OpenSpec change，產出 `.vscode/openspec/changes/{name}/` artifacts
3. 確認 `.vscode/openspec/specs/{module}/` 中是否已有對應規格（見 `.vscode/openspec/specs/INDEX.md`）；若有，讀取後才動手實作

若任務屬於**bug 修復**或**陷阱修補**，跳過 Step 0，直接從 Step 1 開始。

#### Step 1 — 知識庫預讀（4 層階梯）

1. 讀 [agent/INDEX.md](knowledge/agent/INDEX.md)，確認 preflight / failure record / retry rule
2. 執行 `node .vscode/knowledge/scripts/kb.mjs start-check --module=<Module> --file=path.ext --query="keyword"`（若參數不明，不得執行 placeholder）
3. 讀 [INDEX.md](knowledge/INDEX.md)（< 80 行）
4. 依任務讀 `modules/{m}/quickref.md`（< 150 行）
5. 讀 [traps/topics/INDEX.md](knowledge/traps/topics/INDEX.md)（主題目錄，掌握「這類問題以前發生過幾次」）
6. 命中相關主題 → 讀 `traps/topics/{slug}.md`（含相關 trap 表 + 防呆原則）；若防呆原則含 OpenSpec 連結，讀取該 spec.md
7. 必要時讀 `traps/trap-NNN.md`（細節）

替代/補充查詢：

- 模糊查詢 / 全文檢索：`node .vscode/knowledge/scripts/kb.mjs search "<關鍵字>"`
- 「我要修這個檔，以前踩過什麼坑？」→ 直接查 `traps/by-file.json`
- 操作守門：`node .vscode/knowledge/scripts/kb.mjs repair-preflight --tool=terminal --command="..." --intent="..."`
- 失敗記錄：`node .vscode/knowledge/scripts/kb.mjs repair-record --tool=terminal --command="..." --exit-code=1 --error="摘要"`
- 重複失敗檢查：`node .vscode/knowledge/scripts/kb.mjs repair-status`、`node .vscode/knowledge/scripts/kb.mjs repair-health`
- 其他 facet：`by-{module,tag,topic,symptom}.json`

向用戶回報：涉及模組、命中主題（topic slug）、命中陷阱編號、待探索範圍。

### Agent 操作錯誤閉環

- 執行 shell、搜尋 `.vscode` / `.vscode/knowledge`、批次改檔、或重跑曾失敗命令前，先使用 `repair-preflight`。
- 任一工具/命令失敗後，不得原樣重試；先用 `repair-record` 記錄 sanitized failure，再用 `repair-status` 檢查是否重複。
- 同一 fingerprint 第 2 次失敗即視為 pending repair；必須改方法，或新增/更新 operational trap。
- runtime ledger 僅保存摘要與 hash；禁止保存 `.env`、token、密碼、完整 API key 或大段 stdout。

### 任務結束後

依以下固定順序執行，不得跳過：

1. **新增/更新 trap fragment**：
   - 新陷阱：

         node .vscode/knowledge/scripts/kb.mjs new-trap `
           --module=X --title="..." `
           --topics=slug1,slug2 `
           --symptoms="症狀A;症狀B" `
           --files=path.ext --tests=tests/...

     再編輯生成的 `traps/trap-NNN.md` 補完症狀/根因/修正/測試
   - 既有陷阱補充：直接編輯對應 `traps/trap-NNN.md`，必要時更新 `topics:` / `symptoms:`
2. **若需新主題 slug** → 編輯 `traps/topics-taxonomy.yml` 新增條目（slug / name / desc / keywords）
3. **若主題防呆原則需更新** → 編輯 `traps/topics/{slug}.md` 的 `<!-- AUTO_END -->` 以下段落（AUTO 區會被覆寫，不要動）
4. **更新模組知識**（若涉及商務規則或設計變更）：編輯 `modules/{m}/quickref.md` 或細節分檔
5. **更新當月 changelog**：在 `changelog/YYYY-MM.md` 最上方新增一行 `| 日期 | 模組 | 摘要 | 異動檔案 | 備註 |`
6. **檢查 Agent 操作錯誤閉環**：若本任務發生工具/命令失敗或重複嘗試，執行：

    node .vscode/knowledge/scripts/kb.mjs repair-status
    node .vscode/knowledge/scripts/kb.mjs repair-health

  若有 unresolved repeated failure，必須新增/更新 operational trap 或標註 false positive。
7. **OpenSpec change 收尾**（若本任務有建立 OpenSpec change）：

  a. 確認所有 tasks 均已 completed：WSL 使用 `./opsx status --change {name}`；Windows legacy 使用 `.\opsx status --change {name}` 或 `opsx.bat status --change {name}`
   b. 執行 `/opsx:archive` 封存 change（archive 後必須接著執行 rebuild）

   > 若任務僅為 bug 修復（無 OpenSpec change），跳過此步驟。

8. **重建索引並體檢**：

       node .vscode/knowledge/scripts/kb.mjs rebuild
     node .vscode/knowledge/scripts/kb.mjs finish-check

  `rebuild` 會自動：重建 `index.jsonl` + facet JSON + `topics/{slug}.md` AUTO 區 + `fts.db`。
  `finish-check` 必須 0 errors 才算結束。

9. **輸出 commit 訊息**（見下方「Commit 訊息格式」），此為任務最後一步。

> Token 不足時仍需最後提供 commit 訊息；若知識庫或驗證未完成，必須明確列為未完成事項。

### Commit 訊息格式（唯一定義）

以純文字段落輸出，禁止放入 fenced code block：

    {type}({模組}) 摘要說明

    問題:
    - 逐條說明

    變更:
    - 逐條說明

    測試:（僅撰寫測試時才提供）
    - 逐條說明

type 選項：`feat` / `fix` / `hotfix` / `refactor` / `chore` / `docs`

### 檔案編碼規範（防亂碼）

- 所有知識庫檔案必須為 **UTF-8 without BOM**
- 使用 VS Code 編輯器直接儲存，或透過 Node `fs.writeFileSync(..., 'utf8')`
- **禁止** PowerShell `Set-Content` / `Get-Content | Set-Content` / `(Get-Content) -replace`（以 CP950 覆寫，中文永久損毀）
- 健康度檢查：`node .vscode/knowledge/scripts/kb.mjs health`

### Trap fragment 格式（v3）

每筆 trap 為獨立檔案 `traps/trap-NNN.md`，固定 YAML frontmatter：

    ---
    id: 1
    title: 一句話摘要（< 80 字）
    module: SomeModule
    topics: [slug-a, slug-b]                              # 必填，必須在 topics-taxonomy.yml 白名單內
    symptoms:                                              # 可選，每筆症狀短語（協助 by-symptom 索引）
      - 症狀短語 A
      - 症狀短語 B
    related: [12, 34]                                      # 相關 trap id
    date: 2026-04-22
    status: fixed          # fixed | open | archived
    severity: bug          # bug | design | perf | doc
    tags: [tag-a]
    files:
      - src/path/to/file.ext
    tests:
      - tests/path/to/test.ext
    ---

    ## 症狀
    ## 根因
    ## 修正
    ## 測試

衍生產物（`index.jsonl` / `by-*.json` / `topics/{slug}.md` AUTO 區 / `fts.db`）由 `kb.mjs rebuild` 自動生成，**禁止手動編輯**。

### 禁止行為

1. 禁止 PowerShell `Set-Content` 寫入知識庫（CP950 編碼會永久毀掉中文）
2. 禁止在回應中使用 fenced code block 提供 SQL/程式碼（VS Code Chat 視窗會隱藏）
3. 禁止手動編輯 `traps/index.jsonl`、`by-*.json`、`topics/{slug}.md` 的 AUTO 區、`fts.db`（會被 rebuild 覆寫）
4. 新增 trap 必須走 `kb.mjs new-trap`（自動取下一個 id，避免衝突；自動校驗 topics 白名單）
5. 禁止使用未登記於 `topics-taxonomy.yml` 的 topic slug
6. 禁止 `/opsx:archive` 後未執行 `kb.mjs rebuild`（topics 防呆原則中的 spec 連結無法被 FTS 索引）
7. 禁止直接修改 `.vscode/openspec/specs/` 而不更新 `specs/INDEX.md`（Requirements 數量失同步）

---

### OpenSpec 與知識庫分工

| 維度 | OpenSpec（`.vscode/openspec/`）| 知識庫（`.vscode/knowledge/`）|
|------|----------------------|-------------------------------|
| **回答問題** | WHAT（行為契約、需求規格）| WHY/HOW-NOT-TO（根因、修正歷程）|
| **觸發時機** | 新功能 / 規格變更 | 踩坑、bug 修復 |
| **入口指令** | `/opsx:propose`、`/opsx:explore` | `kb.mjs new-trap` |
| **結束動作** | `/opsx:archive` → `kb.mjs rebuild` | `kb.mjs rebuild` |

---

## 四、測試規範

每次修改或新增功能，必須同步撰寫對應測試；Lintr 開發採 TDD 為預設流程。

### TDD 強制流程

1. Red：先寫會失敗的單元或整合測試，明確鎖定新行為或 bug 修正。
2. Green：只寫足以讓測試通過的最小實作，不提前加入未使用抽象。
3. Refactor：測試通過後才整理命名、抽象與 crate 邊界，並再次執行完整驗證。
4. 若暫時無法測試，必須在回報中說明原因、風險與替代驗證方式。

### 測試層次

| 層次 | 位置 | 覆蓋內容 |
|------|------|----------|
| 單元測試 | 各 crate `src/` 內 `#[cfg(test)]` | core 型別、parser wrapper、config parsing、單條 rule 行為 |
| 整合測試 | `crates/lintr/tests/` | `lint()` / `lint_file()` public API、config filtering、diagnostic ordering |
| Fixture 測試 | `tests/fixtures/` 或 crate-local fixtures | Python source 範例、TOML / pyproject config 範例 |

### 必跑指令

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo build --workspace`

### SOLID Review Gate

- 新增規則前，先確認是否能只新增 rule module 與 registry entry。
- 修改 core trait 前，必須確認所有既有 rule 都仍可代換，且沒有把 parser/rules/config 依賴帶回 core。
- config 只能描述規則啟停與 rule settings，不應知道具體規則的實作細節。
