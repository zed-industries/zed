┌─────────────────────────────────────────┐
│            Shared Crate                 │
├─────────────────────────────────────────┤
│ Scheduler trait:                        │
│ - Core object-safe interface             │
│                                         │
│ TestScheduler:                          │
│ - Should implement Scheduler + test features  │
│ - deprioritize() - test-only method     │
│ - spawn_labeled() - labels for testing  │
│ - Task lifecycle tracking               │
│ - creation_thread_id for Foreground checks│
│                                         │
│ Executor wrappers:                      │
│ - Executor: Wraps Arc<dyn Scheduler>, Send futures│
│ - ForegroundExecutor: Wraps Arc<dyn Scheduler>, !Send, thread checks│
└─────────────────────────────────────────┘
                    ▲
          ┌─────────┼─────────┐
          │         │         │
┌─────────┴────┐  ┌─┴─────────┴────┐
│   GPUI       │  │     Cloud       │
│ Uses Executor│  │ ForegroundExec │
│ + Foreground │  │ for single-thrd│
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

The shared crate provides only the trait definition and generic helpers, while platform-specific dispatchers implement the `Scheduler` trait directly in GPUI. **Wrappers handle delegation and thread safety.**

### BackgroundExecutor Integration

GPUI's executors now use wrappers:

```rust
// crates/gpui/src/executor.rs
pub struct BackgroundExecutor {
    executor: Executor,  // Generic wrapper for background tasks (Send futures)
}

impl BackgroundExecutor {
    pub fn new(scheduler: Arc<dyn Scheduler>) -> Self {
        Self { executor: Executor::new(scheduler) }
    }

    // Core spawning methods via wrapper delegation
    pub fn spawn<R>(&self, future: impl Future<Output = R> + Send + 'static) -> Task<R>
    where R: Send + 'static {
        self.executor.spawn(future)
    }

    pub fn spawn_labeled<R>(
        &self,
        label: TaskLabel,
        future: impl Future<Output = R> + Send + 'static
    ) -> Task<R>
    where R: Send + 'static {
        self.executor.spawn_labeled(label, future)
    }

    // Timer functionality via wrapper
    pub fn timer(&self, duration: Duration) -> Task<()> {
        self.executor.timer(duration)
    }

    // Test-specific methods via downcast in wrapper
    pub fn deprioritize(&self, label: TaskLabel) {
        self.executor.deprioritize(label);
    }

    pub fn tick(&self) -> Option<bool> {
        self.executor.tick()
    }
}
```

### ForegroundExecutor Integration

GPUI's foreground executor enforces main-thread usage:

```rust
// crates/gpui/src/executor.rs
pub struct ForegroundExecutor {
    executor: Executor,  // Underlying executor for delegation
    _phantom: PhantomData<Rc<()>>,  // Enforces !Send
    creation_thread_id: ThreadId,  // Stored for checks
}

impl !Send for ForegroundExecutor {}  // Explicitly !Send

impl ForegroundExecutor {
    pub fn new(scheduler: Arc<dyn Scheduler>) -> Result<Self> {
        let creation_thread_id = thread::current().id();
        // Delegate creation to underlying scheduler
        let _ = Executor::new(scheduler.clone());
        Ok(Self { executor: Executor::new(scheduler), _phantom: PhantomData, creation_thread_id })
    }

    // Core spawning for main thread (non-Send futures)
    pub fn spawn<R>(&self, future: impl Future<Output = R> + 'static) -> Task<R>
    where R: 'static {
        if thread::current().id() != self.creation_thread_id {
            panic!("ForegroundExecutor called off main thread");
        }
        // Delegate to scheduler.spawn_foreground via wrapper
        self.executor.scheduler.spawn_foreground(future)
    }

    // Timer and test methods same as BackgroundExecutor but with thread checks
    pub fn timer(&self, duration: Duration) -> Task<()> {
        if thread::current().id() != self.creation_thread_id {
            panic!("ForegroundExecutor called off main thread");
        }
        self.executor.timer(duration)
    }
}
```

## Cloud Integration

### Session Coordination in SimulatedExecutionContext

Cloud's session coordination logic **should be handled directly within SimulatedExecutionContext**, keeping it close to the ExecutionContext trait implementation and avoiding unnecessary abstraction layers. **Uses ForegroundExecutor for single-threaded consistency and to avoid Send requirements on futures.**

```rust
// crates/platform_simulator/src/platform.rs
pub struct SimulatedExecutionContext {
    fg_executor: ForegroundExecutor,  // Single-threaded wrapper for simplicity
    session_counter: AtomicUsize,
    sessions: Mutex<HashMap<SessionId, WorkerSession>>,
    current_session: Mutex<Option<SessionId>>,
}

#[async_trait(?Send)]
impl PlatformRuntime for SimulatedExecutionContext {
    async fn delay(&self, duration: Duration) {
        // Use wrapper's timer for delay
        self.fg_executor.timer(duration).await;
    }
}
```

