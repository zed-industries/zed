use crate::future::Future;
use crate::loom::sync::Arc;
use crate::runtime::scheduler::multi_thread::worker;
use crate::runtime::task::{Notified, Task, TaskHarnessScheduleHooks};
use crate::runtime::{
    blocking, driver,
    task::{self, JoinHandle, SpawnLocation},
    TaskHooks, TaskMeta, TimerFlavor,
};
use crate::util::RngSeedGenerator;

use std::fmt;
use std::num::NonZeroU64;

mod metrics;

cfg_taskdump! {
    mod taskdump;
}

#[cfg(all(tokio_unstable, feature = "time"))]
use crate::loom::sync::atomic::{AtomicBool, Ordering::SeqCst};

/// Handle to the multi thread scheduler
pub(crate) struct Handle {
    /// The name of the runtime
    pub(super) name: Option<String>,

    /// Task spawner
    pub(super) shared: worker::Shared,

    /// Resource driver handles
    pub(crate) driver: driver::Handle,

    /// Blocking pool spawner
    pub(crate) blocking_spawner: blocking::Spawner,

    /// Current random number generator seed
    pub(crate) seed_generator: RngSeedGenerator,

    /// User-supplied hooks to invoke for things
    pub(crate) task_hooks: TaskHooks,

    #[cfg_attr(not(feature = "time"), allow(dead_code))]
    /// Timer flavor used by the runtime
    pub(crate) timer_flavor: TimerFlavor,

    #[cfg(all(tokio_unstable, feature = "time"))]
    /// Indicates that the runtime is shutting down.
    pub(crate) is_shutdown: AtomicBool,
}

impl Handle {
    /// Spawns a future onto the thread pool
    pub(crate) fn spawn<F>(
        me: &Arc<Self>,
        future: F,
        id: task::Id,
        spawned_at: SpawnLocation,
    ) -> JoinHandle<F::Output>
    where
        F: crate::future::Future + Send + 'static,
        F::Output: Send + 'static,
    {
        Self::bind_new_task(me, future, id, spawned_at)
    }

    #[cfg(all(tokio_unstable, feature = "time"))]
    pub(crate) fn is_shutdown(&self) -> bool {
        self.is_shutdown
            .load(crate::loom::sync::atomic::Ordering::SeqCst)
    }

    pub(crate) fn shutdown(&self) {
        self.close();
        #[cfg(all(tokio_unstable, feature = "time"))]
        self.is_shutdown.store(true, SeqCst);
    }

    #[track_caller]
    pub(super) fn bind_new_task<T>(
        me: &Arc<Self>,
        future: T,
        id: task::Id,
        spawned_at: SpawnLocation,
    ) -> JoinHandle<T::Output>
    where
        T: Future + Send + 'static,
        T::Output: Send + 'static,
    {
        let (handle, notified) = me.shared.owned.bind(future, me.clone(), id, spawned_at);

        me.task_hooks.spawn(&TaskMeta {
            id,
            spawned_at,
            _phantom: Default::default(),
        });

        me.schedule_option_task_without_yield(notified);

        handle
    }
}

impl task::Schedule for Arc<Handle> {
    fn release(&self, task: &Task<Self>) -> Option<Task<Self>> {
        self.shared.owned.remove(task)
    }

    fn schedule(&self, task: Notified<Self>) {
        self.schedule_task(task, false);
    }

    fn hooks(&self) -> TaskHarnessScheduleHooks {
        TaskHarnessScheduleHooks {
            task_terminate_callback: self.task_hooks.task_terminate_callback.clone(),
        }
    }

    fn yield_now(&self, task: Notified<Self>) {
        self.schedule_task(task, true);
    }
}

impl Handle {
    pub(crate) fn owned_id(&self) -> NonZeroU64 {
        self.shared.owned.id
    }

    pub(crate) fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

impl fmt::Debug for Handle {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("multi_thread::Handle { ... }").finish()
    }
}
