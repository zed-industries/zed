# Unified Scheduler Architecture - Layered Design

## Overview

A clean layered architecture where:
- **Core**: Basic scheduling interface (`Scheduler` trait) + test-enhanced concrete impl (`TestScheduler`)
- **GPUI**: Uses trait objects for production safety, test features via `TestScheduler`
- **Cloud**: Session coordination integrated directly in `SimulatedExecutionContext` using unified scheduler primitives

Key design principles:
- Main `Scheduler` trait has only essential methods (no test pollution)
- Test-specific features (deprioritization, task tracking) are `TestScheduler`-specific
- Production schedulers implement minimal interface
- Cloud handles session coordination at ExecutionContext layer using unified primitives

## Core Architecture

```
┌─────────────────────────────────────────┐
│            Shared Crate                 │
├─────────────────────────────────────────┤
│ Scheduler trait:                        │
│ - Core object-safe interface             │
│ - Platform integration (park, now)      │
│                                         │
│ TestScheduler:                          │
│ - Should implement Scheduler + test features  │
│ - deprioritize() - test-only method     │
│ - spawn_labeled() - labels for testing  │
│ - Task lifecycle tracking               │
│                                         │
│ Generic spawn helpers:                  │
│ - spawn() / spawn_foreground()          │
│ - timer(), block(), block_with_timeout()│
│ - Future-based API for trait objects    │
└─────────────────────────────────────────┘
                    ▲
          ┌─────────┼─────────┐
          │         │         │
┌─────────┴────┐  ┌─┴─────────┴────┐
│   GPUI       │  │     Cloud       │
│ Uses trait   │  │ Session coord. │
│ objects      │  │ in ExecContext │
│ + TestScheduler│  │ + TestScheduler│
└──────────────┘  └─────────────────┘
```

## GPUI Integration

### Platform Scheduler Implementations in GPUI

Platform-specific scheduler implementations **should remain in GPUI's platform modules**:

- **MacDispatcher**: Should implement `Scheduler` trait, uses GCD APIs
- **LinuxDispatcher**: Should implement `Scheduler` trait, uses thread pools + calloop
- **WindowsDispatcher**: Should implement `Scheduler` trait, uses Windows ThreadPool APIs

**Rationale**: Platform implementations are substantial and deeply integrated with:
- Platform-specific threading APIs (GCD, Windows ThreadPool, etc.)
- GPUI's event loop integration (main thread messaging)
- Platform-specific performance optimizations

The shared crate provides only the trait definition and generic helpers, while platform-specific dispatchers implement the `Scheduler` trait directly in GPUI.

### BackgroundExecutor Integration

GPUI's executors will use trait objects for scheduling:

```rust
// crates/gpui/src/executor.rs
pub struct BackgroundExecutor {
    scheduler: Arc<dyn Scheduler>,  // Any Scheduler implementation via trait objects
}

impl BackgroundExecutor {
    pub fn new(scheduler: Arc<dyn Scheduler>) -> Self {
        Self { scheduler }
    }

    // Core spawning methods via generic helpers
    pub fn spawn<R>(&self, future: impl Future<Output = R> + Send + 'static) -> Task<R>
    where R: Send + 'static {
        // Generic spawn helper implemented on dyn Scheduler - full Future support
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

    // Timer functionality via generic helper using schedule_after
    pub fn timer(&self, duration: Duration) -> Task<()> {
        self.scheduler.timer(duration)
    }

    // Test-specific methods via downcast to TestScheduler
    pub fn deprioritize(&self, label: TaskLabel) {
        if let Some(test_scheduler) = self.scheduler.downcast_ref::<TestScheduler>() {
            test_scheduler.deprioritize(label);
        } else {
            // Production: ignore silently
        }
    }

    pub fn tick(&self) -> Option<bool> {
        self.scheduler.downcast_ref::<TestScheduler>()
            .map(|ts| ts.tick())
    }
}
```

### ForegroundExecutor Integration