### Wait Until Implementation

Session coordination integrated directly with wrapper's task scheduling:

```rust
#[async_trait(?Send)]
impl ExecutionContext for SimulatedExecutionContext {
    fn wait_until(&self, future: LocalBoxFuture<'static, Result<()>>) -> Result<()> {
        // 1. Spawn using wrapper (no Send required)
        let task = self.fg_executor.spawn(async move { future.await })?;
        
        // 2. Register with session coordination via downcast
        if let Some(test_sched) = self.fg_executor.executor.scheduler.as_any().downcast_ref::<TestScheduler>() {
            if let Some(session_id) = test_sched.get_current_session() {
                test_sched.track_task_for_session(task.id(), session_id);
                test_sched.add_wait_until_task(session_id, task.id());
            }
        }

        Ok(())
    }

    async fn pass_through(&self) -> Result<()> {
        // Use wrapper's timer for delay
        self.fg_executor.timer(Duration::from_millis(10)).await;
        Ok(())
    }
}
```

### Session Coordination Methods

Core session operations handled within SimulatedExecutionContext:

```rust
impl SimulatedExecutionContext {
    pub fn new(scheduler: Arc<dyn Scheduler>) -> Result<Self> {
        let fg_executor = ForegroundExecutor::new(scheduler)?;
        Ok(Self {
            fg_executor,
            session_counter: AtomicUsize::new(0),
            sessions: Mutex::new(HashMap::new()),
            current_session: Mutex::new(None),
        })
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
            // Check running tasks using wrapper's TestScheduler access
            if let Some(test_sched) = self.fg_executor.executor.scheduler.as_any().downcast_ref::<TestScheduler>() {
                let dangling_tasks: Vec<TaskId> = session
                    .spawned_tasks
                    .iter()
                    .filter(|&&task_id| test_sched.is_task_running(task_id))
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
        }
        Ok(())
    }

    pub fn set_current_session(&self, session_id: SessionId) {
        *self.current_session.lock() = Some(session_id);
    }

    pub fn get_current_session(&self) -> Option<SessionId> {
        *self.current_session.lock()
    }
}
```

### Cloud-Specific Data Structures

Session coordination is Cloud-specific but built on unified scheduler primitives via wrappers.

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
    creation_thread_id: ThreadId,  // Added for wrapper checks
}

impl Scheduler for TestScheduler {
    fn schedule(&self, runnable: Runnable) {
        // Implementation as before
    }

    fn schedule_labeled(&self, runnable: Runnable, label: TaskLabel) {
        // Implementation as before, with deprioritization
    }

    fn schedule_foreground(&self, runnable: Runnable) {
        assert!(thread::current().id() == self.inner.borrow().creation_thread_id, "schedule_foreground called off main thread");
        // Implementation as before
    }

    // Other trait methods unchanged
}

