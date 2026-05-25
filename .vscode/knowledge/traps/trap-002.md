---
id: 2
title: WSL 使用者缺少 sudo 群組會阻擋 apt 工具鏈安裝
module: lintr
topics: [command-preflight, agent-runtime-failure]
symptoms: ["sudo -v 回報 may not run sudo", "apt-get install 無法由 charles 執行", "WSL 內呼叫 wsl.exe 回 Exec format error"]
related: []
date: "2026-05-25"
status: fixed
severity: bug
tags: []
files: [.vscode/knowledge/agent/INDEX.md, .vscode/knowledge/modules/lintr/decisions/decision-001.md]
tests: [sudo-v, apt-get-install]
---
## 症狀

- `sudo -v` 回報 `Sorry, user charles may not run sudo on Charles.`。
- Phase 2 的 `sudo apt-get update` / `sudo apt-get install ...` 無法由 `charles` 執行。
- 嘗試在 WSL bash 內直接呼叫 `/mnt/c/WINDOWS/system32/wsl.exe` 取得 root 入口時，回報 `cannot execute binary file: Exec format error`。

## 根因

- Ubuntu WSL 初始使用者狀態不符合 Phase 2 apt 安裝需求：`id` 顯示 `charles` 只在自己的群組內，`getent group sudo` 顯示 sudo 群組只有 `ubuntu`。
- `build-essential`、`sqlite3`、`libsqlite3-dev`、`pkg-config` 屬於 apt 系統套件，無法用 user-space 安裝完整取代。
- WSL 內的 Windows interop 不一定可直接執行 `wsl.exe`；修復 sudo membership 需改在 Windows PowerShell / Windows Terminal 進入 root。

## 修正

- 從 Windows PowerShell 執行 `wsl -d Ubuntu-24.04 -u root -- bash -lc "usermod -aG sudo charles && getent group sudo && id charles"`。
- 執行 `wsl --terminate Ubuntu-24.04` 後重新開啟 WSL / VS Code Remote - WSL，使群組 membership 生效。
- 回到 WSL 後執行 `sudo apt-get update` 與 `sudo apt-get install -y build-essential sqlite3 libsqlite3-dev pkg-config`。

## 測試

- `id` 應顯示 `charles` 屬於 `sudo` 群組。
- `sudo -v` 不再回報 may not run sudo。
- `sudo apt-get install -y build-essential sqlite3 libsqlite3-dev pkg-config` exit code 0。
- `gcc --version`、`make --version`、`sqlite3 --version`、`pkg-config --version` 均有輸出。
