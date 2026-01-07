# E2E SSH Remoting Test Implementation Plan

## Overview

The remoting setup in Zed is currently entirely untested at the integration level. The existing tests in `remote_server/src/remote_editing_tests.rs` use a `FakeRemoteConnection` that bypasses all actual SSH mechanics. This document outlines the plan to create end-to-end tests that exercise the real SSH connection flow.

## Goals

Create an end-to-end test that:
1. Spawns a Zed client
2. Connects to a Zed remote server via SSH (localhost)
3. Uploads the pre-built remote server binary
4. Verifies the connection works and initial project state is received

**CI Platform**: Linux → Linux (to avoid cross-compilation complexity)

> **Note**: Tests can be run on any platform locally if SSH to localhost is configured,
> but CI only tests the Linux → Linux scenario, which is the only configuration we
> guarantee to work.

## Current Architecture

### Existing Test Infrastructure

The current tests use:
- `RemoteClient::fake_server()` / `RemoteClient::fake_client()` - in-memory channels
- `FakeRemoteConnection` - bypasses SSH entirely
- `FakeFs` - in-memory filesystem

Location: `crates/remote_server/src/remote_editing_tests.rs`

### Real SSH Connection Flow

1. `SshRemoteConnection::new()` establishes SSH connection via system `ssh` command
2. Uses control sockets on Linux for connection multiplexing
3. `ensure_server_binary()` handles binary provisioning:
   - Checks if binary exists on remote
   - Downloads on server OR uploads locally-built binary
4. Spawns proxy process for communication
5. Platform/shell detection occurs during connection

### Key Environment Variables

- `ZED_BUILD_REMOTE_SERVER` - Controls building from source (values: `false`, `nocompress`, `nomusl`, `mold`)

## Implementation Plan

### Phase 1: Add Pre-built Binary Path Support

#### 1.1 New Environment Variable

Introduce `ZED_REMOTE_SERVER_BINARY` to specify a pre-built binary path for upload.

**File**: `crates/remote/src/transport.rs` (or `ssh.rs`)

```rust
// In ensure_server_binary() or build_remote_server_from_source()
if let Ok(binary_path) = std::env::var("ZED_REMOTE_SERVER_BINARY") {
    let path = PathBuf::from(binary_path);
    if path.exists() {
        return Ok(Some(path));
    } else {
        anyhow::bail!("ZED_REMOTE_SERVER_BINARY points to non-existent file: {}", path.display());
    }
}
```

This allows CI to:
1. Build `remote_server` once in a dedicated step
2. Point tests to the pre-built binary
3. Avoid rebuilding during each test run

### Phase 2: Test Infrastructure

#### 2.1 New Feature Flag

**File**: `crates/remote/Cargo.toml`

```toml
[features]
default = []
test-support = ["fs/test-support"]
e2e-ssh = []  # Enable end-to-end SSH tests
```

#### 2.2 Test Module Structure

**File**: `crates/remote/src/e2e_tests.rs` (new)

```rust
//! End-to-end SSH connection tests.
//!
//! These tests require:
//! - SSH server running on localhost
//! - Passwordless SSH configured for current user
//! - `e2e-ssh` feature enabled
//!
//! Run with: cargo nextest run --package remote --features e2e-ssh

#[cfg(all(test, feature = "e2e-ssh"))]
mod tests {
    // Test implementations
}
```

**File**: `crates/remote/src/remote.rs` (modify)

```rust
// Add at end of file
#[cfg(all(test, feature = "e2e-ssh"))]
mod e2e_tests;
```

#### 2.3 Test Fixture