impl TestScheduler {
    // Test-specific methods (NOT on main trait)
    pub fn deprioritize(&self, label: TaskLabel) { /* implementation */ }
    pub fn is_task_running(&self, task_id: TaskId) -> bool { /* implementation */ }
    pub fn tick(&self) -> bool { /* implementation */ }
    pub fn run_until_parked(&self) { /* implementation */ }
    pub fn advance_clock(&self, duration: Duration) { /* implementation */ }
    pub fn assert_main_thread(&self) { /* implementation */ }
}
```

## Generic Spawn Helpers

Generic spawn methods implemented for `dyn Scheduler`, now called by wrappers.

## Migration Strategy

### Phase 1: Core Infrastructure
1. Define `Scheduler` trait (core methods only)
2. Implement `TestScheduler` with thread ID tracking
3. Add wrapper structs `Executor` and `ForegroundExecutor`
4. Make existing GPUI platform dispatchers implement `Scheduler` trait
5. Add `as_any()` to `Scheduler` for downcasting

### Phase 2: GPUI Migration
1. Update GPUI executors to use `Executor` and `ForegroundExecutor` wrappers
2. Handle downcasting in wrappers for test features
3. Preserve all existing GPUI functionality
4. Test deployments use TestScheduler, production uses minimal schedulers

### Phase 3: Cloud Integration
1. Update `SimulatedExecutionContext` to use `ForegroundExecutor`
2. Move session coordination logic into `SimulatedExecutionContext`
3. Integrate `wait_until()` with wrapper scheduling
4. Use TestScheduler features for session validation via downcast
5. Preserve all existing Cloud platform APIs

### Phase 4: Testing & Validation
1. GPUI tests work with new architecture
2. Cloud session behavior preserved (single-threaded)
3. Production efficiency maintained
4. Both domains benefit from unified test infrastructure

## Platform Backend Files

### GPUI Backends
- `crates/gpui/src/platform/mac/dispatcher.rs` - `MacDispatcher` implements `Scheduler`
- `crates/gpui/src/platform/linux/dispatcher.rs` - `LinuxDispatcher` implements `Scheduler`
- `crates/gpui/src/platform/windows/dispatcher.rs` - `WindowsDispatcher` implements `Scheduler`
- `crates/gpui/src/platform/test/dispatcher.rs` - `TestDispatcher` → `TestScheduler` (moved to shared crate)

### Cloud Backends
- `crates/platform_simulator/src/platform.rs` - `SimulatedExecutionContext` uses `ForegroundExecutor`
- `crates/cloudflare_platform/src/execution_context.rs` - Cloudflare-specific ExecutionContext using `ForegroundExecutor`

## Compatibility Checklist

## Complete GPUI + Cloud Feature Coverage ✅

### GPUI Compatibility
- ✅ `spawn()` → `Executor::spawn()` or `ForegroundExecutor::spawn()`
- ✅ `spawn_labeled(label)` → Wrappers delegate to `dyn Scheduler::spawn_labeled()`
- ✅ `timer(duration)` → Wrappers delegate to `dyn Scheduler::timer()`
- ✅ `block(future)` → Wrappers handle with parking
- ✅ `block_with_timeout(future, timeout)` → Wrappers handle
- ✅ `now()` → `scheduler.now()` (direct trait method)
- ✅ `is_main_thread()` → `scheduler.is_main_thread()` (direct trait method)
- ✅ `num_cpus()` → Generic helper on wrappers
- ✅ `deprioritize(label)` → Downcast in wrappers, then TestScheduler::deprioritize()
- ✅ `tick()` → Downcast in wrappers, then TestScheduler::tick()
- ✅ `run_until_parked()` → Downcast in wrappers, then TestScheduler::run_until_parked()
- ✅ `advance_clock(duration)` → Downcast in wrappers, then TestScheduler::advance_clock()
- ✅ `simulate_random_delay()` → Downcast in wrappers, then TestScheduler::simulate_random_delay()
- ✅ `BackgroundExecutor` → Uses `Executor` wrapper

### Cloud Compatibility
- ✅ **Session Coordination**: `ExecutionContext.wait_until()` via `ForegroundExecutor`
- ✅ **Task Lifecycle**: Uses wrapper's TestScheduler access for validation
- ✅ **Worker Management**: Session context and cleanup validation
- ✅ **Background Tasks**: Explicit permission system for long-running work
- ✅ **Deterministic Testing**: Full TestScheduler integration with session tracking
- ✅ **Platform Runtime**: `PlatformRuntime.delay()` via wrapper timer
- ✅ **Session Validation**: Dangling task detection with proper error reporting
- ✅ **Auto-Association**: Tasks automatically linked to sessions during spawn

### Unified Benefits
- ✅ **Clean Separation**: GPUI gets deprioritization, Cloud gets session coordination
- ✅ **Unified Task Tracking**: Both domains use TestScheduler via wrappers for validation
- ✅ **Composability**: Session coordination built on unified scheduling primitives
- ✅ **Domain-Specific**: Each domain handles its coordination concerns appropriately
- ✅ **Test Infrastructure**: Shared deterministic testing capabilities
- ✅ **Production Ready**: Both domains can use minimal platform schedulers
- ✅ **Extensible**: New coordination patterns can be added without shared crate changes
- ✅ **Thread Safety**: ForegroundExecutor enforces main-thread use across domains

## Implementation Notes

### Key Design Decisions

1. **GPUI**: Uses `Executor` for background (Send), `ForegroundExecutor` for main-thread (!Send)
2. **Cloud**: Uses `ForegroundExecutor` for single-threaded simplicity (no Send required on futures)
3. **Shared**: Core scheduling primitives + wrappers for delegation and safety
4. **Integration**: Both domains use wrappers with consistent API

### Migration Considerations

- **Zero Breaking Changes**: Existing APIs preserved via wrappers
- **Gradual Migration**: Can migrate GPUI and Cloud independently
- **Test Preservation**: All existing test functionality maintained
- **Performance**: Minimal overhead from trait objects in production
- **Cloud Simplification**: ForegroundExecutor allows non-Send futures in single-threaded context

This architecture provides clean separation between GPUI's UI determinism needs and Cloud's session coordination requirements, while sharing the core task scheduling infrastructure and enforcing thread safety through wrappers.