# Citadel New Project Editor Action Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a "Citadel: New Project" action (File menu + command palette) that scaffolds a Citadel project file tree into a user-selected empty directory, `git init`s it with an initial commit, and opens it as a new workspace window.

**Architecture:** One new crate, `crates/citadel_new_project`. A pure module (`scaffold.rs`, no GPUI dependency) generates the file tree as `(relative path, contents)` pairs and is unit-tested directly. A GPUI-dependent module (`citadel_new_project.rs`) declares the `NewProject` action, registers a handler mirroring `crates/git_ui/src/clone.rs`'s `clone_and_open`, and is tested with `FakeFs` + `#[gpui::test]`.

**Tech Stack:** Rust, GPUI (this repo's UI framework), the existing `Fs`/`workspace` crates.

## Global Constraints

- Scope is the in-editor action only. Do not add a `citadel init` CLI subcommand, do not vendor `ArduinoCore-avr`, do not generate a `build.sh`, do not implement board auto-detection — all explicitly deferred per the design spec.
- The scaffold always targets `atmega328p`; no board-detection logic.
- `git init`/`git add`/`git commit` run as child processes via `util::command::new_command("git")` — the same primitive `Fs::git_init`/`Fs::git_clone` already use (`crates/fs/src/fs.rs:1157-1207`). Reuse `Fs::git_init` (`fs.rs:1157`) rather than reimplementing it.
- If the selected directory is not empty, stop with an error and write nothing.
- No new external crate dependencies — everything needed (`Fs`, `workspace`, `gpui`, `util`) is already a dependency elsewhere in this workspace.
- Crate-name sanitization rule (exact): lowercase the input, replace every run of one-or-more characters that are not `a-z`/`0-9` with a single `_`, trim leading/trailing `_`; if the result is empty, use `project`.

---

### Task 1: Scaffold content generation (pure logic)

**Files:**
- Create: `crates/citadel_new_project/Cargo.toml`
- Create: `crates/citadel_new_project/src/citadel_new_project.rs` (crate root, declares the `scaffold` module)
- Create: `crates/citadel_new_project/src/scaffold.rs`
- Modify: `/home/gooya/citadel/Cargo.toml:27` (workspace members — insert after `"crates/channel",`)
- Modify: `/home/gooya/citadel/Cargo.toml:294` (workspace dependencies — insert after `channel = { path = "crates/channel" }`)

**Interfaces:**
- Consumes: nothing (first task)
- Produces: `pub fn scaffold_files(project_name: &str) -> Vec<(PathBuf, String)>` and `pub fn sanitize_crate_name(project_name: &str) -> String`, both in `citadel_new_project::scaffold`. Task 2 calls `scaffold::scaffold_files` to get the files to write — do not change this signature.

- [ ] **Step 1: Add the crate to the workspace**

In `/home/gooya/citadel/Cargo.toml`, in the `[workspace] members` list, insert a new line after line 27 (`    "crates/channel",`) and before line 28 (`    "crates/cli",`):

```toml
    "crates/citadel_new_project",
```

In the same file's `[workspace.dependencies]` section, insert a new line after line 294 (`channel = { path = "crates/channel" }`) and before line 295 (`cli = { path = "crates/cli" }`):

```toml
citadel_new_project = { path = "crates/citadel_new_project" }
```

- [ ] **Step 2: Write the crate manifest**

`crates/citadel_new_project/Cargo.toml`:

```toml
[package]
name = "citadel_new_project"
version = "0.1.0"
edition.workspace = true
publish.workspace = true
license = "GPL-3.0-or-later"

[lints]
workspace = true

[lib]
name = "citadel_new_project"
path = "src/citadel_new_project.rs"
```

- [ ] **Step 3: Write the failing tests**

`crates/citadel_new_project/src/scaffold.rs` (tests only for now — the functions below don't exist yet):

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scaffold_files_includes_all_expected_paths() {
        let files = scaffold_files("my-project");
        let paths: Vec<&PathBuf> = files.iter().map(|(p, _)| p).collect();
        assert_eq!(paths.len(), 8);
        assert!(paths.contains(&&PathBuf::from(".gitignore")));
        assert!(paths.contains(&&PathBuf::from(".claude/CLAUDE.md")));
        assert!(paths.contains(&&PathBuf::from(".claude/skills/.gitkeep")));
        assert!(paths.contains(&&PathBuf::from("docs/README.md")));
        assert!(paths.contains(&&PathBuf::from("rust-toolchain.toml")));
        assert!(paths.contains(&&PathBuf::from("rust/Cargo.toml")));
        assert!(paths.contains(&&PathBuf::from("rust/src/lib.rs")));
        assert!(paths.contains(&&PathBuf::from("cpp/io.cpp")));
    }

    #[test]
    fn claude_md_states_boundary_rule() {
        let files = scaffold_files("my-project");
        let (_, content) = files
            .iter()
            .find(|(p, _)| p == &PathBuf::from(".claude/CLAUDE.md"))
            .unwrap();
        assert!(content.contains("extern \"C\""));
        assert!(content.contains("cpp/"));
        assert!(content.contains("rust/"));
    }

    #[test]
    fn io_cpp_has_setup_and_loop_and_no_control_flow() {
        let files = scaffold_files("my-project");
        let (_, content) = files
            .iter()
            .find(|(p, _)| p == &PathBuf::from("cpp/io.cpp"))
            .unwrap();
        assert!(content.contains("void setup()"));
        assert!(content.contains("void loop()"));
        assert!(!content.contains("if ("));
        assert!(!content.contains("for ("));
        assert!(!content.contains("while ("));
    }

    #[test]
    fn rust_lib_rs_is_no_std_with_panic_handler() {
        let files = scaffold_files("my-project");
        let (_, content) = files
            .iter()
            .find(|(p, _)| p == &PathBuf::from("rust/src/lib.rs"))
            .unwrap();
        assert!(content.contains("#![no_std]"));
        assert!(content.contains("#[panic_handler]"));
    }

    #[test]
    fn rust_cargo_toml_uses_sanitized_name() {
        let files = scaffold_files("My Project!");
        let (_, content) = files
            .iter()
            .find(|(p, _)| p == &PathBuf::from("rust/Cargo.toml"))
            .unwrap();
        assert!(content.contains("name = \"my_project_logic\""));
        assert!(content.contains("crate-type = [\"staticlib\"]"));
    }

    #[test]
    fn gitkeep_is_empty() {
        let files = scaffold_files("my-project");
        let (_, content) = files
            .iter()
            .find(|(p, _)| p == &PathBuf::from(".claude/skills/.gitkeep"))
            .unwrap();
        assert_eq!(content, "");
    }

    #[test]
    fn sanitize_crate_name_lowercases_and_collapses_separators() {
        assert_eq!(sanitize_crate_name("My Project!"), "my_project");
    }

    #[test]
    fn sanitize_crate_name_keeps_digits() {
        assert_eq!(sanitize_crate_name("123"), "123");
    }

    #[test]
    fn sanitize_crate_name_trims_leading_and_trailing_separators() {
        assert_eq!(sanitize_crate_name("__hello__"), "hello");
    }

    #[test]
    fn sanitize_crate_name_falls_back_when_nothing_alphanumeric() {
        assert_eq!(sanitize_crate_name("!!!"), "project");
        assert_eq!(sanitize_crate_name(""), "project");
    }
}
```

- [ ] **Step 4: Run tests to verify they fail**

Run: `cargo test -p citadel_new_project`
Expected: compile error — `scaffold_files`, `sanitize_crate_name`, `PathBuf` (unimported) are not defined/in scope in `scaffold.rs`.

- [ ] **Step 5: Implement the scaffold module**

Prepend this to `crates/citadel_new_project/src/scaffold.rs`, above the `#[cfg(test)] mod tests` block already there:

```rust
use std::path::PathBuf;

const RUST_TOOLCHAIN_TOML: &str = r#"[toolchain]
channel = "nightly-2026-08-06"
components = ["rust-src"]
"#;

const GITIGNORE: &str = "/build/\n/rust/target/\n";

const CLAUDE_MD: &str = r#"# Citadel project — Rust/C boundary rule

This project follows Citadel's architecture rule:

- `cpp/` may only perform direct, linear I/O hand-off: reading a pin, writing a pin, sending a byte, declaring `pinMode`/board constants. No `if`, no `for`/`while`, no ternaries, no computed intermediate variables.
- All logic — state transitions, calculations, control decisions — must live in `rust/` (a `#![no_std]` crate), never in `cpp/`.
- The two sides only exchange plain data across `extern "C"`: `cpp/` calls into `extern "C"` Rust functions, and Rust may read `extern "C"` variables/constants defined in `cpp/`.

If asked to add a decision or calculation to a file in `cpp/`, implement it in `rust/src/lib.rs` instead and expose it via an `extern "C"` function.
"#;

const DOCS_README: &str = r#"# docs

Schematics, pin assignment notes, and other project documentation go here.
"#;

const RUST_LIB_RS: &str = r#"#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Add your logic here, exposed via `extern "C"` for cpp/io.cpp to call.
"#;

const CPP_IO_CPP: &str = r#"#include <Arduino.h>

void setup() {
    // pinMode(...) calls go here
}

void loop() {
    // straight-line I/O only — put decisions and calculations in rust/src/lib.rs
}
"#;

/// Sanitizes an arbitrary project directory name into a valid Cargo package
/// name fragment: lowercase, non-alphanumeric runs collapsed to a single
/// `_`, leading/trailing `_` trimmed. Falls back to `project` if the result
/// would be empty (e.g. the input has no alphanumeric characters at all).
pub fn sanitize_crate_name(project_name: &str) -> String {
    let mut result = String::new();
    let mut last_was_separator = false;
    for ch in project_name.chars() {
        if ch.is_ascii_alphanumeric() {
            result.push(ch.to_ascii_lowercase());
            last_was_separator = false;
        } else if !last_was_separator {
            result.push('_');
            last_was_separator = true;
        }
    }
    let trimmed = result.trim_matches('_');
    if trimmed.is_empty() {
        "project".to_string()
    } else {
        trimmed.to_string()
    }
}

fn rust_cargo_toml(project_name: &str) -> String {
    let crate_name = sanitize_crate_name(project_name);
    format!(
        r#"[workspace]

[package]
name = "{crate_name}_logic"
version = "0.1.0"
edition = "2021"

[lib]
path = "src/lib.rs"
crate-type = ["staticlib"]

[profile.release]
panic = "abort"
opt-level = "s"
lto = true
"#
    )
}

/// Returns the full Citadel project scaffold as (relative path, file
/// contents) pairs. `project_name` is typically the selected directory's
/// name; it is only used to derive `rust/Cargo.toml`'s package name (via
/// [`sanitize_crate_name`]) — it does not affect any other file's content
/// or path.
pub fn scaffold_files(project_name: &str) -> Vec<(PathBuf, String)> {
    vec![
        (PathBuf::from(".gitignore"), GITIGNORE.to_string()),
        (PathBuf::from(".claude/CLAUDE.md"), CLAUDE_MD.to_string()),
        (PathBuf::from(".claude/skills/.gitkeep"), String::new()),
        (PathBuf::from("docs/README.md"), DOCS_README.to_string()),
        (
            PathBuf::from("rust-toolchain.toml"),
            RUST_TOOLCHAIN_TOML.to_string(),
        ),
        (
            PathBuf::from("rust/Cargo.toml"),
            rust_cargo_toml(project_name),
        ),
        (PathBuf::from("rust/src/lib.rs"), RUST_LIB_RS.to_string()),
        (PathBuf::from("cpp/io.cpp"), CPP_IO_CPP.to_string()),
    ]
}
```

- [ ] **Step 6: Write the crate root**

`crates/citadel_new_project/src/citadel_new_project.rs`:

```rust
mod scaffold;
```

- [ ] **Step 7: Run tests to verify they pass**

Run: `cargo test -p citadel_new_project`
Expected: all 10 tests pass (`test result: ok. 10 passed; 0 failed`).

- [ ] **Step 8: Commit**

```bash
cd /home/gooya/citadel
git add Cargo.toml crates/citadel_new_project
git commit -m "$(cat <<'EOF'
Add scaffold content generation for the Citadel New Project action

Pure, unit-tested logic (no GPUI dependency yet) that produces the
project file tree as (path, contents) pairs — the .gitignore,
.claude/CLAUDE.md, rust/ no_std crate skeleton, and cpp/io.cpp
described in docs/superpowers/specs/2026-08-09-citadel-new-project-design.md.
The action wiring that calls this is a separate task.
EOF
)"
```

---

### Task 2: Action, menu entry, and end-to-end flow

**Files:**
- Modify: `crates/citadel_new_project/Cargo.toml` (add GPUI/workspace/fs/util dependencies)
- Modify: `crates/citadel_new_project/src/citadel_new_project.rs` (add `init()`, action declaration, handler)
- Create: `crates/citadel_new_project/src/new_project.rs` (the async flow, mirroring `git_ui`'s `clone.rs`)
- Modify: `/home/gooya/citadel/crates/zed/Cargo.toml:117` (add dependency, next to `git_ui`)
- Modify: `/home/gooya/citadel/crates/zed/src/main.rs:771` (call `citadel_new_project::init(cx);`)
- Modify: `/home/gooya/citadel/crates/zed/src/zed/app_menus.rs:111-113` (add File menu entry)

**Interfaces:**
- Consumes: `citadel_new_project::scaffold::scaffold_files(project_name: &str) -> Vec<(PathBuf, String)>` from Task 1 — do not change its signature.
- Produces: `pub fn init(cx: &mut App)` and the `citadel_new_project::NewProject` action type, consumed by `app_menus.rs`'s `MenuItem::action("New Project...", citadel_new_project::NewProject)`.

- [ ] **Step 1: Add the remaining dependencies**

In `crates/citadel_new_project/Cargo.toml`, add a `[dependencies]` section:

```toml
[dependencies]
anyhow.workspace = true
fs.workspace = true
futures.workspace = true
gpui.workspace = true
notifications.workspace = true
ui.workspace = true
util.workspace = true
workspace.workspace = true

