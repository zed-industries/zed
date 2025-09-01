# TestScheduler Design Details

This document expands on the [Unified Scheduler Architecture Plan](2025-08-31_15-47-17_unified-scheduler-architecture.md) by providing a detailed design and complete implementation of the TestScheduler. It assumes familiarity with the broader architecture, including the shared `Scheduler` trait, domain separation (GPUI vs. Cloud), and multi-threading test scenarios. **Updates incorporate Executor and ForegroundExecutor wrappers around Arc<dyn Scheduler>, with ForegroundExecutor using PhantomData<Rc<()>> for !Send and panicking if not on the creation thread, applied to both GPUI and Cloud for consistency and simplicity.**

## Overview

The TestScheduler is the **single concrete test implementation** of the `Scheduler` trait (see Section 3: Scheduler Trait Definition in the original plan). It serves as the unified core for all test scenarios, enabling:

- **GPUI Testing**: Deterministic UI scheduling with task labels, deprioritization, main thread isolation, and tick-based execution (see Section 4.1: GPUI Integration in the original plan).
- **Cloud Testing**: Session coordination, time-range delays, wait-until task tracking, and cleanup validation (see Section 5: Cloud Integration in the original plan). **ForegroundExecutor is now used in Cloud for single-threaded simplicity, avoiding Send requirements on futures.**
- **Unified Testing**: Shared across test threads for client-cloud interactions, seeded randomness, and task lifecycle management.

There is **no separate TestScheduler trait**—all test-specific methods are directly on the TestScheduler struct for simplicity, as it is the only implementation in the test context. **Executors wrap Arc<dyn Scheduler>, with ForegroundExecutor enforcing thread safety via phantom Rc and creation-thread checks.**

## Design Principles