```rust
pub struct LocalSshTestFixture {
    temp_dir: tempfile::TempDir,
    project_path: PathBuf,
}

impl LocalSshTestFixture {
    pub async fn new() -> Result<Self> {
        // 1. Verify SSH to localhost works
        Self::verify_ssh_available().await?;

        // 2. Create temp directory for test project
        let temp_dir = tempfile::Builder::new()
            .prefix("zed-e2e-ssh-test")
            .tempdir()?;

        let project_path = temp_dir.path().join("test_project");
        std::fs::create_dir_all(&project_path)?;

        Ok(Self { temp_dir, project_path })
    }

    async fn verify_ssh_available() -> Result<()> {
        let output = Command::new("ssh")
            .args(["-o", "BatchMode=yes", "-o", "ConnectTimeout=5", "localhost", "echo", "ok"])
            .output()
            .await?;

        if !output.status.success() {
            anyhow::bail!(
                "SSH to localhost not available. Ensure:\n\
                 1. openssh-server is installed and running\n\
                 2. Passwordless SSH is configured (key in authorized_keys)\n\
                 Error: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
        Ok(())
    }

    pub fn ssh_options(&self) -> SshConnectionOptions {
        SshConnectionOptions {
            host: "localhost".into(),
            username: Some(whoami::username()),
            port: None,
            password: None,
            args: vec![
                "-o".into(), "StrictHostKeyChecking=no".into(),
                "-o".into(), "UserKnownHostsFile=/dev/null".into(),
                "-o".into(), "LogLevel=ERROR".into(),
            ],
            upload_binary_over_ssh: true,
            ..Default::default()
        }
    }

    pub fn project_path(&self) -> &Path {
        &self.project_path
    }
}
```

#### 2.4 Test Delegate

```rust
struct E2eTestDelegate {
    status: Arc<Mutex<Option<String>>>,
}

impl E2eTestDelegate {
    fn new() -> Self {
        Self {
            status: Arc::new(Mutex::new(None)),
        }
    }
}

impl RemoteClientDelegate for E2eTestDelegate {
    fn ask_password(
        &self,
        prompt: String,
        _tx: oneshot::Sender<EncryptedPassword>,
        _cx: &mut AsyncApp,
    ) {
        panic!(
            "Password prompt received: '{}'\n\
             E2E tests require passwordless SSH. Ensure your public key is in ~/.ssh/authorized_keys",
            prompt
        );
    }

    fn download_server_binary_locally(
        &self,
        platform: RemotePlatform,
        _release_channel: ReleaseChannel,
        _version: Option<Version>,
        _cx: &mut AsyncApp,
    ) -> Task<Result<PathBuf>> {
        // Use ZED_REMOTE_SERVER_BINARY if set, otherwise fail
        Task::ready(
            std::env::var("ZED_REMOTE_SERVER_BINARY")
                .map(PathBuf::from)
                .map_err(|_| anyhow::anyhow!(
                    "ZED_REMOTE_SERVER_BINARY not set. Build remote_server first and set this env var."
                ))
        )
    }

    fn get_download_url(
        &self,
        _platform: RemotePlatform,
        _release_channel: ReleaseChannel,
        _version: Option<Version>,
        _cx: &mut AsyncApp,
    ) -> Task<Result<Option<String>>> {
        // Return None to force local upload path
        Task::ready(Ok(None))
    }

    fn set_status(&self, status: Option<&str>, _cx: &mut AsyncApp) {
        *self.status.lock().unwrap() = status.map(String::from);
    }
}
```

### Phase 3: Test Implementation

#### 3.1 Basic Connection Test

```rust
#[gpui::test]
async fn test_e2e_ssh_connection_to_localhost(cx: &mut TestAppContext) {
    // Initialize
    cx.update(|cx| {
        release_channel::init(semver::Version::new(0, 0, 0), cx);
    });

    // Setup fixture - will fail if SSH not available
    let fixture = LocalSshTestFixture::new()
        .await
        .expect("Failed to setup SSH test fixture");

    let delegate = Arc::new(E2eTestDelegate::new());

    // Create real SSH connection
    let connection = SshRemoteConnection::new(
        fixture.ssh_options(),
        delegate.clone(),
        &mut cx.to_async(),
    )
    .await
    .expect("SSH connection to localhost should succeed");

    // Verify platform detection
    assert!(
        matches!(connection.ssh_platform.os, RemoteOs::Linux),
        "Expected Linux platform, got {:?}",
        connection.ssh_platform.os
    );

    // Verify shell detection
    assert!(
        !connection.ssh_shell.is_empty(),
        "Shell should be detected"
    );

    // Verify binary was uploaded
    assert!(
        connection.remote_binary_path.is_some(),
        "Remote binary path should be set after connection"
    );

    // Clean up
    connection.kill().await.ok();
}
```

