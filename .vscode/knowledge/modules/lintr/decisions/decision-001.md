---
id: 1
title: 採用 WSL 2 + RTK 作為 Agent 命令執行層
module: lintr
date: 2026-05-25
related_traps: []
---

## 決策

第一階段直接採用 WSL 2 + VS Code Remote - WSL 作為主要命令執行環境，並在 WSL 內導入 RTK 作為高噪音 shell 輸出的 token 壓縮層。Windows 端 `D:\www\Lintr` 僅作短期 rollback 來源，不作長期雙寫。

Phase 0 基準如下：

- Windows 端 `git status --short` 無未提交檔案。
- Windows 端 `git status --short --branch` 顯示 `main...origin/main [gone]`，WSL 端重新建立 workspace 時需重新確認遠端追蹤狀態。
- Repo 長期目標路徑為 WSL Linux filesystem，例如 `~/www/Lintr`；避免從 WSL 長期操作 `/mnt/d/www/Lintr`。

Phase 1 基準如下：

- WSL 版本為 2.6.3.0，預設 WSL 版本為 2。
- Ubuntu 24.04 LTS 已透過 `winget install --id Canonical.Ubuntu.2404` 安裝；`wsl --install` 與 `--web-download` 下載路徑曾因 `WININET_E_TIMEOUT` 失敗。
- WSL 預設 distro 已設為 `Ubuntu-24.04`，Ubuntu 預設使用者為 `charles`。
- Linux filesystem workspace 已建立於 `/home/charles/www/Lintr`，Windows 端 repo 已一次性複製過去。
- WSL copy 為了對齊目前 Windows 工作樹 CRLF 狀態，已在該 repo 的 local Git config 設定 `core.autocrlf=true`；後續可在專案正式切 WSL 後再規劃 LF 正規化。
- VS Code Remote - WSL extension 已安裝，可用 `code --remote wsl+Ubuntu-24.04 /home/charles/www/Lintr` 開啟主要工作視窗。

Phase 2 基準如下：

- Rust 透過 rustup minimal 安裝於 `~/.cargo/bin`，驗證版本為 `rustc 1.95.0`、`cargo 1.95.0`。
- Node.js 以官方 Linux x64 tarball 安裝於 `~/.local/opt/node-current`，shim 位於 `~/.local/bin`，驗證版本為 `node v24.16.0`、`npm 11.13.0`。
- ripgrep 以 Linux x64 release binary 安裝於 `~/.local/bin/rg`，驗證版本為 `ripgrep 15.1.0`。
- OpenSpec CLI 安裝正確套件 `@fission-ai/openspec`，`openspec` 解析到 `~/.local/bin/openspec`，驗證版本為 `1.3.1`；錯誤的 `openspec@0.0.0` 不可作 CLI 來源。
- Ubuntu apt 套件已安裝 `build-essential`、`sqlite3`、`libsqlite3-dev`、`pkg-config`；驗證版本包含 `git 2.43.0`、`sqlite3 3.45.1`、`gcc 13.3.0`、GNU Make `4.3`、`pkg-config 1.8.1`。
- 初始 `charles` 使用者不在 `sudo` 群組；已由 Windows PowerShell 以 root 進入 WSL 修復 sudo membership 後完成 apt 安裝。
- 目前 repo 尚無 root `Cargo.toml`；`cargo fmt/clippy/test/build --workspace` 待 Lintr MVP workspace scaffold 後驗證。

RTK 初期排除範圍如下：

- `node .vscode/knowledge/scripts/kb.mjs ...`
- `opsx` / `openspec`
- 安裝、下載、環境變數、log、可能含 secrets 的輸出

## 原因

Lintr 是 Rust/Cargo workspace，WSL 2 能讓 AI Chat terminal、Cargo、Node、OpenSpec 與 RTK 在同一個 Linux shell 中運作，降低 PowerShell quoting、encoding、路徑分隔與 shell hook 相容性問題。

RTK 能降低 `git`、`rg`、Cargo test/build/lint 等高噪音命令的 context 成本，但它只負責壓縮命令輸出，不取代 `.vscode/knowledge`、OpenSpec 或 `copilot-instructions.md`。若 RTK 摘要不足以判斷錯誤，必須改用 canonical command、verbose/raw fallback 或 tee failure log。
