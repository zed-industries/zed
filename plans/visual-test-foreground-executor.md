# Controllable Foreground Executor for Visual Tests

## Status: COMPLETED ✓

The implementation is complete. See the summary at the end of this document.

## Problem Statement

The visual test framework (`VisualTestAppContext`) cannot properly test UI interactions that rely on foreground async tasks, such as tooltips, hover states with delays, and other deferred UI updates. This is because:

1. **The visual test framework uses the real macOS platform** for actual rendering (needed for screenshot capture)
2. **But it doesn't drive the macOS run loop**, so foreground tasks spawned via `window.spawn()` never execute
3. **`run_until_parked()` only drives the background executor**, not the foreground executor

### Specific Example: Tooltip Testing

When trying to capture a screenshot of a tooltip:

1. We call `window.simulate_mouse_move(position, cx)` to hover over a button
2. The button's tooltip handler in `register_tooltip_mouse_handlers` (in `div.rs`) spawns a delayed task:
   ```rust
   let delayed_show_task = window.spawn(cx, {
       async move |cx| {
           cx.background_executor().timer(TOOLTIP_SHOW_DELAY).await;  // 500ms
           cx.update(|window, cx| {
               // Set active_tooltip state
               window.refresh();
           }).ok();
       }
   });
   ```
3. We wait with `cx.background_executor().timer(700ms).await`
4. We take a screenshot - **but the tooltip never appears**

The reason: `window.spawn()` schedules work on the **foreground executor**, but our test only drives the **background executor**. The tooltip task is sitting in the foreground queue, never being processed.

## Architecture Overview

### Current Executor Setup

```
┌─────────────────────────────────────────────────────────────┐
│                     VisualTestAppContext                     │
├─────────────────────────────────────────────────────────────┤
│  platform: Rc<dyn Platform>  ←── MacPlatform (real)         │
│  background_executor: BackgroundExecutor                     │
│  foreground_executor: ForegroundExecutor                     │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                       MacPlatform                            │
├─────────────────────────────────────────────────────────────┤
│  dispatcher: Arc<MacDispatcher>                              │
│    - Uses real macOS run loop (CFRunLoop)                   │
│    - as_test() returns None                                  │
└─────────────────────────────────────────────────────────────┘
```

### Test Platform (for comparison)

```
┌─────────────────────────────────────────────────────────────┐
│                      TestAppContext                          │
├─────────────────────────────────────────────────────────────┤
│  platform: TestPlatform                                      │
│  dispatcher: TestDispatcher                                  │
│    - Controllable task queue                                │
│    - as_test() returns Some(self)                           │
│    - run_until_parked() drives both bg and fg tasks         │
└─────────────────────────────────────────────────────────────┘
```

## Key Files to Investigate

### 1. Platform Dispatcher Trait
**File:** `crates/gpui/src/platform.rs`

```rust
pub trait PlatformDispatcher: Send + Sync {
    fn is_main_thread(&self) -> bool;
    fn dispatch(&self, runnable: Runnable, label: Option<TaskLabel>);
    fn dispatch_on_main_thread(&self, runnable: Runnable);
    fn dispatch_after(&self, duration: Duration, runnable: Runnable);
    fn park(&self, timeout: Option<Duration>) -> bool;
    fn unparker(&self) -> Unparker;

    #[cfg(any(test, feature = "test-support"))]
    fn as_test(&self) -> Option<&TestDispatcher> {
        None  // Default implementation - real platforms return None
    }
}
```

### 2. Test Dispatcher Implementation
**File:** `crates/gpui/src/platform/test/dispatcher.rs`

This is the reference implementation for a controllable dispatcher:

```rust
pub struct TestDispatcher {
    state: Mutex<TestDispatcherState>,
    // ...
}

struct TestDispatcherState {
    random: StdRng,
    foreground: Vec<TestTask>,     // ← Foreground task queue
    background: Vec<TestTask>,     // ← Background task queue
    on_parking: Vec<UnparkCallback>,
    waiting_hint: Option<String>,
    block_on_ticks: RangeInclusive<usize>,
    forbid_parking: bool,
}

impl TestDispatcher {
    pub fn run_until_parked(&self) {
        // Drives BOTH foreground and background tasks
        while self.poll(true) {}
    }
    
    fn poll(&self, background_only: bool) -> bool {
        // Randomly selects and runs tasks from both queues
    }
}
```

