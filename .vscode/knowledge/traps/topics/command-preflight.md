# Topic: 命令執行前 preflight

> 自動產生 — `AUTO_BEGIN/AUTO_END` 之間請勿手動編輯（會被 `kb.mjs rebuild` 覆寫）。

<!-- AUTO_BEGIN -->
**定義**：已知錯誤或高風險 shell 指令需先由 repair-preflight 檢查，避免 Agent 原樣重複執行
**關鍵字**：preflight, command, PowerShell, terminal, retry
**相關 trap 數**：3

## 相關 Trap

| id | 一句話 | 主要檔案 | 連結 |
|----|--------|---------|------|
| 1 | WSL opsx wrapper 因 CRLF shebang 或 Windows PATH 失效 | opsx, .vscode/openspec-cheatsheet.md | [→](../trap-001.md) |
| 2 | WSL 使用者缺少 sudo 群組會阻擋 apt 工具鏈安裝 | .vscode/knowledge/agent/INDEX.md, .vscode/knowledge/modules/lintr/decisions/decision-001.md | [→](../trap-002.md) |
| 3 | RTK 同名套件可能不是 Rust Token Killer | .vscode/knowledge/agent/INDEX.md, .vscode/knowledge/modules/lintr/decisions/decision-001.md | [→](../trap-003.md) |
<!-- AUTO_END -->

## 防呆原則

- 在 WSL 內驗證 shell wrapper 前，先確認 wrapper 是 LF line ending，且 `command -v node openspec` 不指向 `/mnt/c/...`。
- `./opsx`、`openspec`、`kb.mjs` 初期保持 canonical/raw；若失敗，先記 repair fingerprint，再改用直接讀檔或 Linux Node fallback。
- 需要 apt 安裝前先跑 `sudo -v`；若使用者不在 sudo 群組，改從 Windows PowerShell 以 WSL root 修復 membership，不要在 WSL bash 內重複 apt。
- 安裝 RTK 前先辨識套件來源；npm `rtk` 與 crates.io `rtk` 都不是 Rust Token Killer，必須以 `TokenFleet-AI/rtk` git 來源安裝並用 `rtk gain` 驗證。
