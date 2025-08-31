# Unified Scheduler Architecture - Layered Design

## Overview

A clean layered architecture where:
- **Core**: Basic scheduling interface (`Scheduler` trait) + test-enhanced concrete impl (`TestScheduler`)
- **GPUI**: Uses trait objects for production safety, test features via `TestScheduler`
- **Cloud**: Session wrapper uses `TestScheduler` for session coordination

Key design principles:
- Main `Scheduler` trait has only essential methods (no test pollution)
- Test-specific features (deprioritization, task tracking) are `TestScheduler`-specific
- Production schedulers implement minimal interface
- Cloud requires `TestScheduler` for session features

## Core Architecture

```
┌─────────────────────────────────────────┐
│            Shared Crate                 │
├─────────────────────────────────────────┤
│ Scheduler trait:                        │
│ - spawn() - core Send futures           │
│ - spawn_foreground() - non-Send         │
│ - Platform integration (park, now)      │
│                                         │
│ TestScheduler:                          │
│ - Implements Scheduler + test features  │
│ - deprioritize() - test-only method     │
│ - spawn_labeled() - labels for testing  │
│                                         │
│ GcdScheduler/ThreadPoolScheduler:       │
│ - Minimal Scheduler implementations     │
└─────────────────────────────────────────┘
                    ▲
          ┌─────────┼─────────┐
          │         │         │
┌─────────┴────┐  ┌─┴─────────┴────┐
│   GPUI       │  │     Cloud       │
│ Uses trait   │  │ CloudSimulated  │
│ objects      │  │ uses Test-     │
│ + TestScheduler│  │ Scheduler     │
└──────────────┘  └─────────────────┘
```

## Scheduler Trait Definition

```rust
pub trait Scheduler: Send + Sync {
    /// Spawn Send future (core functionality)
    fn spawn<R>(&self, future: impl Future<Output = R> + Send + 'static) -> Task<R>;

    /// Spawn Send future with label (defaults to ignoring label)
    fn spawn_labeled<R>(
        &self,
        _label: TaskLabel,
        future: impl Future<Output = R> + Send + 'static
    ) -> Task<R>
    where R: Send + 'static {
        // Default: ignore label and just spawn normally
        self.spawn(future)
    }

    /// Spawn non-Send future on main thread (optional, defaults to panic)
    fn spawn_foreground<R>(&self, future: impl Future<Output = R> + 'static) -> Task<R>
    where R: 'static {
        panic!("spawn_foreground not supported by this scheduler");
    }

    /// Platform integration methods
    fn park(&self, timeout: Option<Duration>) -> bool { false }
    fn unparker(&self) -> Unparker { Arc::new(|_| {}).into() }
    fn is_main_thread(&self) -> bool;
    fn now(&self) -> Instant;
}
```

**Explanation:**
- Core trait has only essential methods
- No test-specific methods (deprioritize stays off main trait)
- Production schedulers implement minimal interface
- Test features are concrete `TestScheduler` methods

## Task<T> Definition

```rust
#[derive(Debug)]
pub struct Task<T> {
    inner: TaskState<T>,
    id: TaskId,  // Mandatory for coordination
    metadata: TaskMetadata,
}

#[derive(Debug)]
pub struct TaskMetadata {
    label: Option<TaskLabel>,        // GPUI test identification
    session: Option<SessionId>,      // Cloud session association
    spawn_location: Option<&'static std::panic::Location>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TaskId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TaskLabel(NonZeroUsize);
```

**Explanation:**
- Mandatory TaskId for session coordination
- Optional metadata for GPUI labels and Cloud sessions
- Task implements Future directly

## TestScheduler (Concrete Implementation)

