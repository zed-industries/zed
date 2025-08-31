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
    /// Schedule a runnable to be executed (object-safe core functionality)
    fn schedule(&self, runnable: Runnable);

    /// Schedule a runnable with label for test tracking
    fn schedule_labeled(&self, runnable: Runnable, label: TaskLabel);

    /// Schedule a runnable on the main thread (optional, defaults to panic)
    fn schedule_foreground(&self, runnable: Runnable) {
        panic!("schedule_foreground not supported by this scheduler");
    }

    /// Platform integration methods
    fn park(&self, timeout: Option<Duration>) -> bool { false }
    fn unparker(&self) -> Unparker { Arc::new(|_| {}).into() }
    fn is_main_thread(&self) -> bool;
    fn now(&self) -> Instant;
}
```

**Explanation:**
- Core trait methods are object-safe (no generic parameters)
- `schedule` methods operate on `Runnable` for low-level execution control
- Scheduler implementations manage task state internally when scheduling runnables
- No task completion hooks needed on `Task` - scheduler tracks running tasks itself

## Generic Spawn Helpers

Generic spawn methods are implemented for `dyn Scheduler` to provide the high-level `Future` interface:

```rust
impl dyn Scheduler {
    /// Spawn Send future (generic helper)
    pub fn spawn<R>(&self, future: impl Future<Output = R> + Send + 'static) -> Task<R>
    where R: Send + 'static {
        let task_id = self.assign_task_id();
        let task_metadata = TaskMetadata {
            id: task_id,
            label: None,
            session: None,
            spawn_location: std::panic::Location::caller(),
        };

        let (runnable, inner_task) = async_task::spawn(future, move |runnable| {
            // Scheduler manages task lifecycle: mark as started when scheduled
            self.mark_task_started(task_id);
            // When runnable completes, scheduler marks as finished
            self.schedule_completion_callback(runnable, task_id);
        });

        // Schedule the runnable (this adds to running tasks)
        self.schedule(runnable);

        Task {
            inner: TaskState::Spawned(inner_task),
            metadata: task_metadata,
        }
    }

    /// Spawn Send future with label (generic helper)
    pub fn spawn_labeled<R>(
        &self,
        label: TaskLabel,
        future: impl Future<Output = R> + Send + 'static
    ) -> Task<R>
    where R: Send + 'static {
        let task_id = self.assign_task_id();
        let task_metadata = TaskMetadata {
            id: task_id,
            label: Some(label),
            session: None,
            spawn_location: std::panic::Location::caller(),
        };

        let (runnable, inner_task) = async_task::spawn(future, move |runnable| {
            self.mark_task_started(task_id);
            self.schedule_completion_callback(runnable, task_id);
        });

        // Apply test-specific logic (e.g., deprioritization) in scheduler
        self.schedule_labeled(runnable, label);

        Task {
            inner: TaskState::Spawned(inner_task),
            metadata: task_metadata,
        }
    }

    /// Spawn non-Send future on main thread (generic helper)
    pub fn spawn_foreground<R>(&self, future: impl Future<Output = R> + 'static) -> Task<R>
    where R: 'static {
        let task_id = self.assign_task_id();
        let task_metadata = TaskMetadata {
            id: task_id,
            label: None,
            session: None,
            spawn_location: std::panic::Location::caller(),
        };

        let (runnable, inner_task) = async_task::spawn_local(future, move |runnable| {
            self.mark_task_started(task_id);
            self.schedule_completion_callback(runnable, task_id);
        });

        self.schedule_foreground(runnable);

        Task {
            inner: TaskState::Spawned(inner_task),
            metadata: task_metadata,
        }
    }
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
    task_labels: HashMap<TaskId, TaskLabel>,
    deprioritized_labels: HashSet<TaskLabel>,  // Test-specific state
    deprioritized_queue: VecDeque<(Runnable, TaskId)>,
    main_thread_queue: VecDeque<Runnable>,
    delayed: Vec<(Instant, Runnable)>,
    parker: Parker,
    is_main_thread: bool,
    now: Instant,
    next_task_id: AtomicUsize,
}

impl Scheduler for TestScheduler {
    fn schedule(&self, runnable: Runnable) {
        let task_id = self.next_task_id.fetch_add(1, Ordering::SeqCst);
        self.inner.borrow_mut().tasks.insert(task_id, TaskState::Running);

        // Schedule the runnable and setup completion callback
        let scheduler = self.clone();
        let completion_runnable = self.create_completion_runnable(runnable, task_id);
        completion_runnable.schedule();
    }