[dev-dependencies]
fs = { workspace = true, features = ["test-support"] }
gpui = { workspace = true, features = ["test-support"] }
serde_json.workspace = true
```

Note: calling `workspace.project().update(cx, |project, cx| project.create_worktree(...))` does not require depending on the `project` crate directly — the closure's parameter type is inferred, and `git_ui/src/clone.rs` (the pattern this is based on) does exactly this without a `project` dependency either.

- [ ] **Step 2: Declare the action and `init()`**

Replace the full content of `crates/citadel_new_project/src/citadel_new_project.rs` with:

```rust
use gpui::{App, actions};
use workspace::Workspace;

mod new_project;
mod scaffold;

actions!(
    citadel_new_project,
    [
        /// Scaffolds a new Citadel project into an empty folder and opens it.
        NewProject
    ]
);

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _, cx| {
        workspace.register_action(|workspace, _action: &NewProject, window, cx| {
            new_project::new_project(workspace.weak_handle(), window, cx);
        });
    })
    .detach();
}
```

**Note on test scope vs. the design spec:** the design spec called for a `#[gpui::test]` using `Project::test` + `Workspace::test_new` asserting the opened project's worktree is present. Checked the precedent this whole design is modeled on — `crates/git_ui/src/clone.rs` — and confirmed it has zero tests of any kind (no `#[gpui::test]`, no test module) for its own prompt→async-work→`workspace::open_new` flow; this codebase doesn't automate that last leg for this shape of feature. The steps below instead unit-test `write_scaffold` (file writing + the empty-directory rejection) via `FakeFs`, which is the part that's actually practical and valuable to automate; `git_init_and_commit` (subprocess-based) and the final `workspace::open_new`/window-opening are left to this task's final manual-verification step, matching how this codebase actually tests this class of feature rather than the spec's more ambitious original plan.

