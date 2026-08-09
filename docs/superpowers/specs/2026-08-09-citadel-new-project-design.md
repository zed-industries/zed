# Design: Citadel New Project scaffolding (editor action)

Status: approved, ready for planning.

## Background

RFC 0002 (`docs/rfcs/0002-product-scope-and-dx.md` § Project scaffolding) describes `citadel init` and the IDE's "new project" flow producing a project directory that's ready to build and ready to commit, with a `rust/`+`cpp/` split matching the boundary rule validated end-to-end this session (`prototypes/0001-hello-blink`, `prototypes/0002-arduino-core`, both flashed and verified on a real ELEGOO UNO R3). This is the first work to land in `crates/` — everything so far has been `prototypes/` and `docs/`.

RFC 0002 also says `citadel init` and the in-editor "New Project" flow should be "the same code path." Investigation into the current codebase found:
- `crates/cli` has no subcommand system at all (a single flat `clap::Parser` args struct for "open these paths"); adding a `citadel init` subcommand is separate, larger work.
- The in-editor flow has a near-exact existing analog: `crates/git_ui/src/clone.rs`'s `clone_and_open`, which prompts for a destination directory, does the work, and opens the result as a new workspace via `workspace::open_new` + `project.create_worktree`.

## Scope

This design covers **only the in-editor "Citadel: New Project" action** — scaffolding a file tree, `git init`-ing it, and opening it as a new workspace. Out of scope, deferred to separate future work:
- The `citadel init` CLI subcommand (needs new subcommand plumbing in `crates/cli` first).
- Actual buildability: vendoring `ArduinoCore-avr`, generating a working `build.sh`, or invoking `cargo`/`avr-g++` at all. This design produces the file tree only.
- Board/toolchain auto-detection (RFC 0002 § Board and toolchain detection is explicitly still open). The scaffold always targets `atmega328p`, matching the hardware already validated this session.

## Architecture

One new crate, `crates/citadel_new_project`, self-contained like `git_ui` (no shared "pure logic crate consumed by CLI and GUI" abstraction yet — there is only one consumer today, and building that split now would be premature; when the CLI subcommand is built later, the non-GPUI scaffold-generation logic in this crate can be extracted into a shared crate at that point).

- Root `Cargo.toml`: add `"crates/citadel_new_project",` to `[workspace] members` (alongside `"crates/git_ui",` at line 88).
- `crates/zed/Cargo.toml`: add `citadel_new_project = { workspace = true }` as a dependency (alongside `git_ui` at line 117).
- `crates/zed/src/main.rs:771`: add `citadel_new_project::init(cx);` next to the existing `git_ui::init(cx);` call.
- `crates/citadel_new_project/src/citadel_new_project.rs`: `pub fn init(cx: &mut App)`, following `git_ui::init`'s pattern (`crates/git_ui/src/git_ui.rs:83`) of using `cx.observe_new::<Workspace>` to call `workspace.register_action(...)` for the new `NewProject` action.
- `crates/zed/src/zed/app_menus.rs`: add `MenuItem::action("New Project...", citadel_new_project::NewProject)` to the `File` menu's `items` vec, directly after the existing `MenuItem::action("New Window", workspace::NewWindow)` entry (currently line 112, ahead of the separator at line 113). This is a single shared menu definition (`app_menus()`, called from `crates/zed/src/main.rs:849` and `crates/zed/src/zed.rs:2283`) rendered as the native menu bar on macOS and the in-window menu on Linux/Windows — no per-platform duplication needed. The command palette also picks up the action automatically from its `actions!` doc comment, the same mechanism `NewFile`/`NewWindow` use, so no separate palette wiring is required.

## File tree and content

Generated inside whatever directory the user picks (that directory becomes the project root directly — no extra subdirectory is created):

```
<selected-directory>/
├── .git/                      # git init + initial commit
├── .gitignore
├── .claude/
│   ├── CLAUDE.md
│   └── skills/
│       └── .gitkeep
├── docs/
│   └── README.md
├── rust-toolchain.toml
├── rust/
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
└── cpp/
    └── io.cpp
```

**`.gitignore`:**
```
/build/
/rust/target/
```

**`.claude/CLAUDE.md`:**
```markdown
# Citadel project — Rust/C boundary rule

This project follows Citadel's architecture rule:

- `cpp/` may only perform direct, linear I/O hand-off: reading a pin, writing a pin, sending a byte, declaring `pinMode`/board constants. No `if`, no `for`/`while`, no ternaries, no computed intermediate variables.
- All logic — state transitions, calculations, control decisions — must live in `rust/` (a `#![no_std]` crate), never in `cpp/`.
- The two sides only exchange plain data across `extern "C"`: `cpp/` calls into `extern "C"` Rust functions, and Rust may read `extern "C"` variables/constants defined in `cpp/`.

