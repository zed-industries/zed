# Perf Callgraph Analysis Plan

**Branch**: `callgraph-perf`
**Goal**: Build a static analysis tool that enforces the property *"no async function calls a blocking/sync function"* in the Zed codebase, modeled after Ferrocene's callgraph analysis approach but using a `syn`-based implementation that works on stable Rust.

---

## 1. Problem Statement

When an `async` function calls a blocking operation (e.g., `std::fs::read`, `std::thread::sleep`, `Mutex::lock`), it blocks the executor thread and stalls all other tasks. In Zed, this means the UI freezes. Today there is no automated way to catch this — it's caught by humans in code review, or worse, by users experiencing hangs.

We want a static analysis tool that detects these violations, runnable via a simple script.

### Property (informal)

> No async function calls a blocking function.

### Property (formal)

> Let **A** be the set of functions reachable from an async context (async functions, closures passed to GPUI's `spawn`/`background_spawn`). Let **B** be the set of known-blocking functions. **A ∩ B = ∅** — no function in **A** may transitively call a function in **B**.

---

## 2. Design Space: Blocklist vs Allowlist

Both designs share the same analysis infrastructure. They differ only in what the lint considers a violation.

### 2a. Blocklist Model ("deny known-bad")

**How it works**: Maintain a curated list of functions known to block (the "blocklist"). The lint fires when an async function transitively calls anything on the list.

**Annotation surface**:
```rust
// No per-function annotation needed — async functions are detected automatically.
// Escape hatch for intentional blocking:
#[allow(perf::blocking_in_async)]
async fn intentionally_blocks() {
    std::thread::sleep(Duration::from_secs(1)); // allowed here
}
```

**Blocklist categories** (starter set):
| Category | Examples |
|----------|----------|
| Filesystem I/O | `std::fs::*` (`read`, `write`, `metadata`, `create_dir`, `remove_file`, …) |
| Thread blocking | `std::thread::sleep`, `std::thread::park` |
| Sync locks | `std::sync::Mutex::lock`, `std::sync::RwLock::read/write`, then `parking_lot::Mutex::lock` |
| Blocking I/O | `std::io::stdin().read_line()`, `std::net::TcpStream::connect` |
| Process spawning (sync) | `std::process::Command::output`, `std::process::Command::status` |
| Blocking futures | `pollster::block_on`, `futures::executor::block_on` |

**Pros**:
- Zero annotation burden on existing code
- Immediately actionable — catches real bugs today
- Easy to grow incrementally
- Backwards-compatible: no code changes needed to compile normally

**Cons**:
- Incomplete: custom blocking code won't be caught unless added to the list
- Doesn't catch blocking in third-party crate internals (syntactic analysis only)
- The blocklist requires maintenance

### 2b. Allowlist Model ("only allow known-good", Ferrocene-style)

**How it works**: Every function callable from an async context must be explicitly marked `#[perf::async_safe]`. Unmarked functions are assumed blocking. The set of async-safe functions must be closed: an `async_safe` function can only call other `async_safe` functions.

**Pros**:
- Sound by construction: if it compiles, the property holds
- Self-documenting: the annotation tells readers "this is safe to call from async"
- Catches ALL blocking calls, not just known ones

**Cons**:
- Enormous annotation burden (~thousands of functions and all their transitive callees)
- Can't annotate third-party crate code or std without wrapper functions or an external manifest
- Requires buy-in from all contributors — every new function must be classified

### 2c. Decision: Start with Blocklist, Path to Hybrid

**Phase 1** (this plan): Implement the **blocklist model**. It provides immediate value with zero annotation cost.

**Phase 2** (future): Add an optional `#[perf::blocking]` annotation so developers can mark their own blocking functions. The lint then checks both the built-in blocklist AND user annotations.

**Phase 3** (future): If coverage gaps are found, consider an allowlist for specific high-value subtrees (e.g., the `editor` crate).

---

## 3. Architecture

### 3.1 Standalone Analysis Tool (syn-based)

We build a standalone Rust binary that uses `syn` for source parsing and `cargo_metadata` for project discovery. This works on **stable Rust** — no nightly toolchain required.

```
┌───────────────────────┐     ┌──────────────────────────┐
│ ./script/              │     │  zed-callgraph            │
│   check-async-blocking │────▶│  (standalone binary)      │
│   -p editor            │     │                          │
└───────────────────────┘     │  1. cargo_metadata        │
                              │     → find source files   │
                              │  2. syn::parse_file       │
                              │     → AST per file        │
                              │  3. Walk AST:             │
                              │     Find async fns        │
                              │     Find spawn closures   │
                              │     Check calls vs        │
                              │       blocklist           │
                              │  4. Emit diagnostics      │
                              └──────────────────────────┘
```

**Key components**:

1. **`zed-callgraph`** — the binary
2. **Blocklist database** — a compiled-in list of path patterns for known-blocking functions, loaded from `blocklist.toml`
3. **Async context detector** — identifies functions that run in async contexts:
   - Rust `async fn`
   - Closures/async blocks passed to GPUI spawn methods
4. **AST callgraph walker** — traverses function bodies using `syn::visit`, matching call expressions against the blocklist

### 3.2 Toolchain Strategy

**No nightly required.** The tool is a normal Rust binary using `syn`, `cargo_metadata`, and standard crates. It compiles with the same stable toolchain as Zed (1.95.0).

The tool lives in `tooling/callgraph/` within the Zed workspace but is a standalone binary, not part of Zed's main build graph.

```sh
# Build the tool
cargo build -p callgraph --release

# Run it
./script/check-async-blocking -p editor
```

**Tradeoff vs. a custom rustc driver (Approach A)**:
| | syn-based (chosen) | rustc driver |
|---|---|---|
| Toolchain | Stable | Nightly only |
| Maintenance burden | Low — syn is stable | High — rustc_private APIs break often |
| Accuracy: direct calls | Good — matches path strings | Perfect — uses DefId |
| Accuracy: method calls | Limited — needs type context | Perfect — resolved by compiler |
| Accuracy: macros | Sees invocations, not expansions | Sees expanded code |
| Accuracy: generics | Cannot resolve | Post-mono pass can resolve |
| Speed | Fast (just parsing) | Slower (full compilation) |

The syn approach is the right starting point: it catches the most common and egregious violations (direct calls to `std::fs::*`, `std::thread::sleep`, etc.) with minimal infrastructure. If we later need deeper analysis, we can upgrade to a rustc driver for Phase 4.

### 3.3 GPUI-Aware Async Context Detection

Plain Rust `async fn` is easy to detect syntactically. The harder part is GPUI's closure-based async:

```rust
cx.spawn(async move |this, cx| {
    // This closure is async, but from the compiler's perspective
    // it's just an AsyncFnOnce closure argument.
    this.update(cx, |this, cx| {
        // This inner closure is SYNC — it's running in Entity::update.
        // Blocking here freezes the UI — flag it too.
    })?;
    Ok(())
});
```

**Detection strategy**:

1. **Rust `async fn`**: Trivially detected — `syn::ItemFn` with `sig.asyncness.is_some()`.

2. **GPUI spawn closures**: Detect method calls where the receiver method name matches known spawn methods, then treat their closure/async-block arguments as async contexts. Methods to recognize:
   - `.spawn(...)`, `.spawn_with_priority(...)`
   - `.spawn_in(...)`, `.spawn_in_with_priority(...)`
   - `.background_spawn(...)`

3. **`Entity::update` closures inside spawn**: These are sync closures that run on the foreground thread. Blocking in them freezes the UI. Flag blocking calls in these closures too.

4. **Escape hatches** (blocking is intentional, suppress warnings):
   - `smol::unblock(|| ...)` — explicitly moves blocking work to a thread pool
   - `std::thread::spawn(|| ...)` — runs on a separate OS thread

### 3.4 AST-Based Call Detection (Phase 1)

The analysis walks the AST of each source file using `syn::visit::Visit`.

**Algorithm**:

```
for each source file in the target crate(s):
    parse with syn::parse_file
    for each function/method F:
        if F is async:
            walk_body(F.body, blocklist, context="async fn F")
        for each call to a spawn method in F.body:
            let closure = extract_closure_arg(call)
            walk_body(closure.body, blocklist, context="spawn closure at line N")

walk_body(body, blocklist, context):
    for each expression in body (recursive via syn::visit):
        if expression is a call to smol::unblock or std::thread::spawn:
            skip the closure argument (it's an escape hatch)
        if expression is a function call or method call:
            let path = extract_call_path(expression)
            if blocklist.matches(path):
                emit_warning(expression.span, path, context)
```

**Path matching**: We match against the syntactic path as written in source. This means:
- `std::fs::read(...)` matches `std::fs::*`
- `fs::read(...)` matches if there's a `use std::fs` import (we resolve `use` declarations)
- `.lock()` on a type we can't resolve → conservative: flag it if the method name matches and we can't prove it's not a Mutex

**What this catches**:
- Direct calls to blocklisted functions from async code
- Calls using common import aliases (`use std::fs; fs::read(...)`)
- Blocking in GPUI spawn closures
- Blocking in Entity::update closures inside spawn

**What this misses** (deferred to later phases):
- Calls through fully-qualified paths we don't recognize
- Method calls where we can't determine the receiver type
- Transitive calls through helper functions (Phase 2)
- Macro-expanded code (Phase 4)

### 3.5 Diagnostic Output

```
warning[blocking-in-async]: blocking call in async context
  --> crates/editor/src/editor.rs:1234:9
   |
1234 |     std::fs::read_to_string(&path)?;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `std::fs::read_to_string` is a blocking filesystem operation
   = help: use `smol::unblock(|| std::fs::read_to_string(&path))` or an async fs API
   = context: async fn `Editor::load_file`

warning[blocking-in-async]: blocking call in async context
  --> crates/project/src/project.rs:567:9
   |
567 |     std::thread::sleep(Duration::from_secs(1));
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `std::thread::sleep` blocks the current thread
   = help: use `cx.background_executor().timer(duration).await`
   = context: closure passed to `cx.spawn()` at line 560
```

Output format options:
- **Human** (default): colored, rustc-style diagnostics as above
- **JSON**: machine-readable for CI integration (`--output json`)

---

## 4. Implementation Plan

### Phase 1: Skeleton & Direct Call Detection (MVP)

| Step | Description | Deliverable |
|------|-------------|-------------|
| 1.1 | Create `tooling/callgraph/` crate with `Cargo.toml`, `src/main.rs`, `src/callgraph.rs` | Crate skeleton that compiles |
| 1.2 | Implement `blocklist.toml` format and loader | `Blocklist` struct that can test path patterns |
| 1.3 | Implement source file discovery via `cargo_metadata` | Given `-p crate_name`, finds all `.rs` source files |
| 1.4 | Implement `async fn` detection using `syn::visit` | Can list all async functions in a crate |
| 1.5 | Implement direct-call detection: walk async fn bodies, match calls against blocklist | Emits warnings for direct blocking calls |
| 1.6 | Implement `use` resolution for common import patterns | `use std::fs; fs::read(...)` is caught |
| 1.7 | Write tests: unit tests for blocklist matching, integration tests with fixture files | CI-runnable test suite |
| 1.8 | Add `script/check-async-blocking` wrapper script | Easy invocation for developers |

### Phase 2: GPUI-Aware + Transitive Analysis

| Step | Description | Deliverable |
|------|-------------|-------------|
| 2.1 | Detect GPUI spawn points and mark closure args as async contexts | Catches blocking in spawn closures |
| 2.2 | Detect `Entity::update` closures inside spawn and flag blocking there too | Catches UI-freezing sync callbacks |
| 2.3 | Build intra-file callgraph: track which local functions call which | Map of function → callees |
| 2.4 | Implement transitive reachability within a file | Catches `async fn foo() { helper(); }` where `helper` blocks |
| 2.5 | Recognize `smol::unblock` and `std::thread::spawn` as escape hatches | Suppresses false positives |
| 2.6 | Add `#[perf::blocking]` user annotation support (parsed from source) | Developers can mark their own blocking functions |

### Phase 3: Hardening & CI Integration

| Step | Description | Deliverable |
|------|-------------|-------------|
| 3.1 | Run on all Zed crates, triage results, tune blocklist | Baseline of known violations |
| 3.2 | Add `#[allow(perf::blocking_in_async)]` parsing for sanctioned violations | Existing code passes cleanly |
| 3.3 | JSON output mode for CI | Machine-parseable results |
| 3.4 | Integrate into CI as a warning (non-blocking) | PR feedback on new violations |
| 3.5 | Promote to CI-blocking once clean | Prevents regressions |

### Phase 4: Upgrade to rustc Driver (Future)

If the syn-based approach proves too limited (too many false negatives through generics, macros, or cross-crate calls), upgrade to a full `rustc_driver` with THIR/MIR passes. The blocklist, diagnostics, and test infrastructure carry over.

---

## 5. Crate Structure

```
tooling/callgraph/
├── Cargo.toml
├── src/
│   ├── callgraph.rs             # lib root: re-exports, shared types
│   ├── main.rs                  # binary entry point, CLI argument parsing
│   ├── blocklist.rs             # Blocklist: loads blocklist.toml, matches paths
│   ├── async_context.rs         # finds async fns + GPUI spawn closures in AST
│   ├── analyzer.rs              # walks AST, checks calls against blocklist
│   └── diagnostics.rs           # formats and emits warnings
├── blocklist.toml               # the blocklist definition (compiled in via include_str!)
└── tests/
    ├── fixtures/                # test Rust source files
    │   ├── direct_block.rs      # should warn
    │   ├── import_block.rs      # should warn (use std::fs)
    │   ├── spawn_block.rs       # should warn (blocking in cx.spawn)
    │   ├── clean.rs             # should NOT warn
    │   └── escape_hatch.rs      # should NOT warn (smol::unblock)
    └── integration.rs           # runs analyzer on fixtures, checks output
```

### `blocklist.toml` format

```toml
# Each entry is a path pattern matched against call paths in source.
# Patterns use `::` separators and support trailing `::*` wildcards.

[[blocking]]
path = "std::fs::*"
category = "filesystem"
help = "use an async fs API or wrap in `smol::unblock`"

[[blocking]]
path = "std::thread::sleep"
category = "thread"
help = "use `cx.background_executor().timer(duration).await`"

[[blocking]]
path = "std::thread::park"
category = "thread"
help = "use an async synchronization primitive"

[[blocking]]
path = "std::sync::Mutex::lock"
category = "sync-lock"
help = "use `parking_lot::Mutex` (non-poisoning, shorter hold) or an async mutex"

[[blocking]]
path = "std::sync::RwLock::read"
category = "sync-lock"
help = "use an async RwLock"

[[blocking]]
path = "std::sync::RwLock::write"
category = "sync-lock"
help = "use an async RwLock"

[[blocking]]
path = "parking_lot::Mutex::lock"
category = "sync-lock"
tier = "pedantic"
help = "consider an async mutex if held across await points"

[[blocking]]
path = "std::net::TcpStream::connect"
category = "network"
help = "use `smol::net::TcpStream::connect` or an async HTTP client"

[[blocking]]
path = "std::net::TcpListener::bind"
category = "network"
help = "use an async networking library"

[[blocking]]
path = "std::process::Command::output"
category = "process"
help = "use `smol::process::Command` or wrap in `smol::unblock`"

[[blocking]]
path = "std::process::Command::status"
category = "process"
help = "use `smol::process::Command` or wrap in `smol::unblock`"

[[blocking]]
path = "pollster::block_on"
category = "blocking-future"
help = "you are blocking on a future inside an async context — just await it"

[[blocking]]
path = "futures::executor::block_on"
category = "blocking-future"
help = "you are blocking on a future inside an async context — just await it"

# Escape hatches: closures passed to these are NOT flagged
[[safe_wrapper]]
path = "smol::unblock"
note = "wraps blocking work in a thread pool"

[[safe_wrapper]]
path = "std::thread::spawn"
note = "runs on a separate OS thread"
```

---

## 6. Key Risks & Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| syn can't resolve method calls by type | False negatives: `.lock()` on non-Mutex types missed, or false positives on `.lock()` on other types | Use heuristics: if the method name is `lock` and we can't prove it's NOT a Mutex, flag it with lower confidence |
| syn can't see through macros | Blocking calls inside `macro_rules!` invocations are invisible | Document the gap; most blocking calls in Zed are not macro-generated |
| Import resolution is imperfect | `use std::fs as myfs; myfs::read()` might be missed | Handle common patterns (`use X`, `use X as Y`, `use X::*`); accept some misses |
| Too many false positives | Developers ignore the tool | Start with a conservative blocklist; `parking_lot::Mutex` is "pedantic" tier (opt-in); tune before CI |
| Too many false negatives | False sense of security | Document known gaps; this is a best-effort lint, not a proof system |
| Large codebase = slow analysis | Annoying to run | syn parsing is fast (~100 files/sec); parallelize with rayon if needed |

---

## 7. Resolved Decisions

1. **`parking_lot::Mutex::lock`**: Include in the blocklist but in a separate "pedantic" tier. Start by flagging `std::sync::Mutex::lock` at default level. `parking_lot::Mutex::lock` is flagged only when `--pedantic` is passed or when the crate opts into pedantic checking.

2. **`Entity::update` closures inside `cx.spawn`**: Flag blocking calls in these closures. They run on the foreground thread and blocking in them freezes the UI. This is a Phase 2 deliverable.

3. **`--fix` mode**: Not for Phase 1. Consider for Phase 3.

---

## 8. Success Criteria

- **Phase 1 done**: Running `./script/check-async-blocking -p some_crate` produces correct warnings for direct blocking calls in `async fn` bodies. Has tests proving it catches `std::fs::*`, `std::thread::sleep`, `std::sync::Mutex::lock` and doesn't false-positive on `smol::unblock` or non-async functions.
- **Phase 2 done**: The tool catches blocking calls through GPUI spawn closures, Entity::update closures, and transitive helper functions, with zero false positives on `smol::unblock` usage.
- **Phase 3 done**: All Zed crates pass the lint cleanly (with sanctioned `#[allow]`s), and CI prevents regressions.
