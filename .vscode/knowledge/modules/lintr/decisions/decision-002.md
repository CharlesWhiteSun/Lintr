---
id: 2
title: 採用 WSL 2 + RTK 作為 Agent 命令執行層（Phase 5 工作流基準）
module: lintr
date: "2026-05-25"
related_traps: []
---
## 決策

Phase 5 將「WSL 2 + VS Code Remote - WSL + RTK」固定為 Agent 命令執行層與日常工作流基準。AI Chat terminal、Rust/Cargo、Node、OpenSpec、RTK 都應在 `/home/charles/www/Lintr` 的同一個 WSL Linux shell 中執行；Windows 端 repo 不再作為第一階段主要實作或驗證來源。

工作流規則如下：

- Repo 長期位於 WSL Linux filesystem，例如 `/home/charles/www/Lintr`；不長期雙寫 Windows repo 與 WSL repo。
- RTK 僅作為高噪音 shell 輸出的 token 壓縮層；知識來源仍是 `.vscode/knowledge`、OpenSpec specs/changes 與 `.vscode/copilot-instructions.md`。
- 允許 RTK hook 或顯式 `rtk ...` 包裝 `git`、`rg`、Cargo test/build/lint 等高噪音命令；若目前 session 未透明 rewrite，保留顯式 fallback。
- `kb.mjs`、`opsx` / `openspec`、安裝/下載、`env` / log / 可能含 secrets 的檔案讀取維持 canonical/raw，不由 RTK 自動壓縮或改寫。
- OpenSpec 在 WSL 中以 project root 的 `./opsx` 作為 canonical wrapper；`opsx.ps1` 與 `opsx.bat` 僅保留為 Windows legacy fallback。
- 知識庫收尾固定使用 canonical `node .vscode/knowledge/scripts/kb.mjs rebuild` 與 `node .vscode/knowledge/scripts/kb.mjs finish-check`；不得依賴 RTK hook rewrite 執行收尾命令。

## 原因

第一階段直接採 WSL 2 可讓 shell、路徑、編碼、Cargo、Node、OpenSpec 與 RTK hook 行為一致，避免 Windows PowerShell、WSL `/mnt/*` shim 與 Linux tooling 混用造成的判斷偏差。

RTK 對 Agent 有價值的地方是降低高噪音命令輸出進入 context 的成本；但規格與決策必須保持可追溯，因此 RTK 不能取代 knowledge base 或 OpenSpec。OpenSpec 流程也必須保留 raw/canonical 命令，確保 status、archive 與 rebuild 順序能被清楚驗證。

`./opsx` Linux wrapper 已存在且會拒絕 `/mnt/c/...` 的 Windows `node` / `openspec` shim；Phase 5 起將它視為 WSL-first OpenSpec 流程的必要維護項。若 wrapper 因 LF 或 PATH 失效，應修正 wrapper/環境，而不是改回長期使用 Windows wrapper。
