date: 2025-08-31T15:47:17.200692-06:00
researcher: claude-code
git_commit: 8a2c37a3c10a5d2869a8d829bdb9688c74e9d403
branch: delta
repository: zed-industries/cloud
topic: "Unified Scheduler Architecture Design"
tags: [research, scheduler, executor, gpui, cloud, architecture, async-task]
status: research
last_updated: 2025-08-31
last_updated_by: claude-code

# Unified Scheduler Architecture Research

**Date**: 2025-08-31T15:47:17.200692-06:00
**Researcher**: Claude Code
**Git Commit**: 8a2c37a3c10a5d2869a8d829bdb9688c74e9d403
**Branch**: delta
**Repository**: zed-industries/cloud

This research explores designing a unified scheduler system to replace GPUI's `PlatformDispatcher` + `Executor` architecture and Cloud's `SimulatorRuntime` architecture. The goal is to create a common scheduling foundation that supports both production performance and testing determinism while maintaining backward compatibility and proper Send/non-Send distinctions.

## Overview and Goals

The current GPUI and Cloud architectures have separate scheduling systems:
- GPUI uses `PlatformDispatcher` (low-level) + `BackgroundExecutor`/`ForegroundExecutor` (high-level)
- Cloud uses `SimulatorRuntime` for deterministic testing

We need a unified abstraction that:
- **Unifies platforms**: Single trait for macOS, Linux, Windows, and testing
- **Handles Send vs non-Send**: Background for Send futures, foreground for main-thread-only futures
- **Provides delayed scheduling**: Native timers without runtime dependencies
- **Supports determinism**: Fake time and controlled execution for testing
- **Maintains compatibility**: Zero breaking changes for GPUI APIs

### Scope Clarification

- **Scheduler Crate**: Platform abstraction exposing single `Scheduler` trait with low-level scheduling primitives, extension methods for high-level APIs, plus `Task<T>` type and backend implementations
- **GPUI Integration**: Re-export scheduler crate's `Task<T>` and adapt existing `BackgroundExecutor`/`ForegroundExecutor` APIs using extension methods
- **Cloud Features**: Cloud-specific features like wait-until and session tracking remain in `SimulatedExecutionContext`, leveraging shared `Task<T>` IDs

## Core Architecture

### Core Types (in Scheduler Crate)

```rust
// Scheduler crate: shared types for GPUI and Cloud
#[derive(Debug)]
pub struct Task<T> {
    pub inner: async_task::Task<T>,  // Underlying async-task handle
    pub id: TaskId,                   // Universal task ID for tracking
    pub spawn_location: Option<&'static std::panic::Location<'static>>,  // For debugging
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TaskId(pub usize);  // Lightweight numeric ID
```

### Scheduler Trait (Low-Level Primitives)

The trait provides platform-agnostic scheduling primitives. It's `Send + Sync` to enable background scheduling.

```rust
pub trait Scheduler: Send + Sync {
    // Immediate scheduling
    fn schedule(&self, runnable: Runnable, label: Option<TaskLabel>);
    fn schedule_foreground(&self, runnable: Runnable);
    
    // Delayed scheduling
    fn schedule_at(&self, runnable: Runnable, time: Instant);
    
    // Platform integration (essential for blocking/parking threads)
    fn park(&self, timeout: Option<Duration>) -> bool;
    fn unparker(&self) -> Unparker;
    fn is_main_thread(&self) -> bool;
    
    // Time abstraction (needed for schedule_at and test determinism)
    fn now(&self) -> Instant;
}
```

- **schedule**: Background immediate execution (Send futures)
- **schedule_foreground**: Main-thread immediate execution (non-Send futures, panics if not on main thread)
- **schedule_at**: Background execution at specific time (for timers)
- **park/unparker**: Efficient thread blocking/waking
- **is_main_thread**: Safety check for foreground scheduling
- **now**: Current time (deterministic in tests)

### Extension Methods (High-Level Convenience APIs)

Convenience methods on `dyn Scheduler` for user-friendly task spawning.