### 3. Mac Dispatcher (Current Visual Test Dispatcher)
**File:** `crates/gpui/src/platform/mac/dispatcher.rs`

```rust
pub(crate) struct MacDispatcher;

impl PlatformDispatcher for MacDispatcher {
    fn dispatch_on_main_thread(&self, runnable: Runnable) {
        // Dispatches to actual macOS main thread via CFRunLoop
        unsafe {
            dispatch_async_f(
                dispatch_get_main_queue(),
                // ...
            );
        }
    }
    
    // as_test() returns None (default)
}
```

### 4. Visual Test Context Creation
**File:** `crates/gpui/src/app/visual_test_context.rs`

```rust
impl VisualTestAppContext {
    pub fn new() -> Self {
        let platform = current_platform(false);  // ← Gets MacPlatform
        let background_executor = platform.background_executor();
        let foreground_executor = platform.foreground_executor();
        // ...
    }
    
    pub fn run_until_parked(&self) {
        self.background_executor.run_until_parked();  // ← Only bg!
    }
}
```

### 5. BackgroundExecutor::run_until_parked
**File:** `crates/gpui/src/executor.rs`

```rust
impl BackgroundExecutor {
    pub fn run_until_parked(&self) {
        self.dispatcher.as_test().unwrap().run_until_parked()
        //              ^^^^^^^^^^^^^^^^ 
        // This panics for MacDispatcher because as_test() returns None!
    }
}
```

Wait - this means `run_until_parked()` should be panicking in visual tests. Let me check if it's actually being called...

Actually, looking at the test output, it seems like it doesn't panic. This needs investigation - perhaps there's a different code path or the method isn't being called.

## Proposed Solution

### Option A: Hybrid Visual Test Platform (Recommended)

Create a new platform type that:
- Uses **real Mac rendering** (Metal, text system, etc.) for actual pixel output
- Uses **TestDispatcher** for controllable task execution

```rust
pub struct VisualTestPlatform {
    // Real Mac components for rendering
    mac_text_system: Arc<MacTextSystem>,
    mac_renderer: MacRenderer,  // Or access via TestWindow with real backing
    
    // Test dispatcher for controllable execution
    dispatcher: Arc<TestDispatcher>,
}
```

**Pros:**
- Full control over task execution timing
- Deterministic test behavior
- Can still capture real rendered output

**Cons:**
- Significant implementation effort
- Need to carefully separate "rendering" from "event loop" concerns
- May need to refactor how windows get their renderer

### Option B: Foreground Task Pumping in Visual Tests

Add a mechanism to manually pump foreground tasks in the visual test context:

```rust
impl VisualTestAppContext {
    pub fn pump_foreground_tasks(&mut self, duration: Duration) {
        let deadline = Instant::now() + duration;
        while Instant::now() < deadline {
            // Manually poll the foreground task queue
            if !self.platform.pump_one_foreground_task() {
                std::thread::sleep(Duration::from_millis(1));
            }
        }
    }
}
```

This would require adding a new method to `PlatformDispatcher`:

```rust
pub trait PlatformDispatcher: Send + Sync {
    // Existing methods...
    
    /// Attempt to run one foreground task if available.
    /// Returns true if a task was run, false if queue was empty.
    fn pump_one_foreground_task(&self) -> bool { false }
}
```

**Pros:**
- Smaller change
- Doesn't require new platform type

**Cons:**
- Less deterministic (still time-based)
- May not work well with macOS's dispatch queue semantics

### Option C: Window-Spawned Task Interception

Intercept tasks spawned via `window.spawn()` and redirect them to a controllable queue:

```rust
impl Window {
    pub fn spawn<R>(&mut self, cx: &mut App, f: impl Future<Output = R>) -> Task<R> {
        #[cfg(any(test, feature = "test-support"))]
        if cx.is_visual_test_mode() {
            return self.spawn_to_test_queue(f);
        }
        
        // Normal spawn path
        self.foreground_executor.spawn(f)
    }
}
```

**Pros:**
- Targeted fix for the specific problem
- Minimal changes to existing code

**Cons:**
- Doesn't solve the general problem
- Test-specific code paths can diverge from production behavior

## Implementation Plan for Option A

### Step 1: Create TestDispatcher with Real Timing Option

Modify `TestDispatcher` to support real-time delays instead of simulated time:

```rust
pub struct TestDispatcher {
    state: Mutex<TestDispatcherState>,
    use_real_time: bool,  // New flag
}

impl TestDispatcher {
    pub fn new_with_real_time() -> Self {
        Self {
            state: Mutex::new(TestDispatcherState::new()),
            use_real_time: true,
        }
    }
}
```

### Step 2: Create VisualTestPlatform

**New file:** `crates/gpui/src/platform/visual_test/mod.rs`

```rust
pub struct VisualTestPlatform {
    dispatcher: Arc<TestDispatcher>,
    text_system: Arc<MacTextSystem>,
    // Other Mac components needed for rendering
}

impl Platform for VisualTestPlatform {
    fn dispatcher(&self) -> Arc<dyn PlatformDispatcher> {
        self.dispatcher.clone()
    }
    
    fn text_system(&self) -> Arc<dyn PlatformTextSystem> {
        self.text_system.clone()
    }
    
    fn open_window(...) -> ... {
        // Create window with real Metal rendering
        // but using our test dispatcher
    }
}
```

### Step 3: Create VisualTestWindow

The window needs to use real rendering but the test dispatcher:

```rust
pub struct VisualTestWindow {
    // Real Metal/rendering components from MacWindow
    renderer: Renderer,
    native_view: ...,
    
    // But dispatches through TestDispatcher
    dispatcher: Arc<TestDispatcher>,
}
```

### Step 4: Update VisualTestAppContext

```rust
impl VisualTestAppContext {
    pub fn new() -> Self {
        let dispatcher = Arc::new(TestDispatcher::new_with_real_time());
        let platform = Arc::new(VisualTestPlatform::new(dispatcher.clone()));
        // ...
    }
    
    pub fn run_until_parked(&self) {
        // Now this works because we have a TestDispatcher
        self.dispatcher.run_until_parked();
    }
}
```

### Step 5: Test the Tooltip Capture

```rust
// In visual_test_runner.rs
cx.simulate_mouse_move(window, button_position, None, Modifiers::default());

// Wait real time for the tooltip delay
cx.background_executor()
    .timer(Duration::from_millis(600))
    .await;

// Drive all pending tasks including the tooltip show task
cx.run_until_parked();

// Now the tooltip should be visible
cx.update_window(window, |_, window, _| window.refresh())?;

// Capture screenshot with tooltip
let screenshot = capture_screenshot(window, cx)?;
```

## Testing the Fix

After implementation, these scenarios should work:

1. **Tooltip on hover**: Mouse over button → wait → tooltip appears in screenshot
2. **Hover styles**: Mouse over element → hover style visible in screenshot  
3. **Delayed animations**: Any animation triggered by foreground tasks
4. **Debounced updates**: UI updates that use debouncing/throttling

## Files to Modify

1. `crates/gpui/src/platform.rs` - May need new trait methods
2. `crates/gpui/src/platform/test/dispatcher.rs` - Add real-time mode
3. `crates/gpui/src/platform/visual_test/mod.rs` - New file
4. `crates/gpui/src/platform/visual_test/platform.rs` - New file
5. `crates/gpui/src/platform/visual_test/window.rs` - New file
6. `crates/gpui/src/app/visual_test_context.rs` - Use new platform
7. `crates/gpui/src/platform/mod.rs` - Export new module
8. `crates/zed/src/visual_test_runner.rs` - Update test code

## Questions to Resolve

1. **Can Metal rendering work without the macOS run loop?** 
   - Need to investigate if `CAMetalLayer` and friends require the run loop
   