#### 3.2 Project State Test

```rust
#[gpui::test]
async fn test_e2e_ssh_remote_project_initial_state(
    cx: &mut TestAppContext,
    server_cx: &mut TestAppContext,
) {
    // Initialize release channel
    cx.update(|cx| {
        release_channel::init(semver::Version::new(0, 0, 0), cx);
    });
    server_cx.update(|cx| {
        release_channel::init(semver::Version::new(0, 0, 0), cx);
    });

    // Setup fixture
    let fixture = LocalSshTestFixture::new()
        .await
        .expect("Failed to setup SSH test fixture");

    // Create test files
    let main_rs = fixture.project_path().join("src/main.rs");
    std::fs::create_dir_all(main_rs.parent().unwrap()).unwrap();
    std::fs::write(&main_rs, "fn main() {\n    println!(\"Hello\");\n}").unwrap();
    std::fs::write(fixture.project_path().join("Cargo.toml"), "[package]\nname = \"test\"").unwrap();

    // Connect and create project
    let delegate = Arc::new(E2eTestDelegate::new());
    let ssh_connection = SshRemoteConnection::new(
        fixture.ssh_options(),
        delegate.clone(),
        &mut cx.to_async(),
    )
    .await
    .expect("SSH connection should succeed");

    // Wrap in RemoteClient
    let (_cancel_tx, cancel_rx) = oneshot::channel();
    let remote_client = cx
        .update(|cx| {
            RemoteClient::new(
                ConnectionIdentifier::setup(),
                Arc::new(ssh_connection),
                cancel_rx,
                delegate,
                cx,
            )
        })
        .await
        .expect("RemoteClient creation should succeed")
        .expect("RemoteClient should be Some");

    // Build project with real SSH connection
    let project = build_project_for_e2e(remote_client.clone(), cx);

    // Add worktree
    let (worktree, _) = project
        .update(cx, |project, cx| {
            project.find_or_create_worktree(fixture.project_path(), true, cx)
        })
        .await
        .expect("Should create worktree");

    // Wait for sync
    cx.executor().run_until_parked();

    // Verify project state received
    let worktree_id = worktree.read_with(cx, |wt, _| wt.id());
    worktree.update(cx, |worktree, _| {
        let paths: Vec<_> = worktree.paths().map(|p| p.to_path_buf()).collect();

        assert!(
            paths.iter().any(|p| p.ends_with("Cargo.toml")),
            "Should see Cargo.toml, got: {:?}",
            paths
        );
        assert!(
            paths.iter().any(|p| p.ends_with("main.rs")),
            "Should see main.rs, got: {:?}",
            paths
        );
    });

    // Verify we can open a buffer
    let buffer = project
        .update(cx, |project, cx| {
            project.open_buffer((worktree_id, Path::new("src/main.rs")), cx)
        })
        .await
        .expect("Should open buffer");

    buffer.read_with(cx, |buffer, _| {
        assert!(
            buffer.text().contains("println"),
            "Buffer should contain file contents"
        );
    });

    // Cleanup
    remote_client.update(cx, |client, cx| {
        client.shutdown_processes(cx);
    });
}

fn build_project_for_e2e(
    ssh: Entity<RemoteClient>,
    cx: &mut TestAppContext,
) -> Entity<Project> {
    cx.update(|cx| {
        if !cx.has_global::<SettingsStore>() {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
        }
    });

    let client = cx.update(|cx| {
        Client::new(
            Arc::new(FakeSystemClock::new()),
            FakeHttpClient::with_404_response(),
            cx,
        )
    });

    let node = NodeRuntime::unavailable();
    let user_store = cx.new(|cx| UserStore::new(client.clone(), cx));
    let languages = Arc::new(LanguageRegistry::test(cx.executor()));
    let fs = Arc::new(RealFs::new(Default::default(), None));

    cx.update(|cx| Project::init(&client, cx));
    cx.update(|cx| Project::remote(ssh, client, node, user_store, languages, fs, false, cx))
}
```

### Phase 4: CI Configuration

