# E2E Test Requirements

## CRITICAL: Always Run Latest Code

Before running the E2E test, you MUST ensure BOTH binaries are current:

### Zed binary (Rust side)
```bash
# 1. Check if zed-build binary matches current zed repo HEAD
cd ~/pm/zed && git log --oneline -1
stat -c '%y' ~/pm/helix/zed-build/zed

# 2. If stale, rebuild:
cd ~/pm/helix && ./stack build-zed dev  # ~3min dev, ~12min release

# 3. ALWAYS copy to e2e test dir before running:
cp ~/pm/helix/zed-build/zed ~/pm/zed/crates/external_websocket_sync/e2e-test/zed-binary
```

### Go test server (Helix side)
The `run_docker_e2e.sh` script rebuilds this automatically from the local helix checkout (via the `replace` directive in go.mod). Pass `--no-build` ONLY if you are certain no Go code has changed.

If `go mod tidy` fails, run it manually first:
```bash
cd ~/pm/zed/crates/external_websocket_sync/e2e-test/helix-ws-test-server && go mod tidy
```

### Verification
The test prints binary timestamps and md5 checksums at startup:
```
=== Binary versions ===
  zed-binary:          2026-03-24 11:17:23  cc426038391008b2
  helix-ws-test-server: 2026-03-24 14:40:21  6f17f887e47e
```
CHECK THESE. If the zed-binary timestamp is older than the latest zed commit, it's stale.

## Running the test
```bash
cd ~/pm/zed/crates/external_websocket_sync/e2e-test
./run_docker_e2e.sh              # full rebuild
./run_docker_e2e.sh --no-build   # ONLY if Go code hasn't changed
```

## Never trust `--no-build`
When investigating test failures, ALWAYS do a full rebuild. Stale binaries are a common source of confusion.