2. **How does `render_to_image()` work?**
   - This is used for screenshot capture - need to ensure it works with test platform

3. **What about system events (keyboard, mouse)?**
   - Visual tests simulate these - should work with test dispatcher

4. **Thread safety concerns?**
   - TestDispatcher is designed for single-threaded use
   - Metal rendering may have threading requirements

## Related Code References

- `Window::spawn` - `crates/gpui/src/window.rs`
- `register_tooltip_mouse_handlers` - `crates/gpui/src/elements/div.rs:2845`
- `handle_tooltip_mouse_move` - `crates/gpui/src/elements/div.rs:2873`
- `TOOLTIP_SHOW_DELAY` - `crates/gpui/src/elements/div.rs:48` (500ms)
- `TestWindow` - `crates/gpui/src/platform/test/window.rs`
- `MacWindow` - `crates/gpui/src/platform/mac/window.rs`

## Success Criteria

1. `diff_review_button_tooltip.png` shows the "Add Review" tooltip
2. Button shows hover border when mouse is over it
3. Tests remain deterministic (same output every run)
4. No reliance on wall-clock timing for correctness

---

## Implementation Summary

### Changes Made

1. **Created `VisualTestPlatform`** (`crates/gpui/src/platform/visual_test.rs`)
   - A hybrid platform that combines real Mac rendering with controllable `TestDispatcher`
   - Uses `MacPlatform` for window creation, text system, rendering, and display management
   - Uses `TestDispatcher` for deterministic task scheduling
   - Implements the `Platform` trait, delegating rendering operations to `MacPlatform`
   - Passes its own `ForegroundExecutor` (from `TestDispatcher`) to `MacWindow::open()`

2. **Added `renderer_context()` method to `MacPlatform`** (`crates/gpui/src/platform/mac/platform.rs`)
   - Allows `VisualTestPlatform` to access the renderer context for window creation
   - Conditionally compiled for test-support

3. **Updated `VisualTestAppContext`** (`crates/gpui/src/app/visual_test_context.rs`)
   - Now creates and uses `VisualTestPlatform` instead of `current_platform()`
   - Gets dispatcher and executors from the platform
   - This ensures `App::spawn()` and `Window::spawn()` use the `TestDispatcher`

4. **Added tests** (`crates/gpui/src/app/visual_test_context.rs`)
   - `test_foreground_tasks_run_with_run_until_parked` - verifies foreground tasks execute
   - `test_advance_clock_triggers_delayed_tasks` - verifies timer-based tasks work
   - `test_window_spawn_uses_test_dispatcher` - verifies window.spawn uses TestDispatcher
   - All tests are marked `#[ignore]` because they require macOS main thread

### How It Works

The key insight was that `App::new_app()` gets its executors from `platform.foreground_executor()`. Previously:

```
VisualTestAppContext
  └── creates TestDispatcher (unused!)
  └── creates App with MacPlatform
        └── MacPlatform has MacDispatcher
        └── App uses MacDispatcher's executors ❌
```

After the fix:

```
VisualTestAppContext
  └── creates VisualTestPlatform
        └── Has TestDispatcher
        └── Has MacPlatform (for rendering)
        └── foreground_executor() returns TestDispatcher's executor ✓
  └── creates App with VisualTestPlatform
        └── App uses TestDispatcher's executors ✓
        └── Window::spawn() uses TestDispatcher ✓
```

### Running Visual Tests

Visual tests require the macOS main thread. Run them with:

```bash
cargo test -p gpui visual_test_context -- --ignored --test-threads=1
cargo test -p zed visual_tests -- --ignored --test-threads=1
```

### Tooltip Testing

With this fix, tooltip testing now works:

```rust
// Simulate hovering over a button
cx.simulate_mouse_move(window, button_position, None, Modifiers::default());

// Advance clock past TOOLTIP_SHOW_DELAY (500ms)
cx.advance_clock(Duration::from_millis(600));

// The tooltip task spawned via window.spawn() is now executed!
// Take screenshot - tooltip will be visible
let screenshot = cx.capture_screenshot(window)?;
```