```rust
// crates/gpui/src/executor.rs
pub struct ForegroundExecutor {
    scheduler: Rc<dyn Scheduler>,  // Rc for single-threaded use
}

impl ForegroundExecutor {
    // Core spawning for main thread (non-Send futures)
    pub fn spawn<R>(&self, future: impl Future<Output = R> + 'static) -> Task<R>
    where R: 'static {
        // Generic spawn_foreground helper implemented on dyn Scheduler
        self.scheduler.spawn_foreground(future)
    }

    // Timer and test methods same as BackgroundExecutor
    pub fn timer(&self, duration: Duration) -> Task<()> {
        self.scheduler.timer(duration)
    }
}
```

## Cloud Integration

### Session Coordination in SimulatedExecutionContext

Cloud's session coordination logic **should be handled directly within SimulatedExecutionContext**, keeping it close to the ExecutionContext trait implementation and avoiding unnecessary abstraction layers:

```rust
// crates/platform_simulator/src/platform.rs
pub struct SimulatedExecutionContext {
    scheduler: Arc<dyn Scheduler>,  // Unified scheduler via composition
    session_counter: AtomicUsize,
    sessions: Mutex<HashMap<SessionId, WorkerSession>>,
    current_session: Mutex<Option<SessionId>>,
}

#[async_trait(?Send)]
impl PlatformRuntime for SimulatedExecutionContext {
    async fn delay(&self, duration: Duration) {
        // Use unified scheduler's delay mechanism through timer
        self.scheduler.timer(duration).await;
    }
}
```

### Wait Until Implementation

Session coordination integrated directly with unified task scheduling:

```rust
#[async_trait(?Send)]
impl ExecutionContext for SimulatedExecutionContext {
    fn wait_until(&self, future: LocalBoxFuture<'static, Result<()>>) -> Result<()> {
        // 1. Spawn using unified scheduler
        let task_id = self.scheduler.spawn(async move {
            // Add delay via scheduler timer for deterministic simulation
            self.scheduler.timer(Duration::from_millis(10)).await;
            let _ = future.await;
        })?;

        // 2. Register with session coordination (direct access)
        if let Some(session_id) = *self.current_session.lock() {
            if let Some(session) = self.sessions.lock().get_mut(&session_id) {
                session.wait_until_task_ids.insert(task_id);
                self.link_task_to_session(task_id, session_id);
            }
        }

        Ok(())
    }

    async fn pass_through(&self) -> Result<()> {
        // Use unified scheduler's timer for delay
        self.scheduler.timer(Duration::from_millis(10)).await;
        Ok(())
    }
}
```

### Session Coordination Methods

Core session operations handled within SimulatedExecutionContext:

```rust
impl SimulatedExecutionContext {
    pub fn new(scheduler: Arc<dyn Scheduler>) -> Self {
        Self {
            scheduler,
            session_counter: AtomicUsize::new(0),
            sessions: Mutex::new(HashMap::new()),
            current_session: Mutex::new(None),
        }
    }

    pub fn create_session(&self) -> SessionId {
        let session_counter = self.session_counter.fetch_add(1, Ordering::SeqCst);
        let session_id = SessionId(session_counter);

        self.sessions.lock().insert(session_id, WorkerSession {
            spawned_tasks: HashSet::new(),
            wait_until_task_ids: HashSet::new(),
        });

        session_id
    }

    pub fn with_session<F, R>(&self, session_id: SessionId, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        {
            let mut current = self.current_session.lock();
            *current = Some(session_id);
        }

        let result = f();

        {
            let mut current = self.current_session.lock();
            *current = None;
        }

        result
    }

    pub fn validate_session_cleanup(&self, session_id: SessionId) -> platform_api::Result<()> {
        let sessions = self.sessions.lock();
        if let Some(session) = sessions.get(&session_id) {
            // Check running tasks using unified scheduler's task tracking
            let dangling_tasks: Vec<TaskId> = session
                .spawned_tasks
                .iter()
                .filter(|&&task_id| self.scheduler.is_task_running(task_id))
                .copied()
                .collect();

            // Cloud-specific permission check
            let unauthorized: Vec<_> = dangling_tasks
                .into_iter()
                .filter(|task_id| !session.wait_until_task_ids.contains(task_id))
                .collect();

            if !unauthorized.is_empty() {
                return Err(platform_api::WorkerError::Other(anyhow!(
                    "Session cleanup failed: {} unauthorized tasks still running",
                    unauthorized.len()
                )));
            }
        }
        Ok(())
    }

    // Link tasks to sessions during spawning
    fn link_task_to_session(&self, task_id: TaskId, session_id: SessionId) {
        if let Some(session) = self.sessions.lock().get_mut(&session_id) {
            session.spawned_tasks.insert(task_id);
        }
    }

    fn spawn_with_session(&self, future: Pin<Box<dyn Future<Output = ()>>>) -> TaskId {
        let task_id = self.scheduler.spawn(future)?;

        // Auto-associate with current session
        if let Some(session_id) = *self.current_session.lock() {
            self.link_task_to_session(task_id, session_id);
        }

        Ok(task_id)
    }
}
```