```rust
pub struct TestScheduler {
    inner: RefCell<TestSchedulerInner>,
}

struct TestSchedulerInner {
    tasks: HashMap<TaskId, TaskState>,
    deprioritized_labels: HashSet<TaskLabel>,  // Test-specific state
    delayed: Vec<(Instant, Runnable)>,
    parker: Parker,
    is_main_thread: bool,
    now: Instant,
    next_task_id: AtomicUsize,
}

impl Scheduler for TestScheduler {
    fn spawn<R>(&self, future: impl Future<Output = R> + Send + 'static) -> Task<R> {
        let task_id = TaskId(self.next_task_id.fetch_add(1, Ordering::SeqCst));
        let task = self.create_task(future, task_id);
        self.inner.borrow_mut().tasks.insert(task_id, TaskState::Running);
        task
    }

    fn spawn_foreground<R>(&self, future: impl Future<Output = R> + 'static) -> Task<R> {
        assert!(self.is_main_thread(), "spawn_foreground called off main thread");
        let task_id = TaskId(self.next_task_id.fetch_add(1, Ordering::SeqCst));
        let task = self.create_local_task(future, task_id);
        self.inner.borrow_mut().tasks.insert(task_id, TaskState::Running);
        task
    }

    fn is_main_thread(&self) -> bool { self.inner.borrow().is_main_thread }
    fn now(&self) -> Instant { self.inner.borrow().now }
    fn park(&self, timeout: Option<Duration>) -> bool {
        self.inner.borrow().parker.park_timeout(timeout.unwrap_or(Duration::MAX))
    }
    fn spawn_labeled<R>(
        &self,
        label: TaskLabel,
        future: impl Future<Output = R> + Send + 'static
    ) -> Task<R>
    where R: Send + 'static {
        let task_id = TaskId(self.next_task_id.fetch_add(1, Ordering::SeqCst));
        let task = self.create_task_with_label(future, task_id, label);

        // Apply deprioritization if label is registered
        if self.inner.borrow().deprioritized_labels.contains(&label) {
            self.move_to_deprioritized_queue(task_id);
        }

        self.inner.borrow_mut().tasks.insert(task_id, TaskState::Running);
        task
    }

    fn unparker(&self) -> Unparker {
        self.inner.borrow().parker.unparker()
    }
}

// Test-specific methods (NOT on main trait)
impl TestScheduler {
    pub fn deprioritize(&self, label: TaskLabel) {
        self.inner.borrow_mut().deprioritized_labels.insert(label);
    }

    pub fn spawn_labeled<R>(
        &self,
        label: TaskLabel,
        future: impl Future<Output = R> + Send + 'static
    ) -> Task<R> {
        let task_id = TaskId(self.next_task_id.fetch_add(1, Ordering::SeqCst));
        let mut task = self.create_task(future, task_id);
        task.metadata.label = Some(label);  // Set label in metadata

        // Apply deprioritization if label is registered
        if self.inner.borrow().deprioritized_labels.contains(&label) {
            self.move_to_deprioritized_queue(task_id);
        }

        self.inner.borrow_mut().tasks.insert(task_id, TaskState::Running);
        task
    }

    pub fn is_task_running(&self, task_id: TaskId) -> bool {
        self.inner.borrow().tasks.contains_key(&task_id)
    }

    fn create_task<R>(&self, future: impl Future<Output = R> + Send + 'static, task_id: TaskId) -> Task<R> {
        let (runnable, inner_task) = async_task::spawn(future, move |runnable| {
            // Schedule to appropriate queue based on label
            self.schedule_runnable(runnable, task_id);
        });
        runnable.schedule();

        Task {
            inner: TaskState::Spawned(inner_task),
            id: task_id,
            metadata: TaskMetadata {
                label: None,
                session: None,
                spawn_location: Some(std::panic::Location::caller()),
            },
        }
    }

    fn create_task_with_label<R>(&self, future: impl Future<Output = R> + Send + 'static, task_id: TaskId, label: TaskLabel) -> Task<R>
    where R: Send + 'static {
        let (runnable, inner_task) = async_task::spawn(future, move |runnable| {
            self.schedule_runnable_with_label(runnable, task_id, label);
        });
        runnable.schedule();

        Task {
            inner: TaskState::Spawned(inner_task),
            id: task_id,
            metadata: TaskMetadata {
                label: Some(label),
                session: None,
                spawn_location: Some(std::panic::Location::caller()),
            },
        }
    }

    fn schedule_runnable_with_label(&self, runnable: Runnable, task_id: TaskId, label: TaskLabel) {
        // TestScheduler-specific scheduling logic for labeled tasks
        if self.inner.borrow().deprioritized_labels.contains(&label) {
            // Put in deprioritized queue for test determinism
            self.inner.borrow_mut().deprioritized_queue.push(runnable);
        } else {
            // Schedule normally
            runnable.schedule();
        }
    }
}
```

**Explanation:**
- `deprioritize()` is a TestScheduler-specific method (not on main trait)
- `spawn_labeled()` is TestScheduler-specific (not on main trait)
- `is_task_running()` provides task status for Cloud session validation
- Test-specific state stays in TestScheduler

## Production Schedulers

```rust
pub struct GcdScheduler {
    main_queue: dispatch_queue_t,
    background_queue: dispatch_queue_t,
    task_counter: AtomicUsize,
}

impl Scheduler for GcdScheduler {
    fn spawn<R>(&self, future: impl Future<Output = R> + Send + 'static) -> Task<R> {
        let task_id = TaskId(self.task_counter.fetch_add(1, Ordering::SeqCst));
        let (runnable, task) = async_task::spawn(future, move |runnable| {
            unsafe { dispatch_async_f(self.background_queue, runnable.into_raw().as_ptr() as *mut c_void, Some(trampoline)); }
        });
        runnable.schedule();

        Task {
            inner: TaskState::Spawned(task),
            id: task_id,
            metadata: TaskMetadata::default(),
        }
    }

    fn is_main_thread(&self) -> bool {
        // macOS-specific main thread detection
        unsafe { msg_send![class!(NSThread), isMainThread] }
    }

    fn now(&self) -> Instant { Instant::now() }
}
```