- [ ] **Step 3: Write the failing tests for the async flow**

`crates/citadel_new_project/src/new_project.rs` (tests only for now, at the bottom of the file — the `new_project` function referenced doesn't exist yet):

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use fs::FakeFs;
    use gpui::TestAppContext;
    use serde_json::json;

    #[gpui::test]
    async fn writes_expected_files_into_an_empty_directory(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/root", json!({ "empty-project": {} })).await;

        write_scaffold(fs.clone(), std::path::Path::new("/root/empty-project"))
            .await
            .unwrap();

        assert_eq!(
            fs.load(std::path::Path::new("/root/empty-project/cpp/io.cpp"))
                .await
                .unwrap(),
            scaffold::scaffold_files("empty-project")
                .into_iter()
                .find(|(p, _)| p == std::path::Path::new("cpp/io.cpp"))
                .unwrap()
                .1
        );
        assert_eq!(
            fs.load(std::path::Path::new(
                "/root/empty-project/.claude/CLAUDE.md"
            ))
            .await
            .unwrap(),
            scaffold::scaffold_files("empty-project")
                .into_iter()
                .find(|(p, _)| p == std::path::Path::new(".claude/CLAUDE.md"))
                .unwrap()
                .1
        );
    }

    #[gpui::test]
    async fn rejects_a_non_empty_directory(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/root",
            json!({ "not-empty": { "existing.txt": "hello" } }),
        )
        .await;

        let result = write_scaffold(fs.clone(), std::path::Path::new("/root/not-empty")).await;

        assert!(result.is_err());
        assert!(
            fs.load(std::path::Path::new("/root/not-empty/cpp/io.cpp"))
                .await
                .is_err(),
            "scaffold must not write anything into a non-empty directory"
        );
    }
}
```

- [ ] **Step 4: Run tests to verify they fail**

Run: `cargo test -p citadel_new_project`
Expected: compile error — `write_scaffold` is not defined.

- [ ] **Step 5: Implement the write-and-check logic and the full async action flow**

Prepend this to `crates/citadel_new_project/src/new_project.rs`, above the `#[cfg(test)] mod tests` block:

```rust
use std::path::Path;
use std::sync::Arc;

use fs::Fs;
use futures::StreamExt;
use gpui::{App, PathPromptOptions, WeakEntity, Window};
use notifications::status_toast::StatusToast;
use ui::{Color, Icon, IconName, IconSize};
use util::ResultExt;
use util::command::new_command;
use workspace::{self, Workspace};

use crate::scaffold;

/// Writes the scaffold into `destination` if it is empty; returns an error
/// (writing nothing) otherwise. `destination` must already exist as a
/// directory (the caller picks it via a folder-selection prompt).
async fn write_scaffold(fs: Arc<dyn Fs>, destination: &Path) -> anyhow::Result<()> {
    let mut entries = fs.read_dir(destination).await?;
    if entries.next().await.is_some() {
        anyhow::bail!("Selected folder is not empty");
    }

    let project_name = destination
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("project");

    for (relative_path, contents) in scaffold::scaffold_files(project_name) {
        let absolute_path = destination.join(&relative_path);
        if let Some(parent) = absolute_path.parent() {
            fs.create_dir(parent).await?;
        }
        fs.write(&absolute_path, contents.as_bytes()).await?;
    }

    Ok(())
}

async fn git_init_and_commit(fs: Arc<dyn Fs>, destination: &Path) -> anyhow::Result<()> {
    fs.git_init(destination, "main".to_string()).await?;

    let add_status = new_command("git")
        .current_dir(destination)
        .args(["add", "-A"])
        .status()
        .await?;
    anyhow::ensure!(add_status.success(), "git add -A failed");

    let commit_status = new_command("git")
        .current_dir(destination)
        .args(["commit", "-m", "Initial commit"])
        .status()
        .await?;
    anyhow::ensure!(commit_status.success(), "git commit failed");

    Ok(())
}

pub fn new_project(workspace: WeakEntity<Workspace>, window: &mut Window, cx: &mut App) {
    let destination_prompt = cx.prompt_for_paths(PathPromptOptions {
        files: false,
        directories: true,
        multiple: false,
        prompt: Some("Select Project Location".into()),
    });

    window
        .spawn(cx, async move |cx| {
            let mut paths = destination_prompt.await.ok()?.ok()??;
            let destination = paths.pop()?;

            let fs = workspace
                .read_with(cx, |workspace, _| workspace.app_state().fs.clone())
                .ok()?;

            let scaffold_result = write_scaffold(fs.clone(), &destination).await;
            if let Err(error) = scaffold_result {
                workspace
                    .update(cx, |workspace, cx| {
                        let toast = StatusToast::new(error.to_string(), cx, |this, _| {
                            this.icon(
                                Icon::new(IconName::XCircle)
                                    .size(IconSize::Small)
                                    .color(Color::Error),
                            )
                            .dismiss_button(true)
                        });
                        workspace.toggle_status_toast(toast, cx);
                    })
                    .ok()?;
                return None;
            }

            if let Err(error) = git_init_and_commit(fs.clone(), &destination).await {
                workspace
                    .update(cx, |workspace, cx| {
                        let toast = StatusToast::new(error.to_string(), cx, |this, _| {
                            this.icon(
                                Icon::new(IconName::XCircle)
                                    .size(IconSize::Small)
                                    .color(Color::Error),
                            )
                            .dismiss_button(true)
                        });
                        workspace.toggle_status_toast(toast, cx);
                    })
                    .ok()?;
                return None;
            }

            workspace
                .update(cx, move |workspace, cx| {
                    let app_state = workspace.app_state().clone();
                    workspace::open_new(Default::default(), app_state, cx, {
                        let destination = destination.clone();
                        move |workspace, window, cx| {
                            cx.activate(true);
                            let create_task = workspace.project().update(cx, |project, cx| {
                                project.create_worktree(destination.as_path(), true, cx)
                            });
                            cx.spawn_in(window, async move |_window, _cx| {
                                create_task.await.log_err();
                            })
                            .detach();
                        }
                    })
                    .detach();
                })
                .ok();

            Some(())
        })
        .detach();
}
```