### Cloud-Specific Data Structures

```rust
// Session coordination is Cloud-specific but built on unified scheduler
pub struct WorkerSession {
    spawned_tasks: HashSet<TaskId>,        // Tracks tasks in session
    wait_until_task_ids: HashSet<TaskId>,  // Explicitly allowed background tasks
}

impl SimulatedExecutionContext {
    pub fn set_current_session(&self, session_id: SessionId) {
        *self.current_session.lock() = Some(session_id);
    }

    pub fn get_current_session(&self) -> Option<SessionId> {
        *self.current_session.lock()
    }
}
```

### Architecture Benefits

✅ **Clean Composition**: Unified scheduling primitives + Cloud-specific session coordination
✅ **Unified Task Tracking**: Uses TestScheduler's `is_task_running()` for session validation
✅ **Natural Coupling**: Session coordination lives where ExecutionContext operates
✅ **Minimal Abstraction**: No additional coordinator layer needed
✅ **Cloud-Specific Concerns**: Session logic remains in Cloud repo
✅ **Test Integration**: Full TestScheduler features available for Cloud testing
✅ **Deterministic Simulation**: Session-aware timing and task ordering

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

    /// Schedule a runnable after a delay (object-safe for timers)
    fn schedule_after(&self, duration: Duration, runnable: Runnable);

    /// Platform integration methods
    fn park(&self, timeout: Option<Duration>) -> bool;
    fn unparker(&self) -> Unparker;
    fn is_main_thread(&self) -> bool;
    fn now(&self) -> Instant;
}
```

## TestScheduler (Concrete Implementation)

```rust
pub struct TestScheduler {
    inner: RefCell<TestSchedulerInner>,
}

struct TestSchedulerInner {
    tasks: HashMap<TaskId, TaskState>,
    task_labels: HashMap<TaskId, TaskLabel>,
    deprioritized_labels: HashSet<TaskLabel>,
    deprioritized_queue: VecDeque<(Runnable, TaskId)>,
    main_thread_queue: VecDeque<Runnable>,
    delayed: Vec<(Instant, Runnable)>,
    parker: Parker,
    is_main_thread: bool,
    now: Instant,
    next_task_id: AtomicUsize,
    rng: StdRng,
    waiting_tasks: HashSet<TaskId>,
    parking_allowed: bool,
    waiting_hint: Option<String>,
    block_tick_range: std::ops::RangeInclusive<usize>,
}

impl Scheduler for TestScheduler {
    fn schedule(&self, runnable: Runnable) {
        let task_id = TaskId(self.next_task_id.fetch_add(1, Ordering::SeqCst));
        self.inner.borrow_mut().tasks.insert(task_id, TaskState::Running);

        // Schedule completion callback
        let scheduler = self.clone();
        let completion_runnable = async_task::spawn(async move {
            runnable.run();
            scheduler.mark_task_completed(task_id);
        }, |_| {}).0;

        completion_runnable.schedule();
    }

    fn schedule_labeled(&self, runnable: Runnable, label: TaskLabel) {
        let task_id = TaskId(self.next_task_id.fetch_add(1, Ordering::SeqCst));

        if self.inner.borrow().deprioritized_labels.contains(&label) {
            self.inner.borrow_mut().deprioritized_queue.push((runnable, task_id));
            self.inner.borrow_mut().task_labels.insert(task_id, label);
        } else {
            self.inner.borrow_mut().tasks.insert(task_id, TaskState::Running);
            let completion_runnable = async_task::spawn(async move {
                runnable.run();
                // Mark as completed when done
            }, |_| {}).0;
            completion_runnable.schedule();
        }
    }