**Explanation:**
- Production schedulers implement only core `Scheduler` trait
- No test-specific methods (deprioritize stays off main trait)
- Minimal implementation, no task tracking overhead

## GPUI Integration

```rust
// BackgroundExecutor uses trait objects (production-safe)
pub struct BackgroundExecutor {
    scheduler: Arc<dyn Scheduler>,  // Any Scheduler implementation
}

impl BackgroundExecutor {
    pub fn new(scheduler: Arc<dyn Scheduler>) -> Self {
        Self { scheduler }
    }

    pub fn spawn<R>(&self, future: impl Future<Output = R> + Send + 'static) -> Task<R>
    where R: Send + 'static {
        self.scheduler.spawn(future)
    }

    pub fn spawn_labeled<R>(
        &self,
        label: TaskLabel,
        future: impl Future<Output = R> + Send + 'static
    ) -> Task<R>
    where R: Send + 'static {
        self.scheduler.spawn_labeled(label, future)
    }

    // When GPUI needs test features, it downcasts to TestScheduler
    pub fn deprioritize(&self, label: TaskLabel) {
        // Downcast to access TestScheduler-specific features
        if let Some(test_scheduler) = self.scheduler.downcast_ref::<TestScheduler>() {
            test_scheduler.deprioritize(label);
        } else {
            // Production: do nothing
        }
    }
}

// ForegroundExecutor also uses trait objects
pub struct ForegroundExecutor {
    scheduler: Rc<dyn Scheduler>,
}

impl ForegroundExecutor {
    pub fn spawn<R>(&self, future: impl Future<Output = R> + 'static) -> Task<R>
    where R: 'static {
        self.scheduler.spawn_foreground(future)
    }
}
```

**Explanation:**
- GPUI executors use trait objects for production safety
- Test features accessed via downcasting to TestScheduler
- Production deployments can use minimal schedulers
- Test deployments get full test features

## Cloud Integration

```rust
// Cloud wrapper requires TestScheduler for session features
pub struct CloudSimulatedScheduler {
    test_scheduler: Arc<TestScheduler>,  // Concrete type for test features
    inner: RefCell<CloudSimulatedSchedulerInner>,
}

struct CloudSimulatedSchedulerInner {
    current_session: Option<SessionId>,
    sessions: HashMap<SessionId, SessionData>,
    task_to_session: HashMap<TaskId, SessionId>,
}

impl CloudSimulatedScheduler {
    pub fn spawn<R>(&self, future: impl Future<Output = R> + Send + 'static) -> Task<R> {
        let task = self.test_scheduler.spawn(future);

        // Auto-associate with current session
        if let Some(session_id) = self.inner.borrow().current_session {
            self.inner.borrow_mut().task_to_session.insert(task.id(), session_id);
            // Track in session...
        }

        task
    }

    pub fn validate_session_cleanup(&self, session_id: SessionId) -> Result<()> {
        // Use TestScheduler's task tracking for validation
        let inner = self.inner.borrow();

        if let Some(session) = inner.sessions.get(&session_id) {
            let running_tasks: Vec<TaskId> = session
                .spawned_tasks
                .iter()
                .filter(|&&task_id| self.test_scheduler.is_task_running(task_id))
                .copied()
                .collect();

            // Check against explicit wait_until permissions
            let unauthorized = running_tasks.difference(&session.wait_until_task_ids);

            if unauthorized.next().is_some() {
                return Err(anyhow!("Session cleanup failed: unauthorized tasks still running"));
            }
        }

        Ok(())
    }
}
```

**Explanation:**
- Cloud requires `TestScheduler` because session features need task tracking
- Auto-associates tasks with current session
- Uses TestScheduler's `is_task_running()` for validation
- Session coordination is test-focused infrastructure

## Migration Strategy

### Phase 1: Core Infrastructure
1. Define `Scheduler` trait (core methods only)
2. Implement `TestScheduler` (with test features like `deprioritize()`)
3. Implement production schedulers (GCD, ThreadPool)
4. Define `Task<T>` with mandatory TaskId

### Phase 2: GPUI Migration
1. Update GPUI executors to use trait objects
2. Add downcasting for test features
3. Preserve all existing GPUI functionality
4. Test deployments use TestScheduler, production uses minimal schedulers

### Phase 3: Cloud Integration
1. Cloud wrapper uses TestScheduler for session coordination
2. Maintain automatic session association
3. Preserve `wait_until` and validation behavior
4. Application code unchanged

### Phase 4: Validation
1. GPUI tests work with new architecture
2. Cloud session behavior preserved
3. Production efficiency maintained

