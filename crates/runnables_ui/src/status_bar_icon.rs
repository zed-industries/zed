use futures::stream::FuturesUnordered;
use futures::stream::StreamExt;
use gpui::{AppContext, Context as _, Model, Task};
use runnable::RunnableHandle;
use ui::Color;

type Succeeded = bool;
/// Tracks status of collapsed runnables panel;
/// tl;dr: it implements that bit where the status bar icon changes color depending on
/// the state of a task.
pub(super) struct StatusIconTracker {
    /// Tracks the state of currently executing tasks;
    /// None -> none of the tasks have failed, though there are still tasks underway.
    /// Some(true) -> all of the tasks have succeeded.
    /// Some(false) -> at least one of the tasks has failed.
    current_status: Option<Succeeded>,
    /// We keep around a handle to the status updater in case the user reopens the panel - in that case, we want to stop polling previous set of the tasks.
    /// That is achieved by creating new `RunnablesStatusBarIcon`, thus we want to stop polling in the old one (once it's dropped).
    _task_poller: Task<()>,
}

impl StatusIconTracker {
    pub(crate) fn new<'a>(tasks: Vec<RunnableHandle>, cx: &mut AppContext) -> Model<Self> {
        cx.new_model(|cx| {
            let mut futures: FuturesUnordered<RunnableHandle> = tasks.into_iter().collect();
            let _task_poller = cx.spawn(|this, mut cx| async move {
                while let Some(Ok(i)) = futures.next().await {
                    if i.status.is_err() {
                        // At least one task has failed, move to failure state; note though that regardless of us bailing there,
                        // the remaining runnables are still gonna run to completion (as we're not the only party polling these futures).
                        this.update(&mut cx, |this: &mut Self, cx| {
                            this.current_status = Some(false);
                            cx.notify()
                        })
                        .ok();
                        return;
                    }
                }
                // All tasks were either cancelled or succeeded, move to success state.
                if let Some(this) = this.upgrade() {
                    this.update(&mut cx, |this: &mut Self, cx| {
                        this.current_status = Some(true);
                        cx.notify();
                    })
                    .ok();
                }
            });
            Self {
                current_status: None,
                _task_poller,
            }
        })
    }
    pub(crate) fn color(&self) -> Color {
        match self.current_status {
            Some(true) => Color::Success,
            Some(false) => Color::Error,
            None => Color::Modified,
        }
    }
}