- [ ] **Step 6: Run tests to verify they pass**

Run: `cargo test -p citadel_new_project`
Expected: all 12 tests pass (the 10 from Task 1 plus the 2 new `#[gpui::test]`s from Step 3).

- [ ] **Step 7: Wire the crate into `crates/zed`**

In `crates/zed/Cargo.toml`, add a new line directly above line 117 (`git_ui = { workspace = true, features = ["call"] }`):

```toml
citadel_new_project = { workspace = true }
```

In `crates/zed/src/main.rs`, add a new line directly above line 771 (`git_ui::init(cx);`):

```rust
citadel_new_project::init(cx);
```

- [ ] **Step 8: Add the File menu entry**

In `crates/zed/src/zed/app_menus.rs`, in the `File` menu's `items` vec, insert a new line directly after line 112 (`MenuItem::action("New Window", workspace::NewWindow),`) and before line 113 (`MenuItem::separator(),`):

```rust
MenuItem::action("New Project...", citadel_new_project::NewProject),
```

- [ ] **Step 9: Verify the whole editor still compiles**

Run: `cargo check -p zed`
Expected: succeeds with no errors (this is a type-check, much faster than a full `cargo build`/link — sufficient to confirm the new crate wires in correctly before attempting a full build).

- [ ] **Step 10: Commit**