```rust
impl dyn Scheduler {
    /// Spawn Send future on background thread pool
    pub fn spawn<R>(&self, future: impl Future<Output = R> + Send + 'static) -> Task<R>
    where R: Send + 'static {
        let task_id = TaskId(0); // Actual ID generated in backend
        let scheduler = self;

        let (runnable, async_task) = async_task::spawn(future, move |runnable| {
            scheduler.schedule(runnable, None);
        });

        let task = Task {
            inner: async_task,
            id: task_id,
            spawn_location: Some(std::panic::Location::caller()),
        };

        runnable.schedule();
        task
    }

    /// Spawn non-Send future on main thread
    pub fn spawn_foreground<R>(&self, future: impl Future<Output = R> + 'static) -> Task<R>
    where R: 'static {
        assert!(self.is_main_thread(), "spawn_foreground called off main thread");

        let task_id = TaskId(0); // Actual ID generated in backend
        let scheduler = self;

        let (runnable, async_task) = async_task::spawn_local(future, move |runnable| {
            scheduler.schedule_foreground(runnable);
        });

        let task = Task {
            inner: async_task,
            id: task_id,
            spawn_location: Some(std::panic::Location::caller()),
        };

        runnable.schedule();
        task
    }

    /// Timer convenience method (equivalent to GPUI's BackgroundExecutor::timer)
    pub fn timer(&self, duration: Duration) -> Task<()> {
        if duration.is_zero() {
            return Task::ready(());
        }

        let task_id = TaskId(0); // Actual ID generated in backend
        let scheduler = self;
        let target_time = self.now() + duration;

        let (runnable, async_task) = async_task::spawn(async move {}, move |runnable| {
            scheduler.schedule_at(runnable, target_time);
        });

        let task = Task {
            inner: async_task,
            id: task_id,
            spawn_location: Some(std::panic::Location::caller()),
        };

        runnable.schedule();
        task
    }
}
```

## GPUI Integration

GPUI re-exports scheduler types and adapts its executors. No API changes needed.

### BackgroundExecutor (Send + Sync)
```rust
pub struct BackgroundExecutor {
    scheduler: Arc<dyn Scheduler>,  // Send + Sync for background use
}

impl BackgroundExecutor {
    pub fn spawn<R>(&self, future: impl Future<Output = R> + Send + 'static) -> Task<R>
    where R: Send + 'static {
        self.scheduler.spawn(future)
    }

    pub fn timer(&self, duration: Duration) -> Task<()> {
        self.scheduler.timer(duration)
    }
}
```

### ForegroundExecutor (Non-Send, Main Thread Only)
```rust
pub struct ForegroundExecutor {
    scheduler: Rc<dyn Scheduler>,  // Rc prevents Send across threads
}

impl ForegroundExecutor {
    pub fn spawn<R>(&self, future: impl Future<Output = R> + 'static) -> Task<R>
    where R: 'static {
        self.scheduler.spawn_foreground(future)
    }
}
```

## Backend Implementations

### Production Backends

#### GCD Backend (macOS)
```rust
pub struct GcdScheduler {
    main_queue: dispatch_queue_t,
    background_queue: dispatch_queue_t,
    parker: Mutex<Parker>,
    unparker: Unparker,
    task_counter: AtomicUsize,  // For TaskId generation
}

impl Scheduler for GcdScheduler {
    fn schedule(&self, runnable: Runnable, _label: Option<TaskLabel>) {
        unsafe { dispatch_async_f(self.background_queue, runnable.into_raw().as_ptr() as *mut c_void, Some(trampoline)); }
    }

    fn schedule_foreground(&self, runnable: Runnable) {
        unsafe { dispatch_async_f(self.main_queue, runnable.into_raw().as_ptr() as *mut c_void, Some(trampoline)); }
    }

    fn schedule_at(&self, runnable: Runnable, time: Instant) {
        unsafe {
            let queue = dispatch_get_global_queue(DISPATCH_QUEUE_PRIORITY_HIGH.try_into().unwrap(), 0);
            let duration = time.saturating_duration_since(self.now());
            let when = dispatch_time(DISPATCH_TIME_NOW as u64, duration.as_nanos() as i64);
            dispatch_after_f(when, queue, runnable.into_raw().as_ptr() as *mut c_void, Some(trampoline));
        }
    }

    fn park(&self, timeout: Option<Duration>) -> bool { /* GCD parking implementation */ }
    fn unparker(&self) -> Unparker { self.unparker.clone() }
    fn is_main_thread(&self) -> bool { unsafe { msg_send![class!(NSThread), isMainThread] } }
    fn now(&self) -> Instant { Instant::now() }
}
```