    fn schedule_foreground(&self, runnable: Runnable) {
        assert!(self.is_main_thread(), "schedule_foreground called off main thread");
        let task_id = TaskId(self.next_task_id.fetch_add(1, Ordering::SeqCst));
        self.inner.borrow_mut().tasks.insert(task_id, TaskState::Running);

        let completion_runnable = async_task::spawn(async move {
            runnable.run();
            // Mark as completed
        }, |_| {}).0;

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
    // Test-specific methods (NOT on main trait)
    pub fn deprioritize(&self, label: TaskLabel) {
        self.inner.borrow_mut().deprioritized_labels.insert(label);
    }

    pub fn is_task_running(&self, task_id: TaskId) -> bool {
        self.inner.borrow().tasks.contains_key(&task_id)
    }

    pub fn tick(&self) -> bool { /* implementation */ }
    pub fn run_until_parked(&self) { /* implementation */ }
    pub fn advance_clock(&self, duration: Duration) { /* implementation */ }
}
```

## Generic Spawn Helpers

Generic spawn methods implemented for `dyn Scheduler`:

```rust
impl dyn Scheduler {
    pub fn spawn<R>(&self, future: impl Future<Output = R> + Send + 'static) -> Task<R>
    where R: Send + 'static {
        let task_id = self.assign_task_id();
        let (runnable, inner_task) = async_task::spawn(future, move |runnable| {
            self.mark_task_started(task_id);
            self.schedule_completion_callback(runnable, task_id);
        });

        self.schedule(runnable);
        Task { /* ... */ }
    }

    pub fn spawn_foreground<R>(&self, future: impl Future<Output = R> + 'static) -> Task<R>
    where R: 'static {
        let task_id = self.assign_task_id();
        let (runnable, inner_task) = async_task::spawn_local(future, move |runnable| {
            self.mark_task_started(task_id);
            self.schedule_completion_callback(runnable, task_id);
        });

        self.schedule_foreground(runnable);
        Task { /* ... */ }
    }

    pub fn timer(&self, duration: Duration) -> Task<()> {
        if duration.is_zero() {
            return Task::ready(());
        }

        let (runnable, inner_task) = async_task::spawn(async move {}, {
            let scheduler = &*self;
            move |runnable| {
                scheduler.schedule_after(duration, runnable);
            }
        });

        runnable.schedule();
        Task { /* ... */ }
    }

