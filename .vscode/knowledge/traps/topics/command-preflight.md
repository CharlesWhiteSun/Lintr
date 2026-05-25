# Topic: 命令執行前 preflight

> 自動產生 — `AUTO_BEGIN/AUTO_END` 之間請勿手動編輯（會被 `kb.mjs rebuild` 覆寫）。

<!-- AUTO_BEGIN -->
**定義**：已知錯誤或高風險 shell 指令需先由 repair-preflight 檢查，避免 Agent 原樣重複執行
**關鍵字**：preflight, command, PowerShell, terminal, retry
**相關 trap 數**：1

## 相關 Trap

| id | 一句話 | 主要檔案 | 連結 |
|----|--------|---------|------|
| 1 | WSL opsx wrapper 因 CRLF shebang 或 Windows PATH 失效 | opsx, .vscode/openspec-cheatsheet.md | [→](../trap-001.md) |
<!-- AUTO_END -->

## 防呆原則

- 在 WSL 內驗證 shell wrapper 前，先確認 wrapper 是 LF line ending，且 `command -v node openspec` 不指向 `/mnt/c/...`。
- `./opsx`、`openspec`、`kb.mjs` 初期保持 canonical/raw；若失敗，先記 repair fingerprint，再改用直接讀檔或 Linux Node fallback。
