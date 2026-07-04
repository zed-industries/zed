# Docker Panel Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a native Docker panel to the Zed fork (branch `database-viewer`) for everyday container/image/compose management, local and over SSH.

**Architecture:** Mirror the existing database-viewer feature. Two new crates: `docker_client` (a `DockerClient` trait with a real `CliDockerClient` that shells out to `docker --format json` — setting `DOCKER_HOST=ssh://user@host` for remote endpoints — plus a `FakeDockerClient` for tests) and `docker_ui` (a GPUI dock panel: endpoint tree → containers/images/compose, detail views, log streaming, confirmations, read-only gate). Settings mirror the DB pattern: raw `DockerSettingsContent` in `settings_content` + a typed `DockerSettings` (`RegisterSetting`) in `docker_ui`. MCP tools are Phase 2 (out of scope here).

**Tech Stack:** Rust, GPUI, `tokio::process::Command` (subprocess), `serde`/`serde_json` (parse `--format json`), `gpui_tokio` (bridge async work into GPUI), `futures` channels (log streaming).

## Global Constraints

- Local builds of the fork require `--features gpui_platform/runtime_shaders` (machine has no Metal/Xcode). Verify compilation with `cargo check -p <crate> --features gpui_platform/runtime_shaders` for anything that pulls in `zed`/`gpui`; pure crates (`docker_client`, `settings_content`) compile without it.
- Lint via `./script/clippy -p <crate>` (not `cargo clippy`); must be clean with `--deny warnings`.
- No `unwrap()`/`expect()`/panic-prone indexing outside tests; guard slice/byte access. No `let _ =` on fallible ops — propagate with `?` or `.log_err()`.
- Crate lib roots named after the crate: `[lib] path = "src/docker_client.rs"` / `src/docker_ui.rs`. Never create `mod.rs`.
- GPUI timers use `cx.background_executor().timer(duration)`, never `smol::Timer::after`.
- Transport is the `docker` CLI with `--format json`; remote endpoints set `DOCKER_HOST=ssh://user@host`. No Docker Engine API / bollard, no TLS/socket plumbing. No secrets stored (SSH auth comes from the user's ssh-agent/`~/.ssh/config`).
- `Panel::activation_priority()` must be a value not already used by another Left-dock panel (DatabasePanel uses 4; pick e.g. 8 and confirm no collision).
- Commit trailer on every commit: `Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>`.
- For any cargo command in a fresh shell, prefix with `source "$HOME/.cargo/env"`.

## File Structure

**New crate `docker_client`** (pure, no gpui):
- `crates/docker_client/Cargo.toml`
- `crates/docker_client/src/docker_client.rs` — types (`DockerEndpoint`, `EndpointKind`, `Container`, `ContainerState`, `Image`, `ComposeProject`, `ComposeService`, `LogChunk`), the `DockerClient` trait, `docker_host_for`, command/arg helpers, and re-exports. Tests in-file.
- `crates/docker_client/src/parse.rs` — pure parsers of `--format json` output.
- `crates/docker_client/src/cli.rs` — `CliDockerClient` (real shell-out).
- `crates/docker_client/src/fake.rs` — `FakeDockerClient` (test-support).

**New crate `docker_ui`** (GPUI):
- `crates/docker_ui/Cargo.toml`
- `crates/docker_ui/src/docker_ui.rs` — module decls, re-exports, `pub fn init(cx: &mut App)`.
- `crates/docker_ui/src/docker_settings.rs` — `DockerSettings` (`RegisterSetting`).
- `crates/docker_ui/src/endpoint_store.rs` — `DockerEndpointStore` (endpoints from settings, connect/test, load containers/images/compose, ClientFactory).
- `crates/docker_ui/src/docker_panel.rs` — `DockerPanel` (`impl Panel`, tree, actions, load/new).
- `crates/docker_ui/src/detail_view.rs` — container/image/compose detail + actions + read-only gate.
- `crates/docker_ui/src/confirm_modal.rs` — confirm modal for destructive actions.
- `crates/docker_ui/src/logs_view.rs` — streaming logs view.

**Modified:**
- `Cargo.toml` (root) — add both crates to `[workspace] members` and `[workspace.dependencies]`.
- `crates/settings_content/src/docker.rs` (new) + `crates/settings_content/src/settings_content.rs` (`mod docker; pub use docker::*;` + `pub docker: Option<DockerSettingsContent>` field).
- `assets/settings/default.json` — `"docker": { "connections": [], "poll_interval_seconds": 5 }`.
- `crates/zed/src/main.rs` and `crates/zed/src/zed.rs` — `docker_ui::init(cx);` calls + `DockerPanel::load` in `initialize_panels`.
- `docs/superpowers/database-viewer-usage.md` or a new `docs/superpowers/docker-panel-usage.md`.

Reference siblings to mirror for structure (read them; do not reinvent): `crates/database_client/src/{database_client.rs,fake.rs}`, `crates/database_ui/src/{database_ui.rs,database_settings.rs,connection_store.rs,database_panel.rs}`, `crates/settings_content/src/database.rs`.

---

### Task 1: `docker_client` crate foundation — types + host/arg helpers

**Files:**
- Create: `crates/docker_client/Cargo.toml`, `crates/docker_client/src/docker_client.rs`
- Modify: `Cargo.toml` (root) — `[workspace] members` + `[workspace.dependencies]`

**Interfaces — Produces:**
```rust
pub enum EndpointKind { Local, Ssh { host: String } }           // host = "user@host"
pub struct DockerEndpoint { pub name: String, pub kind: EndpointKind, pub read_only: bool }
pub enum ContainerState { Running, Exited, Paused, Created, Restarting, Dead, Unknown }
pub struct Container { pub id: String, pub names: String, pub image: String,
                       pub state: ContainerState, pub status: String, pub ports: String }
pub struct Image { pub id: String, pub repository: String, pub tag: String, pub size: String, pub created: String }
pub struct ComposeProject { pub name: String, pub status: String, pub config_files: String }
pub struct ComposeService { pub name: String, pub state: String, pub project: String }
pub struct LogChunk { pub line: String }
pub fn docker_host_for(endpoint: &DockerEndpoint) -> Option<String>; // Ssh -> Some("ssh://user@host"), Local -> None
```
Derive `Debug, Clone, PartialEq, Eq` on all; `Serialize, Deserialize` on `DockerEndpoint`/`EndpointKind`; `Copy` on `ContainerState`.

- [ ] **Step 1: Create `crates/docker_client/Cargo.toml`** (mirror `database_client/Cargo.toml`)
```toml
[package]
name = "docker_client"
version = "0.1.0"
edition.workspace = true
publish.workspace = true
license = "GPL-3.0-or-later"

[lib]
path = "src/docker_client.rs"
doctest = false

[features]
test-support = []

[dependencies]
anyhow.workspace = true
async-trait.workspace = true
futures.workspace = true
log.workspace = true
serde.workspace = true
serde_json.workspace = true
tokio = { workspace = true, features = ["process", "io-util", "rt", "macros"] }

[dev-dependencies]
tokio = { workspace = true, features = ["rt", "macros"] }

[lints]
workspace = true
```

- [ ] **Step 2: Add to root `Cargo.toml`** — insert `"crates/docker_client",` in `[workspace] members` (alphabetical, after the `crates/dev_*` group) and `docker_client = { path = "crates/docker_client" }` in `[workspace.dependencies]` (alphabetical).

- [ ] **Step 3: Write the failing test** in `crates/docker_client/src/docker_client.rs` (`#[cfg(test)] mod tests`)
```rust
#[test]
fn docker_host_for_ssh_and_local() {
    let local = DockerEndpoint { name: "local".into(), kind: EndpointKind::Local, read_only: false };
    let remote = DockerEndpoint { name: "prod".into(), kind: EndpointKind::Ssh { host: "deploy@1.2.3.4".into() }, read_only: true };
    assert_eq!(docker_host_for(&local), None);
    assert_eq!(docker_host_for(&remote), Some("ssh://deploy@1.2.3.4".to_string()));
}
```

- [ ] **Step 4: Run it, expect fail:** `source "$HOME/.cargo/env" && cargo test -p docker_client docker_host_for` → FAIL (types/fn missing).

- [ ] **Step 5: Implement the types + `docker_host_for`** in `docker_client.rs` (with the derives above and the module declarations `pub mod parse; #[cfg(any(test, feature = "test-support"))] pub mod fake; pub mod cli;` — create empty `parse.rs`/`cli.rs`/`fake.rs` stubs so it compiles, or add those `mod`s in later tasks; for this task only declare `mod`s that exist). `docker_host_for` per the interface above.

- [ ] **Step 6: Run tests, expect pass.** Then `cargo fmt -p docker_client` and `./script/clippy -p docker_client`.

- [ ] **Step 7: Commit** — `git add crates/docker_client Cargo.toml && git commit -m "docker_client: Add crate skeleton, endpoint types, and DOCKER_HOST helper"` (+ trailer).

---

### Task 2: Parsers for `docker --format json` output

**Files:**
- Create: `crates/docker_client/src/parse.rs`
- Modify: `crates/docker_client/src/docker_client.rs` (`pub mod parse;`)

**Interfaces — Produces:**
```rust
pub fn parse_containers(stdout: &str) -> anyhow::Result<Vec<Container>>;
pub fn parse_images(stdout: &str) -> anyhow::Result<Vec<Image>>;
pub fn parse_compose_projects(stdout: &str) -> anyhow::Result<Vec<ComposeProject>>;
pub fn parse_compose_services(stdout: &str) -> anyhow::Result<Vec<ComposeService>>;
pub fn parse_container_state(raw: &str) -> ContainerState; // "running"->Running, etc., default Unknown
```
`docker` `--format json` emits **JSON-lines** (one object per line). Each parser splits on lines, skips blank lines, `serde_json::from_str` each into a private `#[derive(Deserialize)]` row struct with `#[serde(rename = "...")]` for Docker's PascalCase keys (`ID`, `Names`, `Image`, `State`, `Status`, `Ports`, `Repository`, `Tag`, `Size`, `CreatedSince`, `Name`, `ConfigFiles`, ...), then maps to the public type. Unknown fields are ignored (default serde behavior). An empty/whitespace stdout yields an empty Vec.

- [ ] **Step 1: Write failing tests** in `parse.rs`
```rust
#[test]
fn parse_containers_jsonlines() {
    let out = concat!(
        r#"{"ID":"abc123","Names":"api","Image":"myapi:latest","State":"running","Status":"Up 3 hours","Ports":"0.0.0.0:8080->8080/tcp"}"#, "\n",
        r#"{"ID":"def456","Names":"db","Image":"postgres:16","State":"exited","Status":"Exited (0) 1 hour ago","Ports":""}"#, "\n",
    );
    let containers = parse_containers(out).unwrap();
    assert_eq!(containers.len(), 2);
    assert_eq!(containers[0].names, "api");
    assert_eq!(containers[0].state, ContainerState::Running);
    assert_eq!(containers[1].state, ContainerState::Exited);
}

#[test]
fn parse_containers_empty_is_empty_vec() {
    assert!(parse_containers("").unwrap().is_empty());
    assert!(parse_containers("   \n\n").unwrap().is_empty());
}

#[test]
fn parse_images_and_compose() {
    let imgs = parse_images(r#"{"ID":"sha256:aaa","Repository":"myapi","Tag":"latest","Size":"120MB","CreatedSince":"2 days ago"}"#).unwrap();
    assert_eq!(imgs[0].repository, "myapi");
    assert_eq!(imgs[0].tag, "latest");
    let projs = parse_compose_projects(r#"{"Name":"shop","Status":"running(3)","ConfigFiles":"/app/docker-compose.yml"}"#).unwrap();
    assert_eq!(projs[0].name, "shop");
    let svcs = parse_compose_services(r#"{"Name":"web","State":"running","Project":"shop"}"#).unwrap();
    assert_eq!(svcs[0].state, "running");
}
```

- [ ] **Step 2: Run, expect fail** — `cargo test -p docker_client parse_`.
- [ ] **Step 3: Implement** the parsers + row structs + `parse_container_state`. Make each parser tolerant: a line that fails to parse is skipped with `log::warn!` (do not fail the whole batch on one bad line) — but a completely malformed non-JSON stdout still returns `Ok(Vec::new())` rather than erroring, since Docker versions vary.
- [ ] **Step 4: Run, expect pass.** `cargo fmt` + `./script/clippy -p docker_client`.
- [ ] **Step 5: Commit** — `docker_client: Parse docker --format json output for containers, images, compose`.

---

### Task 3: `DockerClient` trait + `FakeDockerClient`

**Files:**
- Modify: `crates/docker_client/src/docker_client.rs` (trait + re-export)
- Create: `crates/docker_client/src/fake.rs`

**Interfaces — Produces:**
```rust
#[async_trait::async_trait]
pub trait DockerClient: Send + Sync {
    async fn test_endpoint(&self, endpoint: &DockerEndpoint) -> Result<()>;                 // `docker version`
    async fn list_containers(&self, endpoint: &DockerEndpoint) -> Result<Vec<Container>>;
    async fn list_images(&self, endpoint: &DockerEndpoint) -> Result<Vec<Image>>;
    async fn list_compose_projects(&self, endpoint: &DockerEndpoint) -> Result<Vec<ComposeProject>>;
    async fn list_compose_services(&self, endpoint: &DockerEndpoint, project: &str) -> Result<Vec<ComposeService>>;
    async fn inspect_container(&self, endpoint: &DockerEndpoint, id: &str) -> Result<String>; // pretty JSON
    async fn start_container(&self, endpoint: &DockerEndpoint, id: &str) -> Result<()>;
    async fn stop_container(&self, endpoint: &DockerEndpoint, id: &str) -> Result<()>;
    async fn restart_container(&self, endpoint: &DockerEndpoint, id: &str) -> Result<()>;
    async fn pull_image(&self, endpoint: &DockerEndpoint, reference: &str) -> Result<()>;
    async fn remove_image(&self, endpoint: &DockerEndpoint, id: &str) -> Result<()>;
    async fn compose_up(&self, endpoint: &DockerEndpoint, project: &str, service: Option<&str>) -> Result<()>;
    async fn compose_down(&self, endpoint: &DockerEndpoint, project: &str) -> Result<()>;
    async fn compose_restart(&self, endpoint: &DockerEndpoint, project: &str, service: Option<&str>) -> Result<()>;
    async fn container_logs(&self, endpoint: &DockerEndpoint, id: &str, tail: usize)
        -> Result<futures::channel::mpsc::UnboundedReceiver<LogChunk>>;
}
```
`FakeDockerClient` mirrors `FakeDatabaseClient` exactly: `pub` fields for canned data (`containers: Vec<Container>`, `images`, `compose_projects`, `compose_services`, `inspect: String`, `log_lines: Vec<String>`), a blanket `error: Option<String>`, per-method `Mutex<Option<String>>` overrides with `set_*_error(&self, Option<String>)` setters, a `calls: Mutex<Vec<String>>` with `pub fn calls(&self) -> Vec<String>` and private `record(&self, impl Into<String>)`, `pub fn new() -> Self`, `pub fn with_error(&str) -> Self`. Each method: `check_error()?` → `record(format!("<method> <endpoint.name> ..."))` → return canned data. `container_logs` builds an unbounded channel, sends one `LogChunk` per `self.log_lines`, drops the sender (closing the stream), and returns the receiver.

- [ ] **Step 1: Write failing test** in `fake.rs`
```rust
#[tokio::test]
async fn fake_lists_and_records_calls() {
    let mut fake = FakeDockerClient::new();
    fake.containers = vec![Container { id: "a".into(), names: "api".into(), image: "img".into(),
        state: ContainerState::Running, status: "Up".into(), ports: "".into() }];
    let ep = DockerEndpoint { name: "local".into(), kind: EndpointKind::Local, read_only: false };
    let got = fake.list_containers(&ep).await.unwrap();
    assert_eq!(got.len(), 1);
    assert!(fake.calls().iter().any(|c| c.starts_with("list_containers local")));
}

#[tokio::test]
async fn fake_error_override_propagates() {
    let fake = FakeDockerClient::new();
    fake.set_stop_container_error(Some("boom".into()));
    let ep = DockerEndpoint { name: "local".into(), kind: EndpointKind::Local, read_only: false };
    assert!(fake.stop_container(&ep, "a").await.is_err());
}

#[tokio::test]
async fn fake_logs_stream_yields_canned_lines() {
    use futures::StreamExt as _;
    let mut fake = FakeDockerClient::new();
    fake.log_lines = vec!["line1".into(), "line2".into()];
    let ep = DockerEndpoint { name: "local".into(), kind: EndpointKind::Local, read_only: false };
    let mut rx = fake.container_logs(&ep, "a", 100).await.unwrap();
    let mut lines = vec![];
    while let Some(chunk) = rx.next().await { lines.push(chunk.line); }
    assert_eq!(lines, vec!["line1", "line2"]);
}
```

- [ ] **Step 2: Run, expect fail.**
- [ ] **Step 3: Implement** the trait in `docker_client.rs` and `FakeDockerClient` in `fake.rs` (gate `pub mod fake;` with `#[cfg(any(test, feature = "test-support"))]`). Re-export nothing special beyond the types.
- [ ] **Step 4: Run, expect pass.** fmt + clippy.
- [ ] **Step 5: Commit** — `docker_client: Add DockerClient trait and FakeDockerClient`.

---

### Task 4: `CliDockerClient` — real shell-out (non-streaming methods)

**Files:**
- Create: `crates/docker_client/src/cli.rs`
- Modify: `crates/docker_client/src/docker_client.rs` (`pub mod cli;`, `pub use cli::CliDockerClient;`)

**Interfaces — Produces:** `pub struct CliDockerClient;` implementing `DockerClient` (all methods except the streaming detail of `container_logs`, which lands in Task 5 — for this task `container_logs` may `bail!("not implemented")` and be finished in Task 5).

**Implementation notes (put in the steps as code):**
- Private helper builds the command:
```rust
fn command(endpoint: &DockerEndpoint, args: &[&str]) -> tokio::process::Command {
    let mut cmd = tokio::process::Command::new("docker");
    if let Some(host) = docker_host_for(endpoint) { cmd.env("DOCKER_HOST", host); }
    cmd.args(args);
    cmd.kill_on_drop(true);
    cmd
}
async fn run(endpoint: &DockerEndpoint, args: &[&str]) -> Result<String> {
    let output = command(endpoint, args).output().await
        .with_context(|| format!("running `docker {}`", args.join(" ")))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("docker {} failed: {}", args.join(" "), stderr.trim());
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}
```
- Methods map to args + parser: `list_containers` → `run(ep, &["ps","-a","--format","json"])` then `parse::parse_containers`. `list_images` → `["images","--format","json"]`. `list_compose_projects` → `["compose","ls","--all","--format","json"]`. `list_compose_services` → `["compose","-p",project,"ps","--format","json"]`. `inspect_container` → `["inspect", id]` returned as-is (already pretty JSON array). `start/stop/restart_container` → `["start"|"stop"|"restart", id]`, ignore stdout. `pull_image` → `["pull", reference]`. `remove_image` → `["rmi", id]`. `compose_up` → `["compose","-p",project,"up","-d", (service?)]`; `compose_down` → `["compose","-p",project,"down"]`; `compose_restart` → `["compose","-p",project,"restart",(service?)]`. `test_endpoint` → `["version","--format","json"]`.

- [ ] **Step 1: Write live tests** (gated `#[ignore]`, need local Docker) in `cli.rs`
```rust
// Run manually with: cargo test -p docker_client --features test-support -- --ignored --test-threads=1
#[tokio::test]
#[ignore]
async fn cli_lists_local_containers() {
    let client = CliDockerClient;
    let ep = DockerEndpoint { name: "local".into(), kind: EndpointKind::Local, read_only: false };
    let containers = client.list_containers(&ep).await.unwrap();
    // At least the zed-db-test postgres container is expected to be present when run.
    assert!(containers.iter().any(|c| c.image.contains("postgres")));
}
```
(Also add ignored tests for `list_images` and a `restart_container` round-trip against a known throwaway container.)

- [ ] **Step 2: Run non-ignored suite** `cargo test -p docker_client` (ignored ones skipped) → the crate compiles and existing tests pass.
- [ ] **Step 3: Implement** `CliDockerClient` per notes.
- [ ] **Step 4: Verify** `cargo test -p docker_client` passes (unit + fake). Optionally run `-- --ignored` locally against Docker and record output. fmt + clippy.
- [ ] **Step 5: Commit** — `docker_client: Add CliDockerClient shelling out to docker --format json`.

---

### Task 5: Log streaming (`container_logs`) in `CliDockerClient`

**Files:** Modify `crates/docker_client/src/cli.rs`.

**Interfaces — Consumes/Produces:** finishes `CliDockerClient::container_logs` to return a live `UnboundedReceiver<LogChunk>`.

**Implementation:** spawn `docker logs -f --tail {tail} {id}` with stdout piped; spawn a tokio task that reads lines and forwards them; return the receiver. Because this async fn is awaited inside a `gpui_tokio::Tokio::spawn_result` (tokio runtime), `tokio::spawn` here is valid.
```rust
async fn container_logs(&self, endpoint: &DockerEndpoint, id: &str, tail: usize)
    -> Result<futures::channel::mpsc::UnboundedReceiver<LogChunk>>
{
    use tokio::io::{AsyncBufReadExt as _, BufReader};
    let tail = tail.to_string();
    let mut cmd = command(endpoint, &["logs", "-f", "--tail", &tail, id]);
    cmd.stdout(std::process::Stdio::piped());
    cmd.stderr(std::process::Stdio::piped());
    let mut child = cmd.spawn().context("spawning `docker logs -f`")?;
    let stdout = child.stdout.take().context("capturing docker logs stdout")?;
    let (tx, rx) = futures::channel::mpsc::unbounded();
    tokio::spawn(async move {
        let _child = child; // kept alive (kill_on_drop) until this task ends
        let mut lines = BufReader::new(stdout).lines();
        loop {
            match lines.next_line().await {
                Ok(Some(line)) => { if tx.unbounded_send(LogChunk { line }).is_err() { break; } }
                Ok(None) => break,
                Err(error) => { log::warn!("docker logs read error: {error}"); break; }
            }
        }
    });
    Ok(rx)
}
```

- [ ] **Step 1: Write ignored live test** streaming logs from a running container, asserting at least one non-empty line arrives within a bounded number of reads; drop `rx` and assert the child is cleaned up (best-effort).
- [ ] **Step 2: Run** `cargo test -p docker_client` (compiles; ignored skipped).
- [ ] **Step 3: Implement** as above.
- [ ] **Step 4: Verify** compile + fake log test still green; optionally run the ignored test against Docker. fmt + clippy.
- [ ] **Step 5: Commit** — `docker_client: Stream container logs via docker logs -f`.

---

### Task 6: Settings content — `DockerSettingsContent` + `DockerConnectionContent`

**Files:**
- Create: `crates/settings_content/src/docker.rs`
- Modify: `crates/settings_content/src/settings_content.rs` (`mod docker; pub use docker::*;` + root field `pub docker: Option<DockerSettingsContent>,`), `assets/settings/default.json`.

**Interfaces — Produces:** (mirror `database.rs`)
```rust
#[with_fallible_options]
#[derive(Clone, PartialEq, Default, Serialize, Deserialize, JsonSchema, MergeFrom, Debug)]
pub struct DockerSettingsContent {
    /// Seconds between automatic status refreshes. Default: 5
    pub poll_interval_seconds: Option<u64>,
    /// Configured Docker endpoints. Default: []
    pub connections: Option<Vec<DockerConnectionContent>>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom, Debug)]
pub struct DockerConnectionContent {
    /// Unique display name of the endpoint.
    pub name: String,
    /// How to reach the daemon: "local" (default socket) or "ssh".
    pub kind: DockerEndpointKindContent,
    /// For kind = "ssh": the SSH target `user@host` (used as DOCKER_HOST=ssh://user@host).
    pub ssh_host: Option<String>,
    /// When true, destructive actions (stop/restart/remove/compose down) are blocked. Default: false
    pub read_only: Option<bool>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom, Debug)]
#[serde(rename_all = "snake_case")]
pub enum DockerEndpointKindContent { Local, Ssh }
```

- [ ] **Step 1: Write failing test** in `docker.rs` (serde round-trip; settings_content already builds without gpui):
```rust
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn docker_connection_content_roundtrips() {
        let json = r#"{"name":"prod","kind":"ssh","ssh_host":"deploy@1.2.3.4","read_only":true}"#;
        let parsed: DockerConnectionContent = serde_json::from_str(json).unwrap();
        assert_eq!(parsed.name, "prod");
        assert!(matches!(parsed.kind, DockerEndpointKindContent::Ssh));
        assert_eq!(parsed.ssh_host.as_deref(), Some("deploy@1.2.3.4"));
        assert_eq!(parsed.read_only, Some(true));
    }
}
```
(If `settings_content` has no `serde_json` dev-dep, add it, mirroring existing tests there; otherwise use the crate's existing test convention.)

- [ ] **Step 2: Run, expect fail** — `cargo test -p settings_content docker_connection`.
- [ ] **Step 3: Implement** `docker.rs`; wire `mod docker; pub use docker::*;` and add `pub docker: Option<DockerSettingsContent>,` to the root `SettingsContent` struct (alphabetical, near `database`). Add to `assets/settings/default.json`:
```json
  "docker": {
    // Seconds between automatic status refreshes.
    "poll_interval_seconds": 5,
    // Docker endpoints (SSH auth comes from your ssh-agent/~/.ssh/config).
    "connections": []
  },
```
- [ ] **Step 4: Run, expect pass.** `cargo check -p settings_content`, fmt, `./script/clippy -p settings_content`.
- [ ] **Step 5: Commit** — `settings_content: Add Docker panel settings and endpoint config`.

---

### Task 7: `docker_ui` crate skeleton + `DockerSettings` + init wiring

**Files:**
- Create: `crates/docker_ui/Cargo.toml`, `crates/docker_ui/src/docker_ui.rs`, `crates/docker_ui/src/docker_settings.rs`
- Modify: root `Cargo.toml` (workspace member + dep), `crates/zed/src/main.rs` (`docker_ui::init(cx);` beside `database_ui::init(cx);`), `crates/zed/src/zed.rs` (the second `database_ui::init(cx);` site).

**Interfaces — Produces:**
```rust
// docker_settings.rs
#[derive(Debug, Clone, PartialEq, RegisterSetting)]
pub struct DockerSettings {
    pub poll_interval_seconds: u64,
    pub endpoints: Vec<docker_client::DockerEndpoint>,
}
impl settings::Settings for DockerSettings { fn from_settings(content: &settings::SettingsContent) -> Self { /* map DockerConnectionContent -> DockerEndpoint; kind Local/Ssh{host}; read_only default false; a Local default endpoint is NOT auto-added here (endpoint store adds it) */ } }
// docker_ui.rs
pub fn init(cx: &mut App); // mirror database_ui::init: register Toggle/ToggleFocus workspace actions for DockerPanel (DockerPanel type lands in Task 9; for Task 7, init can be a stub that will be filled when the panel exists — but to keep it compiling, define the actions! and the two register_action closures referencing DockerPanel only once Task 9 exists). For Task 7, `init` may be empty and gain the action wiring in Task 9.
```
Cargo.toml mirrors `database_ui/Cargo.toml` but **drop** `editor`, `multi_buffer`, `ui_input`, `language`, `zed_credentials_provider` (not needed — no SQL editor, no passwords), and **add** `docker_client.workspace = true`, `serde_json.workspace = true`. Keep `anyhow, futures, fs, gpui, gpui_tokio, log, menu, settings, theme, ui, util, workspace`. Dev-deps mirror the DB crate (`docker_client` with `test-support`, `gpui`/`settings`/`theme` test-support, `gpui_tokio`, `theme_settings`).

- [ ] **Step 1: Write failing test** in `docker_ui.rs` (mirror `database_settings_resolve_from_defaults`)
```rust
#[gpui::test]
fn docker_settings_resolve_from_defaults(cx: &mut TestAppContext) {
    init_test(cx); // sets SettingsStore::test, theme_settings, gpui_tokio, crate::init
    cx.update(|cx| {
        let settings = DockerSettings::get_global(cx);
        assert_eq!(settings.poll_interval_seconds, 5);
        assert!(settings.endpoints.is_empty());
    });
}
```
- [ ] **Step 2: Run, expect fail** — `cargo test -p docker_ui docker_settings_resolve --features gpui_platform/runtime_shaders` (or plain if it links without it; use the feature if the build requires GPUI platform).
- [ ] **Step 3: Implement** the Cargo.toml, `docker_ui.rs` (module decls that exist so far: `mod docker_settings; pub use docker_settings::DockerSettings;` + `pub fn init` empty for now), `docker_settings.rs`, the `init_test` helper (mirror database_ui's), add workspace member + dep, and add `docker_ui::init(cx);` at both zed call sites.
- [ ] **Step 4: Run, expect pass.** `cargo check -p zed --features gpui_platform/runtime_shaders` (ensures the zed wiring compiles). fmt + clippy.
- [ ] **Step 5: Commit** — `docker_ui: Add crate skeleton, DockerSettings, and init wiring`.

---

### Task 8: `DockerEndpointStore` — endpoints, connect/test, load data

**Files:** Create `crates/docker_ui/src/endpoint_store.rs`; modify `docker_ui.rs` (`mod endpoint_store; pub use endpoint_store::*;`).

**Interfaces — Produces:** (mirror `ConnectionStore`)
```rust
pub type ClientFactory = std::sync::Arc<dyn Fn() -> std::sync::Arc<dyn docker_client::DockerClient> + Send + Sync>;
pub fn default_client_factory() -> ClientFactory; // returns Arc::new(|| Arc::new(CliDockerClient))
pub enum EndpointStatus { Idle, Connecting, Connected, Error(String) }
pub struct EndpointState {
    pub endpoint: docker_client::DockerEndpoint,
    pub status: EndpointStatus,
    pub containers: Option<Vec<docker_client::Container>>,
    pub images: Option<Vec<docker_client::Image>>,
    pub compose: Option<Vec<docker_client::ComposeProject>>,
}
pub struct DockerEndpointStore { /* endpoints: Vec<EndpointState>, client_factory, _settings_subscription */ }
impl DockerEndpointStore {
    pub fn new(client_factory: ClientFactory, cx: &mut Context<Self>) -> Self;   // reads DockerSettings::get_global(cx).endpoints; prepends a Local default endpoint named "local" if none present
    pub fn endpoints(&self) -> &[EndpointState];
    pub fn refresh(&mut self, endpoint_name: &str, cx: &mut Context<Self>);      // spawns test_endpoint + list_* via gpui_tokio, writes results back
    pub fn refresh_all(&mut self, cx: &mut Context<Self>);                       // refresh every non-Error endpoint
}
```
Emits an event (`EndpointStoreEvent`) on change and calls `cx.notify()`. Loading uses the exact `gpui_tokio::Tokio::spawn_result(cx, async move { client.list_containers(&ep).await }) ` + `cx.spawn(async move |this, cx| { let r = task.await; this.update(cx, |this, cx| { /* apply */ cx.notify() }).ok(); })` pattern from `connection_store.rs`. Re-sync on `cx.observe_global::<SettingsStore>` diffing by endpoint name.

**Autopolling:** `new` starts a recurring poll task stored in a `_poll_task: Task<()>` field. The loop reads `DockerSettings::get_global(cx).poll_interval_seconds` (fresh each tick so a settings change takes effect), sleeps via `cx.background_executor().timer(Duration::from_secs(interval))` (per the GPUI-timer global constraint — never `smol::Timer`), then calls `refresh_all`. Remote (`Ssh`) endpoints are polled at most every `max(interval, 15s)` to avoid hammering SSH; track a per-endpoint "last polled" instant. The poll loop must not run while an endpoint is mid-load (skip endpoints already `Connecting`).

- [ ] **Step 1: Write failing GPUI tests** (mirror `connect_populates_databases_from_client`, with the `init_test`/`set_one_endpoint`/`wait_until` helpers):
```rust
#[gpui::test]
async fn refresh_populates_containers_from_client(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();
    set_one_ssh_endpoint(cx); // name "prod", ssh, read_only true
    let fake = Arc::new(FakeDockerClient::new_with_container("api"));
    let factory: ClientFactory = Arc::new(move || fake.clone() as Arc<dyn DockerClient>);
    let store = cx.new(|cx| DockerEndpointStore::new(factory, cx));
    store.update(cx, |s, cx| s.refresh("prod", cx));
    wait_until(cx, |cx| store.read_with(cx, |s, _| {
        s.endpoints().iter().find(|e| e.endpoint.name == "prod")
            .and_then(|e| e.containers.as_ref()).map_or(false, |c| c.len() == 1)
    })).await;
}

#[gpui::test]
fn new_store_prepends_local_endpoint(cx: &mut TestAppContext) {
    init_test(cx);
    let factory: ClientFactory = Arc::new(|| Arc::new(FakeDockerClient::new()) as Arc<dyn DockerClient>);
    let store = cx.new(|cx| DockerEndpointStore::new(factory, cx));
    store.read_with(cx, |s, _| assert!(s.endpoints().iter().any(|e| e.endpoint.name == "local")));
}

#[gpui::test]
async fn autopoll_refreshes_after_interval(cx: &mut TestAppContext) {
    init_test(cx); // default.json poll_interval_seconds = 5
    cx.executor().allow_parking();
    let fake = Arc::new(FakeDockerClient::new()); // records calls
    let recorder = fake.clone();
    let factory: ClientFactory = Arc::new(move || recorder.clone() as Arc<dyn DockerClient>);
    let _store = cx.new(|cx| DockerEndpointStore::new(factory, cx));
    let before = fake.calls().iter().filter(|c| c.starts_with("list_containers")).count();
    cx.executor().advance_clock(std::time::Duration::from_secs(6));
    cx.run_until_parked();
    let after = fake.calls().iter().filter(|c| c.starts_with("list_containers")).count();
    assert!(after > before, "autopoll should have refreshed at least once");
}
```
(Note: because `default_client_factory` is not used here, the fake is shared across ticks so its `calls()` accumulates. If the factory constructs a fresh client per call, have the test hold the recording `Arc` directly as above.)
- [ ] **Step 2: Run, expect fail.**
- [ ] **Step 3: Implement** `endpoint_store.rs` mirroring `connection_store.rs` structure and the gpui_tokio load pattern. Add a `FakeDockerClient::new_with_container(name)` test-support constructor if convenient (or set the `containers` field directly in the test).
- [ ] **Step 4: Run, expect pass.** fmt + clippy + `cargo check -p zed --features gpui_platform/runtime_shaders`.
- [ ] **Step 5: Commit** — `docker_ui: Add DockerEndpointStore loading endpoints and container/image/compose data`.

---

### Task 9: `DockerPanel` — dock panel, tree, registration

**Files:** Create `crates/docker_ui/src/docker_panel.rs`; modify `docker_ui.rs` (`mod docker_panel; pub use docker_panel::{DockerPanel, Toggle, ToggleFocus};` + fill `init` with the two `register_action` closures), `crates/zed/src/zed.rs` (`use docker_ui::DockerPanel;` + `let docker_panel = DockerPanel::load(...)` + `add_panel_when_ready(docker_panel, ...)` inside `initialize_panels`).

**Interfaces — Produces:** `DockerPanel` with `pub async fn load(workspace, cx) -> Result<Entity<Self>>`, `fn new(...) -> Entity<Self>` (creates `DockerEndpointStore` via `default_client_factory`, subscribes, `impl Panel` with `persistent_name/panel_key = "DockerPanel"`, `position = Left`, `default_size = px(240.)`, `icon = Some(ui::IconName::…)` (pick a Docker-ish icon such as `Server` or `Box`; confirm the variant exists in `ui::IconName`), `toggle_action = Box::new(ToggleFocus)`, `activation_priority` = an unused value, e.g. `8`). `actions!(docker_panel, [Toggle, ToggleFocus, RefreshEndpoint])`. Tree renders `Endpoint → {Containers, Images, Compose} → items` with status dots, mirroring `database_panel.rs` rendering + `UniformListScrollHandle` + expand/collapse `HashSet`.

- [ ] **Step 1: Write failing GPUI test** — construct the panel with a fake-backed store, expand an endpoint, assert the rendered tree contains a container name. (Mirror any existing `database_panel.rs` render test; if the DB panel tests rendering via reading store state rather than the element tree, follow that same approach — assert on `panel.read_with` state that the tree would show.)
- [ ] **Step 2: Run, expect fail.**
- [ ] **Step 3: Implement** `docker_panel.rs` (mirror `database_panel.rs` structure), fill `init`, and wire `DockerPanel::load` into `initialize_panels` in `zed.rs` alongside the other panels.
- [ ] **Step 4: Run, expect pass.** `cargo check -p zed --features gpui_platform/runtime_shaders`, fmt, clippy.
- [ ] **Step 5: Commit** — `docker_ui: Add DockerPanel dock panel with endpoint/container/image/compose tree`.

---

### Task 10: Detail views + actions + read-only gate + confirm modal

**Files:** Create `crates/docker_ui/src/detail_view.rs`, `crates/docker_ui/src/confirm_modal.rs`; modify `docker_panel.rs` (open detail on selection; dispatch actions) and `docker_ui.rs` (module decls).

**Interfaces — Produces:**
- `ConfirmModal` (a `ManagedView`-style modal, mirror `connection_modal.rs`) that shows the exact command string and target endpoint and emits Confirm/Cancel.
- `DetailView` rendering the selected item with action buttons:
  - Container: Start / Stop / Restart / Logs / Inspect(raw JSON scroll).
  - Image: Pull / Remove.
  - Compose project: Up / Down / Restart (+ per-service).
- **Read-only gate:** a helper `fn is_destructive(action) -> bool` (stop/restart/remove/compose down/up/restart). When `endpoint.read_only`, destructive buttons are rendered disabled with tooltip "endpoint is read-only" and the action handler early-returns. When not read-only, invoking a destructive action opens `ConfirmModal`; only on Confirm does the store call the `DockerClient` method (via gpui_tokio) and then refresh.

- [ ] **Step 1: Write failing GPUI tests**
```rust
#[gpui::test]
async fn destructive_action_blocked_on_read_only_endpoint(cx: &mut TestAppContext) {
    // endpoint read_only = true; invoking stop_container must NOT call the client.
    // Build store with a recording fake; trigger the stop path; assert fake.calls()
    // contains no "stop_container ...".
}
#[gpui::test]
async fn destructive_action_requires_confirmation(cx: &mut TestAppContext) {
    // endpoint read_only = false; invoking restart opens a ConfirmModal and does NOT
    // call the client until confirmed; after confirm, fake.calls() contains "restart_container ...".
}
```
- [ ] **Step 2: Run, expect fail.**
- [ ] **Step 3: Implement** detail view, confirm modal, gate, and the store action methods (`stop_container`/`restart_container`/`remove_image`/`compose_*` that spawn via gpui_tokio and refresh on success, surfacing errors to `EndpointStatus::Error`/a transient notice).
- [ ] **Step 4: Run, expect pass.** check zed + fmt + clippy.
- [ ] **Step 5: Commit** — `docker_ui: Add container/image/compose actions with read-only gate and confirmation`.

---

### Task 11: Logs view (streaming)

**Files:** Create `crates/docker_ui/src/logs_view.rs`; modify `docker_panel.rs`/`detail_view.rs` (open logs) + `docker_ui.rs`.

**Interfaces — Produces:** `LogsView` entity holding a `Vec<String>` buffer and a `follow: bool` toggle. On open, calls `container_logs` via `gpui_tokio::Tokio::spawn_result` to obtain the `UnboundedReceiver<LogChunk>`, then `cx.spawn(async move |this, cx| { while let Some(chunk) = rx.next().await { this.update(cx, |this, cx| { this.push_line(chunk.line); cx.notify(); }).ok(); } })`. Dropping the view drops the task/receiver, which stops the reader (and kills the child via `kill_on_drop`). A scroll view with a follow/pause toggle button.

- [ ] **Step 1: Write failing GPUI test** — inject a fake whose `log_lines = ["a","b"]`, open the logs view, `wait_until` the buffer equals `["a","b"]`.
- [ ] **Step 2: Run, expect fail.**
- [ ] **Step 3: Implement** `logs_view.rs`.
- [ ] **Step 4: Run, expect pass.** check zed + fmt + clippy.
- [ ] **Step 5: Commit** — `docker_ui: Add streaming container logs view`.

---

### Task 12: Auto-import `docker context ls`

**Files:** Modify `crates/docker_client/src/{docker_client.rs,parse.rs,cli.rs}` (add `list_contexts`) and `crates/docker_ui/src/endpoint_store.rs` (merge).

**Interfaces — Produces:**
```rust
// docker_client
pub struct DockerContext { pub name: String, pub docker_endpoint: String } // from `docker context ls --format json` (Name, DockerEndpoint)
pub fn parse_contexts(stdout: &str) -> Result<Vec<DockerContext>>;
// trait: async fn list_contexts(&self) -> Result<Vec<DockerContext>>;  (local invocation, no endpoint)
pub fn merge_endpoints(configured: Vec<DockerEndpoint>, contexts: Vec<DockerContext>) -> Vec<DockerEndpoint>;
// merge rule: configured entries win by name; each context not already present becomes an endpoint:
//   DockerEndpoint parsed from its docker_endpoint string — "ssh://user@host" -> Ssh{host}, unix socket / "default" -> Local; read_only defaults to false.
```

- [ ] **Step 1: Write failing unit test** for `parse_contexts` and `merge_endpoints`
```rust
#[test]
fn merge_prefers_configured_and_imports_new_contexts() {
    let configured = vec![DockerEndpoint { name: "prod".into(), kind: EndpointKind::Ssh{host:"me@h".into()}, read_only: true }];
    let contexts = vec![
        DockerContext { name: "prod".into(), docker_endpoint: "ssh://other@h2".into() },   // ignored (name clash)
        DockerContext { name: "staging".into(), docker_endpoint: "ssh://deploy@stg".into() },
        DockerContext { name: "default".into(), docker_endpoint: "unix:///var/run/docker.sock".into() },
    ];
    let merged = merge_endpoints(configured, contexts);
    let prod = merged.iter().find(|e| e.name == "prod").unwrap();
    assert!(prod.read_only); // configured wins
    assert!(merged.iter().any(|e| e.name == "staging" && matches!(e.kind, EndpointKind::Ssh{..})));
    assert!(merged.iter().any(|e| e.name == "default" && matches!(e.kind, EndpointKind::Local)));
}
```
- [ ] **Step 2: Run, expect fail.**
- [ ] **Step 3: Implement** `parse_contexts`, `merge_endpoints`, `list_contexts`, and call it in `DockerEndpointStore::new`/sync (spawn `list_contexts` and merge into the endpoint list, best-effort — a failure to list contexts must not break the panel; log and fall back to configured + local).
- [ ] **Step 4: Run, expect pass.** check zed + fmt + clippy across both crates.
- [ ] **Step 5: Commit** — `docker_client: Import docker context ls endpoints and merge with configured`.

---

### Task 13: Docs, live e2e, final verification

**Files:** Create `docs/superpowers/docker-panel-usage.md`; no code beyond fixes surfaced by verification.

- [ ] **Step 1:** Write `docs/superpowers/docker-panel-usage.md`: how to add local/ssh endpoints in `settings.json` (`docker.connections`), the `read_only` flag, confirmation behavior, auto-imported contexts, and that SSH auth uses the user's ssh config (no secrets stored). Note v1 scope and that MCP tools are Phase 2.
- [ ] **Step 2:** Full unit/UI suites: `source "$HOME/.cargo/env" && cargo test -p docker_client -p docker_ui -p settings_content 2>&1 | tail -30` — all green.
- [ ] **Step 3:** Build gate: `cargo check -p zed --features gpui_platform/runtime_shaders` — passes.
- [ ] **Step 4:** Lint gate: `./script/clippy -p docker_client -p docker_ui -p settings_content` — clean.
- [ ] **Step 5:** Live smoke (manual, against local Docker with the running `zed-db-test` container): run the `#[ignore]` tests `cargo test -p docker_client --features test-support -- --ignored --test-threads=1` and record output; note anything to fix.
- [ ] **Step 6: Commit** — `docs: Add Docker panel usage guide` (+ any verification fixes squashed into their task's spirit or a `docker_ui: Fix …` commit).

---

## Notes for execution

- Security-sensitive surface is the destructive-action path on remote/prod endpoints: the read-only gate (Task 10) and confirmation must be verified to actually block the `DockerClient` call — the tests assert on `fake.calls()` precisely for this reason.
- The subprocess boundary (Tasks 4–5) can't be unit-tested without Docker; the pure arg-building (Task 1) and parsing (Task 2) carry the deterministic coverage, and the `#[ignore]` live tests carry the integration coverage. Don't add flaky Docker-dependent tests to the default suite.
- Phase 2 (out of scope): `docker_mcp` binary giving the agent read tools (list/logs/inspect) and guarded actions behind the same `read_only` flag + a propose/confirm step, mirroring `database_mcp`.