    pub fn is_task_running(&self, task_id: TaskId) -> bool {
        // Requires downcast to TestScheduler
        None // Default implementation
    }
}
```

## Migration Strategy

### Phase 1: Core Infrastructure
1. Define `Scheduler` trait (core methods only)
2. Implement `TestScheduler` (with test features like `deprioritize()`)
3. Make existing GPUI platform dispatchers implement `Scheduler` trait
   - MacDispatcher implements `Scheduler` for GCD integration
   - LinuxDispatcher implements `Scheduler` for thread pools
   - WindowsDispatcher implements `Scheduler` for Windows ThreadPool
4. Define `Task<T>` with mandatory TaskId

### Phase 2: GPUI Migration
1. Update GPUI executors to use trait objects
2. Add downcasting for test features
3. Preserve all existing GPUI functionality
4. Test deployments use TestScheduler, production uses minimal schedulers

### Phase 3: Cloud Integration
1. Update `SimulatedExecutionContext` to use `Arc<dyn Scheduler>`
2. Move session coordination logic into `SimulatedExecutionContext`
3. Integrate `wait_until()` with unified task scheduling
4. Use TestScheduler features for session validation
5. Preserve all existing Cloud platform APIs

### Phase 4: Testing & Validation
1. GPUI tests work with new architecture
2. Cloud session behavior preserved
3. Production efficiency maintained
4. Both domains benefit from unified test infrastructure

## Platform Backend Files

### GPUI Backends
- `crates/gpui/src/platform/mac/dispatcher.rs` - `MacDispatcher` should implement `Scheduler` trait
- `crates/gpui/src/platform/linux/dispatcher.rs` - `LinuxDispatcher` should implement `Scheduler` trait
- `crates/gpui/src/platform/windows/dispatcher.rs` - `WindowsDispatcher` should implement `Scheduler` trait
- `crates/gpui/src/platform/test/dispatcher.rs` - `TestDispatcher` → `TestScheduler` (moved to shared crate)

### Cloud Backends
- `crates/platform_simulator/src/platform.rs` - `SimulatedExecutionContext` should contain `Scheduler` + session coordination
- `crates/cloudflare_platform/src/execution_context.rs` - Cloudflare-specific ExecutionContext using Scheduler

## Compatibility Checklist

## Complete GPUI + Cloud Feature Coverage ✅

### GPUI Compatibility
- ✅ `spawn()` → `dyn Scheduler::spawn()` (generic helper on trait object)
- ✅ `spawn_labeled(label)` → `dyn Scheduler::spawn_labeled()` (generic helper on trait object)
- ✅ `timer(duration)` → `dyn Scheduler::timer()` (generic helper using schedule_after)
- ✅ `block(future)` → `dyn Scheduler::block()` (generic helper with parking)
- ✅ `block_with_timeout(future, timeout)` → `dyn Scheduler::block_with_timeout()` (generic helper)
- ✅ `now()` → `scheduler.now()` (direct trait object method)
- ✅ `is_main_thread()` → `scheduler.is_main_thread()` (direct trait object method)
- ✅ `num_cpus()` → `dyn Scheduler::num_cpus()` (generic helper)
- ✅ `deprioritize(label)` → Downcast to TestScheduler, then TestScheduler::deprioritize()
- ✅ `tick()` → Downcast to TestScheduler, then TestScheduler::tick()
- ✅ `run_until_parked()` → Downcast to TestScheduler, then TestScheduler::run_until_parked()
- ✅ `advance_clock(duration)` → Downcast to TestScheduler, then TestScheduler::advance_clock()
- ✅ `simulate_random_delay()` → Downcast to TestScheduler, then TestScheduler::simulate_random_delay()
- ✅ `BackgroundExecutor` → Trait object wrapper using `dyn Scheduler`

### Cloud Compatibility
- ✅ **Session Coordination**: `ExecutionContext.wait_until()` with direct session integration
- ✅ **Task Lifecycle**: Uses unified scheduler's `is_task_running()` for validation
- ✅ **Worker Management**: Session context and cleanup validation
- ✅ **Background Tasks**: Explicit permission system for long-running work
- ✅ **Deterministic Testing**: Full TestScheduler integration with session tracking
- ✅ **Platform Runtime**: `PlatformRuntime.delay()` via unified scheduler timer
- ✅ **Session Validation**: Dangling task detection with proper error reporting
- ✅ **Auto-Association**: Tasks automatically linked to sessions during spawn

### Unified Benefits
- ✅ **Clean Separation**: GPUI gets deprioritization, Cloud gets session coordination
- ✅ **Unified Task Tracking**: Both domains use `TestScheduler.is_task_running()` for validation
- ✅ **Composability**: Session coordination built on unified scheduling primitives
- ✅ **Domain-Specific**: Each domain handles its coordination concerns appropriately
- ✅ **Test Infrastructure**: Shared deterministic testing capabilities
- ✅ **Production Ready**: Both domains can use minimal platform schedulers
- ✅ **Extensible**: New coordination patterns can be added without shared crate changes

## Implementation Notes

### Key Design Decisions

1. **GPUI**: Uses task labels for deterministic UI testing
2. **Cloud**: Uses session coordination for worker lifecycle management
3. **Shared**: Core scheduling primitives + TestScheduler for task tracking
4. **Integration**: Both domains use composition with unified scheduler

### Migration Considerations

- **Zero Breaking Changes**: Existing APIs preserved via generic helpers
- **Gradual Migration**: Can migrate GPUI and Cloud independently
- **Test Preservation**: All existing test functionality maintained
- **Performance**: Minimal overhead from trait objects in production

This architecture provides clean separation between GPUI's UI determinism needs and Cloud's session coordination requirements, while sharing the core task scheduling infrastructure.