```bash
cd /home/gooya/citadel
git add crates/citadel_new_project crates/zed/Cargo.toml crates/zed/src/main.rs crates/zed/src/zed/app_menus.rs
git commit -m "$(cat <<'EOF'
Wire up the Citadel New Project action end to end

Adds the citadel_new_project::NewProject action (File menu + command
palette), which prompts for an empty destination folder, writes the
Task 1 scaffold, git inits and commits it, and opens it as a new
workspace window — mirroring git_ui::clone::clone_and_open, the
closest existing analog in this codebase.
EOF
)"
```

- [ ] **Step 11: Manual verification (human required)**

This step needs a human at a real GUI session — the automated steps above only prove the code compiles and the pure/fake-fs-backed logic is correct; they cannot prove the actual folder-picker dialog, git behavior on a real filesystem, and new-window-opening behave correctly together.

Run `cargo run -p zed` (or `cargo run`, per `docs/src/development/linux.md`), then:
1. Open the File menu — confirm "New Project..." appears between "New Window" and the separator above "Open File...".
2. Trigger it (via the menu or the command palette, searching "New Project"). Select an **empty** folder. Confirm: a new window opens showing that folder as the project root, containing exactly the 8 files/paths from Task 1's scaffold, and `git log` in that folder shows one commit ("Initial commit").
3. Trigger it again, this time selecting a **non-empty** folder (e.g. one containing any file). Confirm an error toast appears and the folder's contents are unchanged.

Do not mark this task complete until a human confirms both outcomes.