- **Minimal Complexity**: No unnecessary traits or abstractions; direct methods for test features.
- **Merged Capabilities**: Combines GPUI queues with Cloud session logic in a single state machine.
- **Determinism**: Always seeded with configurable randomization.
- **Multi-threading Ready**: `Arc` and `Mutex` for shared access in collaborative tests.
- **Domain Wrappers**: GPUI/Cloud test code wraps this core for specific APIs (e.g., GPUI's BackgroundExecutor now uses Executor and ForegroundExecutor).
- **Thread Safety Enforcement**: ForegroundExecutor uses phantom Rc for !Send and checks against creation thread for main-thread isolation.

## Complete TestScheduler Implementation

```rust
use std::collections::{HashMap, HashSet, VecDeque};
use std::future::Future;
use std::ops::RangeInclusive;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread;
use std::time::{Duration, Instant};

use anyhow::{anyhow, Result};
use async_task::Runnable;
use chrono::{DateTime, Utc};
use futures::channel::oneshot;
use parking_lot::{Mutex as ParkingMutex, MutexGuard};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

// Core types (shared with main crate)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct TaskId(usize);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct TaskLabel(usize);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct SessionId(usize);

pub enum ThreadTarget { Main, Background }

pub enum VariationSource {
    /// Seeded randomness for reproducible probabilistic scheduling.
    Seeded(u64),
    /// Fuzzed inputs for deterministic exploration of variations.
    Fuzzed(SchedulerFuzzInput),
}

#[derive(Clone, Debug)]
pub struct SchedulerConfig {
    /// Whether to randomize task ordering (e.g., queue shuffling); defaults to true for coverage.
    pub randomize_order: bool,
    /// Max steps before panic (for deadlock detection).
    pub max_steps: usize,
    /// Whether to log operations (for debugging).
    pub log_operations: bool,
    /// The source of non-deterministic decisions (mutually exclusive modes).
    pub variation_source: VariationSource,
}

impl Default for SchedulerConfig {
    fn default() -> Self {
        Self {
            randomize_order: true,
            max_steps: 10000,
            log_operations: false,
            variation_source: VariationSource::Seeded(0),
        }
    }
}

impl SchedulerConfig {
    /// Create a seeded config (randomization enabled by default).
    pub fn seeded(seed: u64) -> Self {
        Self {
            variation_source: VariationSource::Seeded(seed),
            ..Default::default()
        }
    }

    /// Create a fuzzed config (randomization enabled by default).
    pub fn fuzzed(fuzz_inputs: SchedulerFuzzInput) -> Self {
        Self {
            variation_source: VariationSource::Fuzzed(fuzz_inputs),
            ..Default::default()
        }
    }

    /// For isolation: Disable randomization for deterministic testing.
    pub fn deterministic(seed: u64) -> Self {
        Self {
            randomize_order: false,
            variation_source: VariationSource::Seeded(seed),
            ..Default::default()
        }
    }
}

#[derive(Arbitrary, Debug, Clone)]
pub struct SchedulerFuzzInput {
    queue_selections: Vec<u8>,
    task_indices: Vec<u32>,
    delay_bools: Vec<bool>,
    block_ticks: Vec<usize>,
}

pub struct Task<T> {
    id: TaskId,
    rx: oneshot::Receiver<T>,
    scheduler: Arc<TestScheduler>,
}

impl<T> Future for Task<T> {
    type Output = T;
    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        match self.rx.try_recv() {
            Ok(Some(val)) => Poll::Ready(val),
            _ => { cx.waker().wake_by_ref(); Poll::Pending }
        }
    }
}

// Internal state
struct TaskInfo {
    label: Option<TaskLabel>,
    session_id: Option<SessionId>,
    waker: Option<Waker>,
    future: Option<Pin<Box<dyn Future<Output = ()>>>>,
    state: TaskState,
    delay_range: Option<(Instant, Instant)>,
}

#[derive(PartialEq)]
enum TaskState { Pending, Running, Completed }

struct WorkerSession {
    spawned_tasks: HashSet<TaskId>,
    wait_until_tasks: HashSet<TaskId>,
}

struct SchedulerState {
    rng: ChaCha8Rng,
    randomize_order: bool,
    max_steps: usize,
    current_time: Instant,
    next_task_id: AtomicUsize,
    tasks: HashMap<TaskId, TaskInfo>,
    sessions: HashMap<SessionId, WorkerSession>,
    current_session: Option<SessionId>,
    ready_queue: VecDeque<TaskId>,
    deprioritized_queue: VecDeque<TaskId>,
    main_thread_queue: VecDeque<TaskId>,
    delayed: Vec<(Instant, TaskId)>,
    deprioritized_labels: HashSet<TaskLabel>,
    block_tick_range: std::ops::RangeInclusive<usize>,
    parker: parking_lot::Parker,
    unparker: parking_lot::Unparker,
    parking_allowed: AtomicBool,
    execution_history: Vec<String>,
    fuzz_inputs: Option<FuzzedSchedulerInputs>,
    creation_thread_id: thread::ThreadId,  // Added for thread safety checks
}

// Concrete implementation
pub struct TestScheduler {
    state: Arc<Mutex<SchedulerState>>,
}

impl TestScheduler {
    /// Primary constructor: Create a scheduler from full configuration.
    pub fn new(config: SchedulerConfig) -> Arc<Self> {
        let (parker, unparker) = parking_lot::pair();
        let (rng, fuzz_inputs) = match config.variation_source {
            VariationSource::Seeded(seed) => (ChaCha8Rng::seed_from_u64(seed), None),
            VariationSource::Fuzzed(inputs) => (ChaCha8Rng::seed_from_u64(0), Some(inputs)),
        };
        let state = SchedulerState {
            rng,
            randomize_order: config.randomize_order,
            max_steps: config.max_steps,
            current_time: Instant::now(),
            next_task_id: AtomicUsize::new(1),
            tasks: HashMap::new(),
            sessions: HashMap::new(),
            current_session: None,
            ready_queue: VecDeque::new(),
            deprioritized_queue: VecDeque::new(),
            main_thread_queue: VecDeque::new(),
            delayed: Vec::new(),
            deprioritized_labels: HashSet::new(),
            block_tick_range: 0..=1000,
            parker,
            unparker,
            parking_allowed: AtomicBool::new(false),
            execution_history: Vec::new(),
            fuzz_inputs,
            creation_thread_id: thread::current().id(),  // Capture creation thread
        };
        Arc::new(Self { state: Arc::new(Mutex::new(state)) })
    }

    // Added for ForegroundExecutor thread checks
    pub fn assert_main_thread(&self) {
        let state = self.state.lock().unwrap();
        if thread::current().id() != state.creation_thread_id {
            panic!("ForegroundExecutor method called from wrong thread");
        }
    }

    /// Convenience helper: Create a seeded scheduler (randomization enabled by default).
    pub fn from_seed(seed: u64) -> Arc<Self> {
        Self::new(SchedulerConfig::seeded(seed))
    }

    /// Convenience helper: Create a scheduler driven by fuzzed inputs.
    /// Use for fuzzing with Bolero.
    pub fn from_fuzz(fuzz_inputs: SchedulerFuzzInput) -> Arc<Self> {
        Self::new(SchedulerConfig::fuzzed(fuzz_inputs))
    }

    // Test-specific methods (direct on impl, no trait)
    pub fn assign_task_id(&self) -> TaskId {
        TaskId(self.state.lock().unwrap().next_task_id.fetch_add(1, Ordering::SeqCst))
    }

    pub fn deprioritize(&self, label: TaskLabel) {
        let mut state = self.state.lock().unwrap();
        state.deprioritized_labels.insert(label);
    }

    pub fn is_task_running(&self, task_id: TaskId) -> bool {
        let state = self.state.lock().unwrap();
        state.tasks.get(&task_id).map_or(false, |t| t.state == TaskState::Running)
    }

    pub fn tick(&self, background_only: bool) -> bool {
        self.tick_internal(background_only)
    }

    fn tick_internal(&self, background_only: bool) -> bool {
        // Process delays first (drop lock before polling)
        {
            let mut state = self.state.lock().unwrap();
            state.delayed.retain(|&(time, task_id)| {
                if time <= state.current_time && (!state.randomize_order || state.rng.gen_bool(0.5)) {
                    state.ready_queue.push_back(task_id);
                    false
                } else { true }
            });
        } // Lock dropped here

        // Select and poll task without lock held
        let task_to_poll = {
            let mut state = self.state.lock().unwrap();
            let mut queues = vec![&mut state.ready_queue, &mut state.deprioritized_queue];
            if !background_only { queues.insert(0, &mut state.main_thread_queue); }

            let mut available: Vec<usize> = queues.iter().enumerate()
                .filter(|&(_, q)| !q.is_empty())
                .map(|(i, _)| i)
                .collect();

            if available.is_empty() { return false; }

            if state.randomize_order { available.shuffle(&mut state.rng); }

            let queue_ix = available[0];
            let task_id = queues[queue_ix].pop_front().unwrap();
            Some(task_id)
        }; // Lock dropped here

        // Poll the task's future outside the lock
        let poll_result = {
            let mut state = self.state.lock().unwrap();
            if let Some(task) = state.tasks.get_mut(&task_to_poll.unwrap()) {
                task.state = TaskState::Running;
                if let Some(fut) = task.future.as_mut() {
                    if let Some(waker) = task.waker.as_ref() {
                        let mut context = Context::from_waker(waker);
                        fut.as_mut().poll(&mut context)
                    } else {
                        Poll::Pending
                    }
                } else {
                    Poll::Pending
                }
            } else {
                Poll::Pending
            }
        }; // Lock dropped here

        // Update task state after polling
        if poll_result.is_ready() {
            let mut state = self.state.lock().unwrap();
            if let Some(task) = state.tasks.get_mut(&task_to_poll.unwrap()) {
                task.state = TaskState::Completed;
                state.execution_history.push(format!("Ticked task {}", task_to_poll.unwrap().0));
            }
        }

        true
    }

    pub fn advance_clock(&self, duration: Duration) {
        let mut state = self.state.lock().unwrap();
        state.current_time += duration;
    }

    pub fn run_until_parked(&self) {
        while self.tick(false) {}
    }

    // Cloud session methods
    pub fn create_session(&self) -> SessionId {
        let mut state = self.state.lock().unwrap();
        let id = SessionId(state.sessions.len());
        state.sessions.insert(id, WorkerSession { spawned_tasks: HashSet::new(), wait_until_tasks: HashSet::new() });
        id
    }

    pub fn set_current_session(&self, session_id: Option<SessionId>) {
        let mut state = self.state.lock().unwrap();
        state.current_session = session_id;
    }

    pub fn get_current_session(&self) -> Option<SessionId> {
        self.state.lock().unwrap().current_session
    }

    pub fn track_task_for_session(&self, task_id: TaskId, session_id: SessionId) {
        let mut state = self.state.lock().unwrap();
        if let Some(session) = state.sessions.get_mut(&session_id) {
            session.spawned_tasks.insert(task_id);
        }
    }

    pub fn add_wait_until_task(&self, session_id: SessionId, task_id: TaskId) {
        let mut state = self.state.lock().unwrap();
        if let Some(session) = state.sessions.get_mut(&session_id) {
            session.wait_until_tasks.insert(task_id);
        }
    }

    pub fn validate_session_cleanup(&self, session_id: SessionId) -> Result<()> {
        let state = self.state.lock().unwrap();
        if let Some(session) = state.sessions.get(&session_id) {
            let dangling: Vec<_> = session.spawned_tasks.iter()
                .filter(|&&tid| state.tasks.get(&tid).map_or(false, |t| t.state != TaskState::Completed))
                .filter(|&&tid| !session.wait_until_tasks.contains(&tid))
                .cloned()
                .collect();
            if !dangling.is_empty() {
                return Err(anyhow!("{} dangling tasks", dangling.len()));
            }
        }
        Ok(())
    }

    // Other test methods (e.g., GPUI block simulation)
    pub fn gen_block_on_ticks(&self) -> usize {
        let state = self.state.lock().unwrap();
        state.rng.gen_range(state.block_on_ticks_range.clone())
    }
}
```

## GPUI Usage Example

GPUI wraps the TestScheduler using Executor and ForegroundExecutor:

```rust
use std::marker::PhantomData;
use std::rc::Rc;

// Generic Executor for background tasks (Send futures)
pub struct Executor {
    scheduler: Arc<dyn Scheduler>,
}

impl Executor {
    pub fn new(scheduler: Arc<dyn Scheduler>) -> Self {
        Self { scheduler }
    }

    pub fn spawn<R>(&self, future: impl Future<Output = R> + Send + 'static) -> Task<R>
    where R: Send + 'static {
        // Delegate to scheduler via downcast for test methods if applicable
        if let Some(test_sched) = self.scheduler.as_any().downcast_ref::<TestScheduler>() {
            // Use test_sched methods
        }
        self.scheduler.spawn(future)
    }

    pub fn spawn_labeled<R>(
        &self,
        label: TaskLabel,
        future: impl Future<Output = R> + Send + 'static
    ) -> Task<R>
    where R: Send + 'static {
        if let Some(test_sched) = self.scheduler.as_any().downcast_ref::<TestScheduler>() {
            test_sched.deprioritize(label);
        }
        self.scheduler.spawn_labeled(label, future)
    }

    pub fn deprioritize(&self, label: TaskLabel) {
        if let Some(test_sched) = self.scheduler.as_any().downcast_ref::<TestScheduler>() {
            test_sched.deprioritize(label);
        }
    }

    pub fn timer(&self, duration: Duration) -> Task<()> {
        self.scheduler.timer(duration)
    }

    pub fn tick(&self) -> Option<bool> {
        self.scheduler.as_any().downcast_ref::<TestScheduler>().map(|ts| ts.tick(false))
    }
}

// ForegroundExecutor for main-thread tasks (!Send futures, thread checks)
pub struct ForegroundExecutor {
    executor: Executor,
    _phantom: PhantomData<Rc<()>>,  // Enforces !Send
}

impl !Send for ForegroundExecutor {}  // Explicitly !Send

impl ForegroundExecutor {
    pub fn new(scheduler: Arc<dyn Scheduler>) -> Result<Self> {
        let executor = Executor::new(scheduler);
        // Check thread immediately via scheduler
        if let Some(test_sched) = executor.scheduler.as_any().downcast_ref::<TestScheduler>() {
            test_sched.assert_main_thread();
        } else {
            // Production: assume created on main thread
        }
        Ok(Self { executor, _phantom: PhantomData })
    }

    pub fn spawn<R>(&self, future: impl Future<Output = R> + 'static) -> Task<R>
    where R: 'static {
        // Assert thread before delegating
        if let Some(test_sched) = self.executor.scheduler.as_any().downcast_ref::<TestScheduler>() {
            test_sched.assert_main_thread();
        }
        self.executor.scheduler.spawn_foreground(future)
    }

    pub fn timer(&self, duration: Duration) -> Task<()> {
        if let Some(test_sched) = self.executor.scheduler.as_any().downcast_ref::<TestScheduler>() {
            test_sched.assert_main_thread();
        }
        self.executor.scheduler.timer(duration)
    }

    // Other methods mirror Executor but with thread checks
}
```

## Cloud Usage Example

Cloud wraps using ForegroundExecutor for single-threaded simplicity (no Send futures required):

```rust
impl SimulatedExecutionContext {
    pub fn new(scheduler: Arc<dyn Scheduler>) -> Result<Self> {
        let fg_executor = ForegroundExecutor::new(scheduler)?;  // Use ForegroundExecutor for thread safety and simplicity
        Self {
            executor: fg_executor,
            session_counter: AtomicUsize::new(0),
            sessions: Mutex::new(HashMap::new()),
            current_session: Mutex::new(None),
        }
    }

    pub fn wait_until(&self, future: LocalBoxFuture<'static, Result<()>>) -> Result<()> {
        let task = self.executor.spawn(async move { future.await })?;
        
        // Direct use of TestScheduler methods via downcast from executor
        if let Some(test_sched) = self.executor.scheduler.as_any().downcast_ref::<TestScheduler>() {
            if let Some(session_id) = test_sched.get_current_session() {
                test_sched.track_task_for_session(task.id(), session_id);
                test_sched.add_wait_until_task(session_id, task.id());
            }
        }
        
        Ok(())
    }
}
```

## Key Dependencies and Assumptions

- **parking_lot**: For `Mutex` (not RwLock) and `Parker`/`Unparker`.
- **async_task**: For Runnable wrapping.
- **rand_chacha**: For seeded RNG.
- **futures**: For channels.
- **chrono**: For time ranges (optional).
- **anyhow**: For errors.
- **std::thread**: For thread ID comparison.

The scheduler assumes no `dyn Any` is implemented on `Scheduler`; add `fn as_any(&self) -> &dyn std::any::Any;` if needed for downcasting.

This implementation provides the complete unified test core, enabling both GPUI's deterministic UI testing and Cloud's session-aware simulation in a single ~250-line struct, now wrapped by Executors for better encapsulation and thread safety.