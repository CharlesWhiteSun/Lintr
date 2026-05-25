# Agent 操作守門

> 目的：把 AI Agent 的讀取失敗、錯誤命令、錯誤搜尋路徑與重複嘗試，轉成可記錄、可阻斷、可回歸的知識。

## 執行環境與 RTK 政策

- 第一階段直接採用 WSL 2 + VS Code Remote - WSL 作為主要命令執行環境；Windows 端 `D:\www\Lintr` 僅作短期 rollback 來源。
- WSL 長期工作目錄應位於 Linux filesystem，例如 `~/www/Lintr`；不建議從 WSL 長期操作 `/mnt/d/www/Lintr`。
- Phase 0 基準：Windows 端 `git status --short` 無未提交檔案；`git status --short --branch` 顯示 `main...origin/main [gone]`，WSL 端重建遠端追蹤前需注意。
- Phase 1 基準：已安裝 Ubuntu 24.04 LTS WSL 2，預設 distro 為 `Ubuntu-24.04`，預設使用者為 `charles`，Linux workspace 為 `/home/charles/www/Lintr`。
- Phase 2 基準：Rust/Cargo、Node/npm、ripgrep、git、SQLite/build tools 與 Linux OpenSpec 已安裝在 WSL shell；`node` / `openspec` / `rg` 不得解析到 `/mnt/c/...`。
- Phase 3 基準：RTK `0.40.0` 已由 `cargo install --git https://github.com/TokenFleet-AI/rtk --locked --force` 安裝至 `/home/charles/.cargo/bin/rtk`；`rtk gain`、`rtk git status`、`rtk ls .`、`rtk read .vscode/knowledge/INDEX.md` 已驗證。
- Phase 4 基準：`rtk init -g --copilot --auto-patch` 已安裝 project-scoped Copilot hook 檔；`rtk rewrite` / `rtk hook check --agent copilot` 已驗證 `git status`、`rg ...`、`cargo test` 可 rewrite。
- VS Code Remote - WSL extension 已安裝；主要工作視窗應以 `code --remote wsl+Ubuntu-24.04 /home/charles/www/Lintr` 或 Remote - WSL 指令開啟。
- RTK 只作為高噪音 shell 輸出的 token 壓縮層，不是知識來源；`.vscode/knowledge` 仍以直接讀檔、列目錄或 include ignored 搜尋為準。
- 既有 AI Chat terminal 可能不會即時載入新 Copilot hook；若 raw `git status` / `rg` 未透明 rewrite，先用顯式 `rtk ...` fallback，並在重啟 IDE/Copilot session 後再驗證。
- RTK hook config 必須保留 raw/canonical 排除：`node .vscode/knowledge/scripts/kb.mjs ...`、`opsx` / `openspec`、安裝/下載命令、`env` / log / 可能含 secrets 的輸出。
- 使用 RTK 後若需要完整錯誤上下文，改用 canonical command、RTK verbose/raw fallback，或讀取 RTK tee failure log；不得只依壓縮摘要做破壞性判斷。

## 任務開始

1. 先讀本檔，再執行 `start-check`。
2. 執行 shell、搜尋 `.vscode`、批次改檔、或重跑曾失敗命令前，先執行：

  node .vscode/knowledge/scripts/kb.mjs repair-preflight --tool=<tool> --command="..." --path=path --intent="..."

3. preflight 回傳 `deny` 時不得執行原命令；改用輸出的替代方式。
4. preflight 回傳 `warn` 時可繼續，但需先說明風險與替代策略。

## 失敗閉環

1. 任一工具/命令失敗後，不得原樣重試。
2. 先記錄 sanitized failure：

  node .vscode/knowledge/scripts/kb.mjs repair-record --tool=<tool> --command="..." --path=path --exit-code=1 --intent="..." --error="摘要"

3. 再檢查是否重複：

  node .vscode/knowledge/scripts/kb.mjs repair-status

4. 同一 fingerprint 第 2 次失敗即視為 pending repair；必須改方法或新增/更新 operational trap。

## Fingerprint 欄位

| 欄位 | 說明 |
|------|------|
| tool | terminal / search / read / edit / browser / subagent |
| cwd | 執行目錄，預設為專案根目錄 |
| command | 正規化後的命令或工具摘要 |
| path | 主要操作路徑 |
| exit_code | 命令或工具失敗代碼 |
| error_hash | 錯誤摘要 hash，不保存完整敏感輸出 |
| intent | 本次嘗試目的 |

## 收尾規則

- `repair-health` 必須 0 errors，任務才可結束。
- repeated failure 必須升級成 `traps/trap-NNN.md` 的 operational trap，或寫入 false positive 並附到期日。
- runtime ledger 僅保存摘要與 hash；禁止保存 `.env`、token、密碼、完整 API key 或大段 stdout。

## 既知高風險

- `.vscode` / `.vscode/knowledge` 可能被一般搜尋忽略；請改用直接讀檔、列目錄或 include ignored。
- Windows PowerShell 5.1 不使用 `&&`；請用分號或分開命令。
- WSL shell wrapper（例如 `opsx`）與 `.vscode/**` 工作流/知識庫檔必須保持 LF line ending，並用 `.gitattributes` 固定；若 shebang 變成 `sh\r`，先修正換行再驗證。
- WSL-first 階段不得把 `/mnt/c/...` 的 Windows `node`、`npm`、`npx` 或 `openspec` 當成 Linux baseline。
- 安裝 RTK 時不得使用 npm `rtk`（release tool）或 crates.io `rtk`（Rust Type Kit）；若 `rtk gain` 不存在或失敗，先移除錯包並改用 `TokenFleet-AI/rtk` git 來源。
- 改動 `/home/charles/.config/rtk/config.toml` 後必須跑 `rtk config`；不要留下不完整 section（例如缺欄位的 `[tee]`），否則 hook/rewrite 會退化或報 parse error。
- 若 `sudo -v` 回報 `user charles may not run sudo`，需從 Windows PowerShell 以 root 進入 WSL 執行 `usermod -aG sudo charles`，重啟 distro 後再跑 apt；不要在 WSL bash 內原樣重試 apt。
- 禁止 `Get-Content | Set-Content`、`Set-Content`、`(Get-Content) -replace` 改寫知識庫 UTF-8 檔案。
- `/start-plan` 的 `agent` 值以本機 VS Code diagnostics 為準；若 `Plan` 合法，不要擅自改成 lowercase `plan`。
- VS Code `files.encoding` 應使用 `utf8`，不是 `utf-8`。