    fn schedule_labeled(&self, runnable: Runnable, label: TaskLabel) {
        let task_id = self.next_task_id.fetch_add(1, Ordering::SeqCst);

        // Apply deprioritization if label is registered
        if self.inner.borrow().deprioritized_labels.contains(&label) {
            // Store label association and put in deprioritized queue
            self.inner.borrow_mut().deprioritized_queue.push((runnable, task_id));
            self.inner.borrow_mut().task_labels.insert(task_id, label);
        } else {
            self.inner.borrow_mut().tasks.insert(task_id, TaskState::Running);
            let completion_runnable = self.create_completion_runnable(runnable, task_id);
            completion_runnable.schedule();
        }
    }

    fn schedule_foreground(&self, runnable: Runnable) {
        assert!(self.is_main_thread(), "schedule_foreground called off main thread");
        let task_id = self.next_task_id.fetch_add(1, Ordering::SeqCst);
        self.inner.borrow_mut().tasks.insert(task_id, TaskState::Running);

        let completion_runnable = self.create_completion_runnable(runnable, task_id);
        // Schedule on main thread queue
        self.inner.borrow_mut().main_thread_queue.push(completion_runnable);
    }

    fn is_main_thread(&self) -> bool { self.inner.borrow().is_main_thread }
    fn now(&self) -> Instant { self.inner.borrow().now }
    fn park(&self, timeout: Option<Duration>) -> bool {
        self.inner.borrow().parker.park_timeout(timeout.unwrap_or(Duration::MAX))
    }

    fn unparker(&self) -> Unparker {
        self.inner.borrow().parker.unparker()
    }
}

impl TestScheduler {
    fn assign_task_id(&self) -> TaskId {
        TaskId(self.next_task_id.fetch_add(1, Ordering::SeqCst))
    }

    fn mark_task_started(&self, task_id: TaskId) {
        // Task already marked running in schedule methods
    }

    fn schedule_completion_callback(&self, runnable: Runnable, task_id: TaskId) -> Runnable {
        let scheduler = self.clone();
        async_task::spawn(async move {
            // Run the original runnable
            runnable.schedule();
            // Mark task as completed when done
            scheduler.mark_task_completed(task_id);
        }, |_| {}).0
    }

    fn mark_task_completed(&self, task_id: TaskId) {
        self.inner.borrow_mut().tasks.remove(&task_id);
    }

    fn create_completion_runnable(&self, runnable: Runnable, task_id: TaskId) -> Runnable {
        let scheduler = self.clone();
        async_task::spawn(async move {
            runnable.schedule();
            scheduler.mark_task_completed(task_id);
        }, |_| {}).0
    }
}

// Test-specific methods (NOT on main trait)
impl TestScheduler {
    pub fn deprioritize(&self, label: TaskLabel) {
        self.inner.borrow_mut().deprioritized_labels.insert(label);
    }

    pub fn is_task_running(&self, task_id: TaskId) -> bool {
        self.inner.borrow().tasks.contains_key(&task_id)
    }

    // Additional internal methods for task lifecycle management
    fn move_to_deprioritized_queue(&self, task_id: TaskId) {
        // Move task to deprioritized queue for deterministic testing
        // This is called from deprioritize to move already scheduled tasks
        if let Some(runnable) = self.inner.borrow_mut().tasks.remove(&task_id) {
            self.inner.borrow_mut().deprioritized_queue.push_back((runnable, task_id));
        }
    }
}

    // Task creation now handled by generic spawn helpers
    // Runnable scheduling managed internally by schedule methods
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
}

impl Scheduler for GcdScheduler {
    fn schedule(&self, runnable: Runnable) {
        unsafe {
            dispatch_async_f(self.background_queue, runnable.into_raw().as_ptr() as *mut c_void, Some(trampoline));
        }
    }

    fn schedule_labeled(&self, runnable: Runnable, _label: TaskLabel) {
        // Production scheduler ignores labels
        unsafe {
            dispatch_async_f(self.background_queue, runnable.into_raw().as_ptr() as *mut c_void, Some(trampoline));
        }
    }

    fn schedule_foreground(&self, runnable: Runnable) {
        unsafe {
            dispatch_async_f(self.main_queue, runnable.into_raw().as_ptr() as *mut c_void, Some(trampoline));
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
- Production schedulers implement object-safe `Scheduler` trait
- No test-specific features or task state tracking
- Minimal implementation with direct dispatch to GCD queues
- Test features only available via `TestScheduler` wrapper in GPUI

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
        // Generic spawn helper implemented on dyn Scheduler
        self.scheduler.spawn(future)
    }

    pub fn spawn_labeled<R>(
        &self,
        label: TaskLabel,
        future: impl Future<Output = R> + Send + 'static
    ) -> Task<R>
    where R: Send + 'static {
        // Generic spawn_labeled helper implemented on dyn Scheduler
        self.scheduler.spawn_labeled(label, future)
    }