#### ThreadPool Backend (Linux/Windows)
Similar structure using thread pools with std::thread::sleep for schedule_at.

### Testing Backends

#### Test Scheduler (GPUI-style)
Uses fake time and queues for deterministic execution.

#### Simulator Backend (Cloud-style)
Single-threaded with fake time and delay queues for Cloud testing.

## Cloud Migration

Cloud leverages the shared `Task<T>` IDs for session tracking:

```rust
#[async_trait(?Send)]
impl ExecutionContext for SimulatedExecutionContext {
    fn wait_until(&self, future: LocalBoxFuture<'static, Result<()>>) -> Result<()> {
        let task = self.scheduler.spawn(future);

        if let Some(session_id) = self.get_current_session() {
            self.wait_until_tasks.lock().unwrap()
                .entry(session_id)
                .or_insert_with(HashSet::new)
                .insert(task.id);
        }

        Ok(())
    }
}
```

## Benefits

### Performance
- **Native Integration**: GCD, thread pools, OS timers maintained
- **Zero-Copy**: Direct `Runnable` scheduling
- **Efficient Parking**: Platform-specific blocking

### Compatibility  
- **Zero Breaking Changes**: GPUI APIs unchanged
- **Incremental Migration**: Adopt piece-by-piece
- **Shared Types**: Task<T> owned by scheduler crate

### Testing & Determinism
- **Fake Time**: now() provides controllable time
- **Controlled Execution**: Test backends with predictable scheduling
- **Session Tracking**: Cloud uses Task IDs for cleanup

## Migration Strategy

### Phase 1: Create Scheduler Crate
1. Define `Task<T>`, `TaskId`, `Scheduler` trait
2. Implement extension methods
3. Add placeholder backends

### Phase 2: Implement Backends
1. GCD (macOS) scheduler
2. ThreadPool (Linux/Windows) scheduler  
3. Test and Simulator schedulers

### Phase 3: GPUI Migration
1. Re-export scheduler types
2. Adapt `BackgroundExecutor`/`ForegroundExecutor`
3. Update AppContext to use new schedulers

### Phase 4: Cloud Integration
1. Migrate to scheduler crate Task<T>
2. Update SimulatedExecutionContext to use extension methods
3. Leverage TaskIds for session cleanup

### Phase 5: Testing & Validation
1. Verify determinism in tests
2. Performance benchmarks vs. current implementation
3. API compatibility checks

## Implementation Challenges

### Platform Differences
- macOS: Grand Central Dispatch integration
- Linux/Windows: Native thread pool management
- Testing: Deterministic time and execution order

### Safety for Non-Send Futures
- ForegroundExecutor uses Rc to prevent accidental Send
- Main thread assertions in spawn_foreground
- Proper TaskId generation across backends

## Conclusion

This unified scheduler architecture provides a clean, efficient foundation for GPUI and Cloud. By centralizing scheduling in a single trait with clear separation of primitives and convenience methods, we eliminate duplication while maintaining platform performance and testing determinism.

The key insight is building timer functionality directly on the `schedule_at` primitive, avoiding runtime dependencies while ensuring main-thread safety for non-Send futures. The design supports the existing Zed codebase's heavy timer usage while providing the foundation for future observability features through universal TaskIds.

```
<file_path>
zed/thoughts/2025-08-31_15-47-17_unified-scheduler-architecture.md
</file_path>