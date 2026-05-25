---
id: 1
title: WSL opsx wrapper 因 CRLF shebang 或 Windows PATH 失效
module: lintr
topics: [command-preflight, agent-runtime-failure]
symptoms:
  - "/usr/bin/env: sh carriage return"
  - ./opsx list 在 WSL 內失敗
  - openspec 解析到 /mnt/c Windows npm path
related: []
date: "2026-05-25"
status: fixed
severity: bug
tags: []
files: [opsx, .vscode/openspec-cheatsheet.md]
tests: [./opsx-list]
---
## 症狀

- 在 WSL 內執行 `./opsx list` 時出現 `/usr/bin/env: 'sh\r': No such file or directory`。
- `command -v openspec` 解析到 `/mnt/c/Users/.../AppData/Roaming/npm/openspec`，代表 WSL shell 正在吃 Windows npm shim。
- `node .vscode/knowledge/scripts/kb.mjs ...` 因 Linux `node` 不在 PATH 而失敗，但 Windows `npm` / `npx` 仍被 PATH 帶入。

## 根因

- `opsx` wrapper 被 CRLF 化，shebang 的 `sh` 變成 `sh\r`，Linux `/usr/bin/env` 找不到對應 interpreter。
- Phase 1 要求 Node、OpenSpec、RTK 與 terminal 同在 WSL Linux shell；若 PATH 優先解析到 `/mnt/c/...`，實際仍在混用 Windows 工具鏈。
- 只檢查 wrapper 檔案存在不足以驗證 WSL-first；必須檢查 line ending、可執行權限與實際命中的 `node` / `openspec` 路徑。

## 修正

- 將 `opsx` 改為 LF shebang 的 POSIX shell wrapper。
- 新增 `.gitattributes`，固定 `.vscode/**`、`opsx` 與 `*.sh` 使用 LF，避免 local `core.autocrlf=true` 下次 checkout 或 Git touch 時復發。
- wrapper 先檢查 Linux `node` 與 `openspec` 是否存在，且拒絕 `/mnt/c/...` Windows path。
- 在 `copilot-instructions.md`、OpenSpec cheatsheet、agent guard、quickref 與 `kb.mjs openspec-check` 補上 WSL-first 驗證與 fallback 規則。

## 測試

- `sh -n opsx`
- `./opsx list`：目前因 Linux `node` 尚未安裝而預期失敗，錯誤需清楚停在 wrapper guard，而不是 `sh\r` 或 Windows npm shim。
- `kb.mjs repair-status`：確認本次記錄的 failure 沒有 pending 重複。
