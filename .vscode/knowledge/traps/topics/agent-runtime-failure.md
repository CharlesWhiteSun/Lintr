# Topic: Agent runtime failure 閉環

> 自動產生 — `AUTO_BEGIN/AUTO_END` 之間請勿手動編輯（會被 `kb.mjs rebuild` 覆寫）。

<!-- AUTO_BEGIN -->
**定義**：AI Agent 工具/命令失敗後需記錄 fingerprint、停止原樣重試，並升級為 repair guard 或 trap
**關鍵字**：Agent, repair-record, repair-health, fingerprint, repeated failure
**相關 trap 數**：3

## 相關 Trap

| id | 一句話 | 主要檔案 | 連結 |
|----|--------|---------|------|
| 1 | WSL opsx wrapper 因 CRLF shebang 或 Windows PATH 失效 | opsx, .vscode/openspec-cheatsheet.md | [→](../trap-001.md) |
| 2 | WSL 使用者缺少 sudo 群組會阻擋 apt 工具鏈安裝 | .vscode/knowledge/agent/INDEX.md, .vscode/knowledge/modules/lintr/decisions/decision-001.md | [→](../trap-002.md) |
| 3 | RTK 同名套件可能不是 Rust Token Killer | .vscode/knowledge/agent/INDEX.md, .vscode/knowledge/modules/lintr/decisions/decision-001.md | [→](../trap-003.md) |
<!-- AUTO_END -->

## 防呆原則

- WSL-first 工具失敗時，若 `node` 本身不在 PATH，可用 VS Code Server 內建的 Linux Node 作為臨時 `kb.mjs repair-record` 執行器，但不得把 Windows Node/npm shim 視為已驗證環境。
- `repair-status` 未出現 pending 不代表可忽略根因；若 failure 暴露可重複踩到的 wrapper、PATH、encoding 問題，仍應補 operational trap。
- sudo membership 或 Windows interop 失敗時，記錄 fingerprint 後改用 Windows PowerShell root repair；不要把同一條 WSL 內 `wsl.exe` 或 apt 命令原樣重試。
- 若 `rtk --version` 找不到或 `rtk gain` 不符合 token savings 統計，記錄 failure 後先檢查是否安裝到同名錯包，不要直接啟用 hook。
