# Problem: App-Alive Check in Trampoline - Threading Constraints

## Context

We're implementing Phase 1 of the async-app-result-removal project: adding infrastructure to cancel foreground tasks when the app is dropped. The goal is to check if the app is still alive before running each task in the trampoline function.

## The Original Design

The brief proposed adding `Option<Weak<AppCell>>` to `RunnableMeta`:

```rust
pub struct RunnableMeta {
    pub location: &'static Location<'static>,
    pub app: Option<Weak<AppCell>>,  // NEW
}
```

The trampoline would then check:
```rust
if let Some(app_weak) = &task.metadata().app {
    if app_weak.upgrade().is_none() {
        drop(task);  // Cancel task
        return;
    }
}
```

## The Problem

`AppCell` contains `RefCell<App>`, and `Weak<AppCell>` is actually `Weak<Rc<AppCell>>` which is `!Send` and `!Sync`.

`RunnableMeta` is used for **both** foreground and background tasks. Background tasks use channels (like flume) to dispatch work across threads. When `RunnableMeta` contains `Weak<AppCell>`, the entire `Runnable<RunnableMeta>` type becomes `!Send`, breaking background task dispatch:

```
error[E0277]: `std::rc::Weak<app::AppCell>` cannot be sent between threads safely
   --> crates/gpui/src/executor.rs:317:17
    |
317 | /                 Box::new(move || {
318 | |                     while let Ok(runnable) = rx.recv() {  // flume channel
    ...
```

## Key Insight

**Background tasks don't need the app-alive check.** They can't hold `AsyncApp` because `AsyncApp` is `!Send` (it contains `Weak<Rc<...>>`). Only foreground tasks spawned via `AsyncApp::spawn` need this check.

## Potential Solutions

### Option 1: Thread-Local Storage (Simplest)

Store the `Weak<AppCell>` in a thread-local on the main thread. The trampoline (which runs on the main thread for foreground tasks) checks this thread-local.

```rust
thread_local! {
    static APP_WEAK: RefCell<Option<Weak<AppCell>>> = RefCell::new(None);
}

// In trampoline:
extern "C" fn trampoline(runnable: *mut c_void) {
    // Check if app is alive via thread-local
    let app_alive = APP_WEAK.with(|weak| {
        weak.borrow().as_ref().map(|w| w.upgrade().is_some()).unwrap_or(true)
    });
    if !app_alive {
        drop(task);
        return;
    }
    // ... run task
}
```

**Pros:**
- Simple, no changes to `RunnableMeta`
- Thread-local is naturally `!Send`, which is fine since it's only accessed on main thread
- Background tasks unaffected

**Cons:**
- Need to initialize the thread-local when app starts
- All foreground tasks get the check, not just those spawned via `AsyncApp`

### Option 2: Separate Metadata Types

Create `ForegroundRunnableMeta` with the weak pointer, used only for `dispatch_on_main_thread`. Keep `RunnableMeta` (or a `BackgroundRunnableMeta`) without it.

```rust
pub struct ForegroundRunnableMeta {
    pub location: &'static Location<'static>,
    pub app: Option<Weak<AppCell>>,
}

pub struct BackgroundRunnableMeta {
    pub location: &'static Location<'static>,
}

pub enum RunnableVariant {
    Foreground(Runnable<ForegroundRunnableMeta>),
    Background(Runnable<BackgroundRunnableMeta>),
    Compat(Runnable),
}
```

**Pros:**
- Type-safe: foreground tasks can carry app pointer, background can't
- Explicit about which tasks get checked

**Cons:**
- More complex, need to update all dispatch paths
- `RunnableVariant` becomes larger

### Option 3: `Arc<AtomicBool>` Flag

Use a thread-safe atomic flag instead of `Weak<AppCell>`:

```rust
pub struct RunnableMeta {
    pub location: &'static Location<'static>,
    pub app_alive: Option<Arc<AtomicBool>>,
}
```

Set the flag to `false` when the app is dropped (via `impl Drop for App`).

**Pros:**
- `Arc<AtomicBool>` is `Send + Sync`
- Works uniformly across all task types

**Cons:**
- Need to implement `Drop` for `App` to set the flag
- Extra allocation and atomic operations
- Conceptually awkward: we're duplicating "is the app alive" state

## Current State

We started implementing Option 3 (`Arc<AtomicBool>`) but paused to reconsider. The code changes made so far:

1. Added `app_alive: Option<Arc<AtomicBool>>` to `RunnableMeta`
2. Updated Mac dispatcher trampoline to check the flag
3. Updated Test dispatcher tick function to check the flag
4. Added `spawn_with_app` method to `ForegroundExecutor`
5. Updated `AsyncApp::spawn` and `AsyncWindowContext::spawn` to use it
6. Started adding `app_alive: Arc<AtomicBool>` field to `App` struct

## Recommendation

**Option 1 (Thread-Local)** seems cleanest because:
- It leverages the fact that foreground tasks always run on the main thread
- No changes needed to `RunnableMeta` or task dispatch infrastructure
- The "is app alive" check naturally belongs on the main thread where the app lives
- Minimal code changes

The thread-local would be set when `Application` is created and cleared when it's dropped.