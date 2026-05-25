# Lintr

Lintr 是一個規劃中的 Rust library，用來分析 Python source code 並回傳 linter diagnostics。MVP 先聚焦在 library API、穩定診斷排序、TOML 設定，以及一組內建 style / bug / security 規則。

## 目前狀態

此 repo 目前處於規格與工作流基準階段，Rust Cargo workspace 尚未 scaffold。

| 項目 | 狀態 |
|------|------|
| WSL 2 workspace | 已就緒，路徑為 `/home/charles/www/Lintr` |
| OpenSpec 工作流 | 已就緒，WSL 內使用 `./opsx` |
| 知識庫檢查 | 已就緒，使用 `kb.mjs rebuild` 與 `finish-check` |
| RTK 命令輸出壓縮 | 已就緒，僅用於允許的高噪音命令 |
| Rust Cargo workspace | 尚未建立；目前沒有 root `Cargo.toml` |
| Lintr library API | 已規格化，尚未實作 |
| CLI / LSP / autofix | 不在 MVP 範圍內 |

因為 Cargo workspace 尚未建立，`cargo test --workspace` 這類命令目前預期會失敗；待 MVP scaffold 完成後才會成為正常驗證流程。

## MVP 功能範圍

Lintr 第一版會是一個分析 Python source code 的 Rust library。

預計 public API：

| API | 用途 |
|-----|------|
| `lint(source, config)` | 分析 Python source text，回傳已啟用規則的 diagnostics |
| `lint_file(path, config)` | 讀取 Python 檔案後，套用與 `lint()` 相同的分析契約 |

預計行為：

- 解析合法 Python source，並執行已啟用的 lint rules。
- 遇到非法 Python source 時回傳 typed parse error；parse 失敗後不得執行 rules。
- `lint_file()` 讀檔失敗時回傳 typed IO error。
- 在執行 rule 前套用 config，包含 rule code filtering、category filtering 與 per-rule settings。
- diagnostics 穩定排序：start offset、end offset、rule code。

MVP 不包含：

- CLI binary
- LSP integration
- Autofix
- Plugin system
- Type inference
- 跨檔 import graph
- 深度 dataflow analysis

## 預計內建規則

| Code | Category | Rule |
|------|----------|------|
| `E001` | Style | Line too long |
| `E002` | Style | Trailing whitespace |
| `E003` | Style | Missing whitespace around operator |
| `B001` | Bug | Mutable default argument |
| `B002` | Bug | Compare to `None` with `==` / `!=` |
| `B003` | Bug | Unreachable code |
| `S001` | Security | Use of `eval` |
| `S002` | Security | Hardcoded password |

## 預計 Rust 使用方式

以下範例描述 Cargo workspace 與 crates 完成後的預期 API。這些 API 目前尚未實作。

```rust
use lintr::{lint, Config};

fn main() -> Result<(), lintr::Error> {
    let source = "password = 'secret'\n";
    let config = Config::default();
    let diagnostics = lint(source, &config)?;

    for diagnostic in diagnostics {
        println!("{}: {}", diagnostic.code, diagnostic.message);
    }

    Ok(())
}
```

檔案分析預計沿用相同 contract：

```rust
use lintr::{lint_file, Config};

fn main() -> Result<(), lintr::Error> {
    let config = Config::default();
    let diagnostics = lint_file("src/example.py", &config)?;

    for diagnostic in diagnostics {
        println!("{}: {}", diagnostic.code, diagnostic.message);
    }

    Ok(())
}
```

## 預計設定格式

Lintr 將支援 `lintr.toml` 與 `pyproject.toml` 的 `[tool.lintr]` 區塊。

`lintr.toml` 範例：

```toml
max_line_length = 100
select = ["E", "B", "S"]
ignore = ["E003"]

[rules.E001]
max_line_length = 100
```

`pyproject.toml` 範例：

```toml
[tool.lintr]
max_line_length = 100
select = ["E", "B", "S"]
ignore = ["E003"]
```

實際欄位名稱會在 MVP 實作 `lintr-config` crate 時由測試鎖定。

## 開發環境

第一階段基準環境是 WSL 2 + VS Code Remote - WSL。請從 Linux filesystem workspace 工作：

```bash
cd /home/charles/www/Lintr
code --remote wsl+Ubuntu-24.04 /home/charles/www/Lintr
```

不要把 Windows 端 repo 當成長期主要工作區；Windows copy 只作短期 rollback 來源。

## OpenSpec 工作流

OpenSpec 用來記錄行為契約與實作計劃。在 WSL 內，從 repo root 使用 Linux wrapper：

```bash
./opsx list
./opsx status --change lintr-python-linter-mvp
./opsx instructions apply --change lintr-python-linter-mvp --json
```

Windows wrapper `opsx.ps1` 與 `opsx.bat` 只作 legacy fallback。在 WSL 內若 `./opsx` 失效，優先修正 `./opsx`、LF line endings、PATH、Node.js 或 OpenSpec 安裝，不改回長期使用 Windows wrapper。

## 知識庫工作流

知識庫記錄 project decisions、operational traps、quickrefs 與 changelogs。這些檢查使用 canonical Node commands，不透過 RTK wrapper：

```bash
node .vscode/knowledge/scripts/kb.mjs rebuild
node .vscode/knowledge/scripts/kb.mjs finish-check
```

目前已知 warning：`lintr-python-linter-mvp` 仍有未完成 OpenSpec tasks，原因是 Cargo workspace 與 library crates 尚未實作。

## RTK 使用方式

RTK 是命令輸出壓縮層，不取代 OpenSpec、知識庫或 source-of-truth 文件。

可將 RTK 用於允許的高噪音命令：

```bash
rtk git status
rtk git log -10
rtk grep "pattern" .
rtk cargo test
```

以下命令維持 canonical/raw：

```bash
node .vscode/knowledge/scripts/kb.mjs ...
./opsx ...
openspec ...
env / printenv
cat / head / tail / less / more
curl / wget / npm install / npm ci / npx / cargo install / apt / apt-get
```

RTK analytics 指令：

```bash
rtk gain
rtk discover
rtk session
```

如果壓縮摘要遮蔽錯誤，請改跑 canonical/raw command 取得完整輸出，並記錄 operational trap。

## Repo 結構

```text
.
├── crates/
│   ├── lintr/          # 預計 public API crate
│   ├── lintr-config/   # 預計 TOML config crate
│   ├── lintr-core/     # 預計 common types 與 Rule<C> abstraction
│   ├── lintr-parser/   # 預計 Python parser wrapper
│   └── lintr-rules/    # 預計內建 rule implementations
├── opsx                # WSL / Linux OpenSpec wrapper
├── opsx.bat            # Windows CMD fallback
├── opsx.ps1            # Windows PowerShell fallback
└── .vscode/
    ├── copilot-instructions.md
    ├── knowledge/
    └── openspec/
```

## 驗證指令

Cargo workspace 建立後，程式碼變更應執行：

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build --workspace
```

在 root `Cargo.toml` 尚未存在前，先使用目前已可執行的工作流檢查：

```bash
./opsx list
node .vscode/knowledge/scripts/kb.mjs rebuild
node .vscode/knowledge/scripts/kb.mjs finish-check
rtk gain
rtk discover
rtk session
```

## Source Of Truth

- 專案規範與架構：`.vscode/copilot-instructions.md`
- Lintr quick reference：`.vscode/knowledge/modules/lintr/quickref.md`
- MVP OpenSpec change：`.vscode/openspec/changes/lintr-python-linter-mvp/`
- WSL-first / RTK decision：`.vscode/knowledge/modules/lintr/decisions/decision-002.md`