## Benefits

✅ **Clean Separation**: Test methods only on TestScheduler
✅ **Production Safety**: GPUI executors use trait objects
✅ **Session Intelligence**: Cloud gets full coordination features
✅ **Flexible Architecture**: Production vs test deployments
✅ **Backward Compatibility**: All existing functionality preserved

This design keeps test concerns in TestScheduler while maintaining production safety and session coordination capabilities.

## Implementation Reference

### GPUI File Paths & Types

**Core Executor Files:**
- `crates/gpui/src/executor.rs` - `BackgroundExecutor`, `ForegroundExecutor`, `Task<T>`
- `crates/gpui/src/app/app.rs` - App-level executor access
- `crates/gpui/src/platform.rs` - `PlatformDispatcher` trait (current system)

**Types to Update:**
- `BackgroundExecutor` - Switch from `PlatformDispatcher` to `Arc<dyn Scheduler>`
- `ForegroundExecutor` - Switch from `PlatformDispatcher` to `Rc<dyn Scheduler>`
- `Task<T>` - Ensure compatibility with new `Task<T>` design

**Test Infrastructure:**
- `crates/gpui/src/platform/test/dispatcher.rs` - `TestDispatcher` (current)
- Will need new `TestScheduler` implementation
- `TaskLabel` usage in tests

### Cloud File Paths & Types

**Core Runtime Files:**
- `crates/platform_simulator/src/runtime.rs` - `SimulatorRuntime`, session management
- `crates/platform_simulator/src/platform.rs` - `SimulatedExecutionContext`
- `crates/platform_simulator/src/lib.rs` - Cloud worker setup

**Key Types:**
- `SessionId` - Session identification
- `WorkerSession` - Session state tracking
- `ExecutionContext::wait_until()` - Session coordination API
- `SimulatorRuntime::validate_session_cleanup()` - Cleanup validation

**Worker Files:**
- `crates/client_api/src/client_api.rs` - API endpoints using sessions
- `crates/cloud_worker/src/worker.rs` - Worker execution with sessions
- `crates/cloudflare_platform/src/execution_context.rs` - Platform-specific execution context

### Migration Points

**GPUI Areas:**
- Replace `cx.background_executor()` calls with new executors
- Update any direct `PlatformDispatcher` usage
- Preserve `spawn_labeled()` and `deprioritize()` APIs

**Cloud Areas:**
- Replace `SimulatorRuntime` usage in tests
- Update session management to use new scheduler wrapper
- Preserve `wait_until()` and session validation behavior

### Test Files Impacted

**GPUI Tests:**
- `crates/gpui/src/app/test_context.rs` - Test setup
- `crates/gpui_macros/src/test.rs` - Test macro generation
- Project-specific test files using `deprioritize()`

**Cloud Tests:**
- `crates/platform_simulator/src/lib.rs` - Test setup
- Worker test files using session features
- Session validation test cases

### Platform Backend Files

**macOS:**
- `crates/gpui/src/platform/mac/dispatcher.rs` - `MacDispatcher` → `GcdScheduler`

**Linux:**
- `crates/gpui/src/platform/linux/dispatcher.rs` - `LinuxDispatcher` → `ThreadPoolScheduler`

**Windows:**
- `crates/gpui/src/platform/windows/dispatcher.rs` - `WindowsDispatcher` → `ThreadPoolScheduler`

**Test:**
- `crates/gpui/src/platform/test/dispatcher.rs` - `TestDispatcher` → `TestScheduler`

## Compatibility Checklist

### GPUI Compatibility
- ✅ `spawn()` → `scheduler.spawn()`
- ✅ `spawn_labeled(label)` → `scheduler.spawn_labeled(label)`
- ✅ `deprioritize()` → Downcast to TestScheduler
- ✅ `timer()` → `scheduler.timer()`
- ✅ `BackgroundExecutor` → Trait object wrapper

### Cloud Compatibility
- ✅ `ExecutionContext::wait_until()` → Scheduler wrapper
- ✅ Session validation → `validate_session_cleanup()`
- ✅ Automatic session association → Wrapper intelligence
- ✅ Task cleanup checking → TestScheduler task tracking
- ✅ `spawn()` in sessions → Auto-association

### Test Compatibility
- ✅ Test determinism → TestScheduler deprioritization
- ✅ Task labeling → TestScheduler spawn_labeled override
- ✅ Session coordination → Cloud wrapper
- ✅ Production efficiency → Minimal scheduler implementations

## Next Steps

1. **Create shared scheduler crate** with core types
2. **Implement TestScheduler** with task tracking and test features
3. **Update GPUI executors** to use trait objects
4. **Create Cloud wrapper** with session coordination
5. **Migrate platform backends** to new scheduler implementations
6. **Update tests** to use new architecture
7. **Validate performance** and backward compatibility