> **Note**: Zed workflows are generated via `xtask`. The YAML files in `.github/workflows/`
> are generated from Rust code in `tooling/xtask/src/tasks/workflows/`. After creating the
> xtask workflow module, run `cargo xtask workflows` to regenerate the YAML.

#### 4.1 New xtask Workflow Module

**File**: `tooling/xtask/src/tasks/workflows/run_e2e_ssh_tests.rs` (new)

```rust
use gh_workflow::{Concurrency, Event, Job, PullRequest, Push, Run, Step, Workflow};

use super::{
    runners::{self, Platform},
    steps::{self, named, release_job, FluentBuilder, NamedJob, BASH_SHELL},
};

pub(crate) fn run_e2e_ssh_tests() -> Workflow {
    let job = e2e_ssh_tests_job();

    named::workflow()
        .add_event(
            Event::default()
                .push(Push::default().add_branch("main"))
                .pull_request(
                    PullRequest::default()
                        .add_branch("**")
                        .add_path("crates/remote/**")
                        .add_path("crates/remote_server/**")
                        .add_path(".github/workflows/run_e2e_ssh_tests.yml"),
                ),
        )
        .concurrency(
            Concurrency::default()
                .group("${{ github.workflow }}-${{ github.ref }}")
                .cancel_in_progress(true),
        )
        .add_env(("CARGO_TERM_COLOR", "always"))
        .add_env(("RUST_BACKTRACE", 1))
        .add_env(("CARGO_INCREMENTAL", 0))
        .add_job(job.name, job.job)
}

fn e2e_ssh_tests_job() -> NamedJob {
    NamedJob {
        name: "e2e_ssh_tests".to_string(),
        job: release_job(&[])
            .runs_on(runners::LINUX_DEFAULT)
            .add_step(steps::checkout_repo())
            .add_step(steps::setup_cargo_config(Platform::Linux))
            .add_step(steps::cache_rust_dependencies_namespace())
            .add_step(steps::setup_linux())
            .add_step(steps::install_mold())
            .add_step(steps::download_wasi_sdk())
            .add_step(setup_ssh_localhost())
            .add_step(steps::cargo_install_nextest())
            .add_step(build_remote_server())
            .add_step(run_e2e_ssh_tests_step())
            .add_step(steps::cleanup_cargo_config(Platform::Linux))
            .timeout_minutes(30),
    }
}

fn setup_ssh_localhost() -> Step<Run> {
    named::bash("./script/setup-ssh-localhost")
}

fn build_remote_server() -> Step<Run> {
    Step::run("cargo build --package remote_server --features debug-embed --release")
        .name("Build remote_server binary")
        .shell(BASH_SHELL)
}

fn run_e2e_ssh_tests_step() -> Step<Run> {
    Step::run(indoc::indoc! {r#"
        cargo nextest run \
            --package remote \
            --features e2e-ssh \
            -E 'test(/e2e/)' \
            --no-fail-fast
    "#})
    .name("Run E2E SSH tests")
    .shell(BASH_SHELL)
    .env(("ZED_REMOTE_SERVER_BINARY", "${{ github.workspace }}/target/release/remote_server"))
    .env(("RUST_LOG", "remote=debug,remote_server=debug"))
}
```

#### 4.2 Register Workflow in workflows.rs

**File**: `tooling/xtask/src/tasks/workflows.rs` (modify)

Add import:
```rust
mod run_e2e_ssh_tests;
```

Add to workflows array:
```rust
WorkflowFile::zed(run_e2e_ssh_tests::run_e2e_ssh_tests),
```

#### 4.3 Generated Workflow File

After running `cargo xtask workflows`, this will generate:

**File**: `.github/workflows/run_e2e_ssh_tests.yml`

