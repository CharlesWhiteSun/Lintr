---
id: 3
title: RTK 同名套件可能不是 Rust Token Killer
module: lintr
topics: [command-preflight, agent-runtime-failure]
symptoms: ["rtk gain 失敗或沒有 gain 子命令", "npm rtk 描述為 release tool", "crates.io rtk 描述為 Rust Type Kit"]
related: []
date: "2026-05-25"
status: fixed
severity: bug
tags: []
files: [.vscode/knowledge/agent/INDEX.md, .vscode/knowledge/modules/lintr/decisions/decision-001.md]
tests: [rtk-version, rtk-gain]
---
## 症狀

- `rtk gain` 失敗、沒有 `gain` 子命令，或輸出不像 token savings 統計。
- npm `rtk` 的描述是 release/changelog/version tagging tool。
- crates.io `rtk` 的描述是 Rust Type Kit，與命令輸出壓縮無關。

## 根因

- `rtk` 這個名稱存在多個不相干套件；直接 `npm install -g rtk` 或 `cargo install rtk` 會拿到錯誤工具。
- 本計劃需要的是 Rust Token Killer，來源為 `TokenFleet-AI/rtk` git repository。
- 未先跑 `rtk --version` 與 `rtk gain` 就啟用 RTK，會把同名錯包誤當作輸出壓縮層。

## 修正

- 先查套件 metadata：npm `rtk` / crates.io `rtk` 都不是本計劃 RTK。
- 使用 `cargo install --git https://github.com/TokenFleet-AI/rtk --locked --force` 安裝。
- Phase 3 只直接呼叫 `rtk ...` smoke tests；不執行 `rtk init` 或 hook 自動 rewrite。

## 測試

- `command -v rtk` 應為 `/home/charles/.cargo/bin/rtk`。
- `rtk --version` 應輸出 `rtk 0.40.0`。
- `rtk gain` 應可輸出 Global Scope token savings；初次無資料時，先跑 `rtk git status` / `rtk ls .` / `rtk read .vscode/knowledge/INDEX.md` 後再查一次。
- `rtk git status`、`rtk ls .`、`rtk read .vscode/knowledge/INDEX.md` 均可成功執行。