    // When GPUI needs test features, it downcasts to TestScheduler
    pub fn deprioritize(&self, label: TaskLabel) {
        // Downcast to access TestScheduler-specific features
        if let Some(test_scheduler) = self.scheduler.downcast_ref::<TestScheduler>() {
            test_scheduler.deprioritize(label);
        } else {
            // Production: do nothing (ignore test-only calls)
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
        // Generic spawn_foreground helper implemented on dyn Scheduler
        self.scheduler.spawn_foreground(future)
    }
}

**Explanation:**
- GPUI executors use trait objects for production safety and object-safe `Scheduler` trait
- Generic spawn helpers provide the familiar Future-based API on `dyn Scheduler`
- Object-safe schedule methods allow trait object usage without downcasting for basic operations
- Test features still require downcasting to `TestScheduler` for deprioritization
- Production deployments can use minimal schedulers via trait objects
- Test deployments get full test features through TestScheduler wrapper

## Cloud Integration

```rust
// Cloud wrapper requires TestScheduler for session features and task tracking
pub struct CloudSimulatedScheduler {
    scheduler: Arc<dyn Scheduler>,  // Object-safe scheduler (usually TestScheduler)
    inner: RefCell<CloudSimulatedSchedulerInner>,
}

struct CloudSimulatedSchedulerInner {
    current_session: Option<SessionId>,
    sessions: HashMap<SessionId, SessionData>,
    task_to_session: HashMap<TaskId, SessionId>,
}

impl CloudSimulatedScheduler {
    pub fn new(scheduler: Arc<dyn Scheduler>) -> Self {
        Self {
            scheduler,
            inner: RefCell::new(CloudSimulatedSchedulerInner {
                current_session: None,
                sessions: HashMap::new(),
                task_to_session: HashMap::new(),
            }),
        }
    }

    // Use generic spawn helpers with session tracking
    pub fn spawn<R>(&self, future: impl Future<Output = R> + Send + 'static) -> Task<R>
    where R: Send + 'static {
        // Get task from generic spawn helper (includes task_id assignment)
        let task = self.scheduler.spawn(future);

        // Auto-associate with current session
        if let Some(session_id) = self.inner.borrow().current_session.clone() {
            self.inner.borrow_mut().task_to_session.insert(task.metadata.id, session_id.clone());
            // Track spawned task in session
            if let Some(session) = self.inner.borrow_mut().sessions.get_mut(&session_id) {
                session.spawned_tasks.push(task.metadata.id);
            }
        }

        task
    }

    pub fn spawn_labeled<R>(
        &self,
        label: TaskLabel,
        future: impl Future<Output = R> + Send + 'static
    ) -> Task<R>
    where R: Send + 'static {
        // Use generic spawn_labeled helper
        let task = self.scheduler.spawn_labeled(label, future);

        // Auto-associate with current session
        if let Some(session_id) = self.inner.borrow().current_session.clone() {
            self.inner.borrow_mut().task_to_session.insert(task.metadata.id, session_id.clone());
            // Track spawned task in session
            if let Some(session) = self.inner.borrow_mut().sessions.get_mut(&session_id) {
                session.spawned_tasks.push(task.metadata.id);
            }
        }

        task
    }

    pub fn validate_session_cleanup(&self, session_id: SessionId) -> Result<()> {
        // Use TestScheduler's internal task tracking for validation
        if let Some(test_scheduler) = self.scheduler.downcast_ref::<TestScheduler>() {
            let inner = self.inner.borrow();

            if let Some(session) = inner.sessions.get(&session_id) {
                let running_tasks: Vec<TaskId> = session
                    .spawned_tasks
                    .iter()
                    .filter(|&&task_id| test_scheduler.is_task_running(task_id))
                    .copied()
                    .collect();

                // Check against explicit wait_until permissions
                let unauthorized = running_tasks.difference(&session.wait_until_task_ids);

                if unauthorized.next().is_some() {
                    return Err(anyhow!("Session cleanup failed: unauthorized tasks still running"));
                }
            }
        } else {
            // Production scheduler: no task tracking available
            return Err(anyhow!("Session validation requires TestScheduler"));
        }

        Ok(())
    }

    // Session management methods
    pub fn create_session(&self) -> SessionId {
        let session_id = SessionId::new();
        self.inner.borrow_mut().sessions.insert(session_id.clone(), SessionData {
            spawned_tasks: Vec::new(),
            wait_until_task_ids: HashSet::new(),
        });
        self.inner.borrow_mut().current_session = Some(session_id.clone());
        session_id
    }

    pub fn add_wait_until_task(&self, session_id: SessionId, task_id: TaskId) {
        if let Some(session) = self.inner.borrow_mut().sessions.get_mut(&session_id) {
            session.wait_until_task_ids.insert(task_id);
        }
    }
}
```

**Explanation:**
- Cloud wrapper uses object-safe `Scheduler` trait with generic spawn helpers
- Internal task management: scheduler tracks running tasks, Cloud wrapper associates with sessions
- Session tracking enhanced: tasks automatically associated via spawn helpers and metadata
- Task lifecycle: scheduler manages completion internally, Cloud validates against running tasks
- Test features: downcast to TestScheduler for `is_task_running()` validation
- Production safety: uses trait objects, but session features require TestScheduler

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

✅ **Object-Safe Trait**: Scheduler trait is object-safe, enabling trait objects without downcasting for core operations
✅ **Internal Task Management**: Scheduler manages task lifecycle and completion state internally, providing unified task tracking
✅ **Clean Separation**: Test methods only on TestScheduler, generic spawn helpers on trait objects
✅ **Production Safety**: GPUI executors use trait objects with minimal dyn dispatch overhead
✅ **Session Intelligence**: Cloud gets full coordination features with automatic task-session association via spawn helpers
✅ **Flexible Architecture**: Production vs test deployments with scheduler implementations optimized for each context
✅ **Backward Compatibility**: All existing functionality preserved via generic spawn helpers on `dyn Scheduler`

This design keeps test concerns in TestScheduler while maintaining production safety and session coordination capabilities through internal scheduler task management.

### Benefits of This Approach

✅ **Interface Compatibility**: GPUI code continues using Future-based spawn, Cloud uses session-aware wrappers
✅ **Performance**: Trait objects have minimal overhead, direct Runnable scheduling in production
✅ **Separation of Concerns**: Low-level Runnable scheduling in trait, high-level Future API as helpers
✅ **Object Safety**: Enables `Arc<dyn Scheduler>` usage without runtime downcasting for basic operations

### Migration Impact

- GPUI executors: Simple switch from `Arc<dyn PlatformDispatcher>` to `Arc<dyn Scheduler>`
- Cloud wrapper: Enhanced to automatically associate tasks with sessions via spawn helpers
- Test infrastructure: TestScheduler provides both low-level scheduling and task tracking internally

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
- Update GPUI executors to use `Arc<dyn Scheduler>` trait objects
- Replace `PlatformDispatcher` usage with object-safe `Scheduler` methods and generic spawn helpers
- Preserve `spawn_labeled()` and `deprioritize()` APIs via generic helpers and downcasting
- Update `BackgroundExecutor` and `ForegroundExecutor` to call `dyn Scheduler` spawn helpers

**Cloud Areas:**
- Replace `SimulatorRuntime` with `CloudSimulatedScheduler` wrapper around `dyn Scheduler`
- Implement session management using wrapper's spawn helpers with automatic task association
- Preserve `wait_until()` and session validation via downcast to TestScheduler for task tracking
- Update `ExecutionContext` implementation to use new wrapper

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
- ✅ `spawn()` → `dyn Scheduler::spawn()` (generic helper on trait object)
- ✅ `spawn_labeled(label)` → `dyn Scheduler::spawn_labeled()` (generic helper on trait object)
- ✅ `deprioritize()` → Downcast to TestScheduler, then `TestScheduler::deprioritize()`
- ✅ `timer()` → `scheduler.timer()` (platform method on trait object)
- ✅ `BackgroundExecutor` → Trait object wrapper using `dyn Scheduler`

### Cloud Compatibility
- ✅ `ExecutionContext::wait_until()` → Scheduler wrapper with generic spawn helpers
- ✅ Session validation → `validate_session_cleanup()` with downcast to TestScheduler
- ✅ Automatic session association → Via spawn helpers and task metadata
- ✅ Task cleanup checking → Internal scheduler task tracking (downcast to TestScheduler for running status)
- ✅ `spawn()` in sessions → `dyn Scheduler::spawn()` with auto-association in wrapper

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