```yaml
# Generated from xtask::workflows::run_e2e_ssh_tests
# Rebuild with `cargo xtask workflows`.
name: run_e2e_ssh_tests

env:
  CARGO_TERM_COLOR: always
  RUST_BACKTRACE: '1'
  CARGO_INCREMENTAL: '0'

on:
  pull_request:
    branches:
      - '**'
    paths:
      - 'crates/remote/**'
      - 'crates/remote_server/**'
      - '.github/workflows/run_e2e_ssh_tests.yml'
  push:
    branches:
      - main

concurrency:
  group: ${{ github.workflow }}-${{ github.ref }}
  cancel-in-progress: true

jobs:
  e2e_ssh_tests:
    if: github.repository_owner == 'zed-industries'
    runs-on: namespace-profile-16x32-ubuntu-2204
    timeout-minutes: 30
    steps:
      - name: Checkout repo
        uses: actions/checkout@v4
        with:
          clean: false

      - name: Setup cargo config
        run: |
          mkdir -p ./../.cargo
          cp ./.cargo/ci-config.toml ./../.cargo/config.toml
        shell: bash -euxo pipefail {0}

      - name: Cache Rust dependencies
        uses: namespacelabs/nscloud-cache-action@v1
        with:
          cache: rust

      - name: Setup Linux dependencies
        run: ./script/linux
        shell: bash -euxo pipefail {0}

      - name: Install mold
        run: ./script/install-mold
        shell: bash -euxo pipefail {0}

      - name: Download WASI SDK
        run: ./script/download-wasi-sdk
        shell: bash -euxo pipefail {0}

      - name: Setup SSH for localhost
        run: ./script/setup-ssh-localhost
        shell: bash -euxo pipefail {0}

      - name: Install cargo-nextest
        uses: taiki-e/install-action@nextest

      - name: Build remote_server binary
        run: cargo build --package remote_server --features debug-embed --release
        shell: bash -euxo pipefail {0}

      - name: Run E2E SSH tests
        run: |
          cargo nextest run \
            --package remote \
            --features e2e-ssh \
            -E 'test(/e2e/)' \
            --no-fail-fast
        env:
          ZED_REMOTE_SERVER_BINARY: ${{ github.workspace }}/target/release/remote_server
          RUST_LOG: remote=debug,remote_server=debug
        shell: bash -euxo pipefail {0}

      - name: Cleanup cargo config
        if: always()
        run: rm -rf ./../.cargo
        shell: bash -euxo pipefail {0}
```

### Phase 5: Helper Scripts

#### 5.1 SSH Setup Script

**File**: `script/setup-ssh-localhost`

```bash
#!/usr/bin/env bash
set -euxo pipefail

# Setup passwordless SSH to localhost for E2E testing
# This script is idempotent and safe to run multiple times

echo "Setting up SSH for localhost testing..."

# Detect OS
OS="$(uname)"
echo "Detected OS: $OS"

if [[ "$OS" == "Linux" ]]; then
    # Install openssh-server if not present
    if ! command -v sshd &> /dev/null; then
        echo "Installing openssh-server..."
        sudo apt-get update
        sudo apt-get install -y openssh-server
    fi

    # Ensure SSH service is running
    echo "Starting SSH service..."
    sudo systemctl start ssh || sudo service ssh start || true
    sudo systemctl enable ssh || true
elif [[ "$OS" == "Darwin" ]]; then
    # macOS: Enable Remote Login in System Preferences or via command line
    echo "On macOS, ensure 'Remote Login' is enabled in System Preferences > Sharing"
    echo "Or run: sudo systemsetup -setremotelogin on"
else
    echo "Warning: Unsupported OS '$OS'. SSH setup may require manual configuration."
fi

# Create .ssh directory if needed
mkdir -p ~/.ssh
chmod 700 ~/.ssh

# Generate SSH key if doesn't exist
if [ ! -f ~/.ssh/id_ed25519 ]; then
    echo "Generating SSH key..."
    ssh-keygen -t ed25519 -f ~/.ssh/id_ed25519 -N "" -C "zed-e2e-test"
fi

# Add to authorized_keys
echo "Configuring authorized_keys..."
cat ~/.ssh/id_ed25519.pub >> ~/.ssh/authorized_keys
# Remove duplicates
sort -u ~/.ssh/authorized_keys -o ~/.ssh/authorized_keys
chmod 600 ~/.ssh/authorized_keys

# Configure SSH client for localhost
echo "Configuring SSH client..."
cat >> ~/.ssh/config << 'EOF'

# Zed E2E Test Configuration
Host localhost 127.0.0.1
    StrictHostKeyChecking no
    UserKnownHostsFile /dev/null
    LogLevel ERROR
    BatchMode yes
EOF
chmod 600 ~/.ssh/config

# Wait a moment for SSH to be ready
sleep 1

# Verify SSH works
echo "Verifying SSH connection..."
if ssh -o ConnectTimeout=5 localhost echo "SSH localhost test successful"; then
    echo "✓ SSH to localhost is configured and working"
else
    echo "✗ SSH to localhost failed"
    echo "Debug info:"
    echo "  - SSH service status:"
    sudo systemctl status ssh || sudo service ssh status || true
    echo "  - authorized_keys:"
    cat ~/.ssh/authorized_keys
    exit 1
fi
```

