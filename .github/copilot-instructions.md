<!-- rtk-instructions v2 -->
# RTK — Token-Optimized CLI

**rtk** is a CLI proxy that filters and compresses command outputs, saving 60-90% tokens.

## Rule

Use RTK for allowed high-noise shell commands in this WSL workspace:

```bash
# Instead of:              Use:
git status                 rtk git status
git log -10                rtk git log -10
cargo test                 rtk cargo test
docker ps                  rtk docker ps
kubectl get pods           rtk kubectl pods
```

Do not auto-prefix project control, install/download, env/log, or possible secret-reading commands:

```bash
node .vscode/knowledge/scripts/kb.mjs ...
./opsx ...
openspec ...
env / printenv
cat / head / tail / less / more
curl / wget / npm install / npm ci / npx / cargo install / apt / apt-get
```

If the transparent Copilot hook is not active in the current IDE or Chat session, use explicit `rtk ...` fallback for allowed high-noise commands.

## Meta commands (use directly)

```bash
rtk gain              # Token savings dashboard
rtk gain --history    # Per-command savings history
rtk discover          # Find missed rtk opportunities
rtk proxy <cmd>       # Run raw (no filtering) but track usage
```
<!-- /rtk-instructions -->