If asked to add a decision or calculation to a file in `cpp/`, implement it in `rust/src/lib.rs` instead and expose it via an `extern "C"` function.
```

**`docs/README.md`:**
```markdown
# docs

Schematics, pin assignment notes, and other project documentation go here.
```

**`.claude/skills/.gitkeep`:** empty file (Git doesn't track empty directories).

**`rust-toolchain.toml`** (same pin as `prototypes/0001-hello-blink` and `prototypes/0002-arduino-core`):
```toml
[toolchain]
channel = "nightly-2026-08-06"
components = ["rust-src"]
```

**`rust/Cargo.toml`** (`<name>` = the selected directory's name, sanitized to a valid Cargo package name: lowercase the name, replace every run of one-or-more characters that are not `a-z`/`0-9` with a single `_`, then trim leading/trailing `_`; if the result is empty, use `project`. E.g. `My Project!` → `my_project`, `123!!!` → `project`):
```toml
[workspace]

[package]
name = "<name>_logic"
version = "0.1.0"
edition = "2021"

[lib]
path = "src/lib.rs"
crate-type = ["staticlib"]

[profile.release]
panic = "abort"
opt-level = "s"
lto = true
```

**`rust/src/lib.rs`:**
```rust
#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Add your logic here, exposed via `extern "C"` for cpp/io.cpp to call.
```

**`cpp/io.cpp`:**
```cpp
#include <Arduino.h>

void setup() {
    // pinMode(...) calls go here
}

void loop() {
    // straight-line I/O only — put decisions and calculations in rust/src/lib.rs
}
```

## UI / action flow

Mirrors `clone_and_open` (`crates/git_ui/src/clone.rs:8-159`), simplified — there is no "add to existing project or open new window" choice (git clone supports adding an existing repo into the current workspace; a brand-new project always opens in its own window, so that branch doesn't apply here):

```
"Citadel: New Project" action dispatched
  → cx.prompt_for_paths({ files: false, directories: true, multiple: false,
                           prompt: "Select Project Location" })
  → window.spawn(cx, async move |cx| { ... }).detach()
      → read the selected directory with Fs; if not empty, show a StatusToast
        error ("Selected folder is not empty") and stop
      → write the file tree (Fs::create_dir + Fs::write, per the table above)
      → Fs::git_init(&selected_dir)                         (fs.rs:1157)
      → new_command("git").current_dir(&selected_dir).args(["add", "-A"])
      → new_command("git").current_dir(&selected_dir).args(["commit", "-m", "Initial commit"])
      → on any failure above: StatusToast (git_ui::clone's error-toast pattern,
        crates/git_ui/src/clone.rs:47-62), stop — do not open a window over a
        half-scaffolded directory
      → workspace::open_new(Default::default(), app_state, cx, |workspace, window, cx| {
            project.create_worktree(&selected_dir, true, cx)
        }).detach()
```

`new_command` is `util::command::new_command` (`crates/util/src/command.rs:16-18`), the same primitive `Fs::git_init`/`Fs::git_clone` use internally (`crates/fs/src/fs.rs:1157-1207`) — not a new dependency.

**Empty-directory check:** if the selected directory is not empty, stop with an error rather than writing into it. This is the only validation; no separate "enter a project name" step exists (see Section 3 discussion — the selected directory name is the project name, used to derive `rust/Cargo.toml`'s package name).

## Testing

- **Scaffold content generation** (a pure function mapping project name → list of `(relative_path, content)` pairs, no `Fs`/GPUI involved) gets plain `#[test]` unit tests: assert the exact file list, assert `.claude/CLAUDE.md`/`cpp/io.cpp`/`rust/Cargo.toml` content, assert crate-name sanitization per the rule above (e.g. `My Project!` → package name `my_project_logic`; `123!!!` → `project_logic`).
- **Directory writing + empty-check + git init** get `#[gpui::test]`s using `FakeFs::new` + `insert_tree` (pattern: `crates/workspace/src/workspace.rs:11704-11796`, `test_tracking_active_path`) to seed an empty vs. non-empty fake directory and assert the resulting file contents / the rejection behavior.
- **Opening as a new workspace** gets a `#[gpui::test]` using `Project::test` + `Workspace::test_new` (`crates/project/src/project.rs:2068`, `crates/workspace/src/workspace.rs:7906`), asserting the new project's worktree is present after the action completes.

## Definition of done

- `crates/citadel_new_project` exists, registered in the root workspace and `crates/zed`, with `init()` called at startup.
- "New Project..." appears in the File menu (after "New Window") and is discoverable via the command palette.
- Running the action, selecting an empty directory, produces exactly the file tree above, a git repository with one commit, and opens the directory as a new workspace window.
- Selecting a non-empty directory shows an error toast and does not touch the directory.
- Unit tests for scaffold content generation and `#[gpui::test]`s for the write/git-init/open flow all pass.