## Files to Create/Modify

| File | Action | Purpose |
|------|--------|---------|
| `crates/remote/src/e2e_tests.rs` | Create | E2E test implementations |
| `crates/remote/src/remote.rs` | Modify | Add `mod e2e_tests;` |
| `crates/remote/src/transport.rs` | Modify | Add `ZED_REMOTE_SERVER_BINARY` support |
| `crates/remote/Cargo.toml` | Modify | Add `e2e-ssh` feature |
| `tooling/xtask/src/tasks/workflows/run_e2e_ssh_tests.rs` | Create | xtask workflow definition |
| `tooling/xtask/src/tasks/workflows.rs` | Modify | Register new workflow |
| `.github/workflows/run_e2e_ssh_tests.yml` | Generated | CI workflow (via `cargo xtask workflows`) |
| `script/setup-ssh-localhost` | Create | SSH setup helper script |
| `docs/src/development/e2e-ssh-testing-plan.md` | Create | This document |

## Environment Variables

| Variable | Purpose | Example |
|----------|---------|---------|
| `ZED_BUILD_REMOTE_SERVER` | Build from source (existing) | `nocompress` |
| `ZED_REMOTE_SERVER_BINARY` | Path to pre-built binary (new) | `/path/to/remote_server` |
| `RUST_LOG` | Debug logging | `remote=debug` |

## Running Tests Locally

### Linux

```bash
# 1. Setup SSH (one-time)
./script/setup-ssh-localhost

# 2. Build remote_server
cargo build --package remote_server --features debug-embed

# 3. Run E2E tests
ZED_REMOTE_SERVER_BINARY=target/debug/remote_server \
  cargo nextest run --package remote --features e2e-ssh -E 'test(/e2e/)'
```

### macOS

```bash
# 1. Enable Remote Login in System Preferences > Sharing
#    Or: sudo systemsetup -setremotelogin on

# 2. Setup SSH keys for localhost
./script/setup-ssh-localhost

# 3. Build and run (same as Linux)
cargo build --package remote_server --features debug-embed
ZED_REMOTE_SERVER_BINARY=target/debug/remote_server \
  cargo nextest run --package remote --features e2e-ssh -E 'test(/e2e/)'
```

### Windows

Windows requires WSL or a third-party SSH server. This configuration is not officially supported.

## Regenerating CI Workflows

After modifying `tooling/xtask/src/tasks/workflows/run_e2e_ssh_tests.rs`:

```bash
cargo xtask workflows
```

This regenerates `.github/workflows/run_e2e_ssh_tests.yml`.

## Success Criteria

The implementation is complete when:

1. ✓ E2E tests pass in CI (Linux → Linux) on every PR touching `crates/remote/**` or `crates/remote_server/**`
2. ✓ Tests exercise real SSH connection to localhost
3. ✓ Tests verify binary upload mechanism works
4. ✓ Tests verify initial project state is received correctly
5. ✓ CI fails if SSH setup or tests fail (no silent skips)
6. ✓ Tests can be run locally on macOS/Linux (with appropriate SSH setup)

## Future Enhancements

Once the basic infrastructure is in place, consider adding:

1. **Terminal tests**: Verify terminal spawning on remote works
2. **File editing tests**: Test file modifications sync correctly
3. **Reconnection tests**: Test connection recovery after network issues
4. **Multi-worktree tests**: Test projects with multiple remote directories
5. **Performance benchmarks**: Track connection setup time
