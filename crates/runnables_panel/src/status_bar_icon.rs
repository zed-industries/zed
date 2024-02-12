use futures::stream::FuturesUnordered;
use futures::stream::StreamExt;
use gpui::{AppContext, Context as _, Model, Subscription, Task};
use runnable::{ExecutionResult, TaskHandle};
use ui::Color;

type Succeeded = bool;
/// Tracks status of collapsed runnables panel;
/// tl;dr: it implements that bit where the status bar icon changes color depending on
/// the state of a task.
pub(super) struct RunnablesStatusBarIcon {
    /// Tracks the state of currently executing tasks;
    /// None -> none of the tasks have failed, though there are still tasks underway.
    /// Some(true) -> all of the tasks have succeeded.
    /// Some(false) -> at least one of the tasks has failed.
    current_status: Option<Succeeded>,
    _task_pooler: Task<()>,
}

impl RunnablesStatusBarIcon {
    pub(crate) fn new<'a>(tasks: Vec<TaskHandle>, cx: &mut AppContext) -> Model<Self> {
        cx.new_model(|cx| {
            let mut futures: FuturesUnordered<TaskHandle> = tasks.into_iter().collect();
            let _task_pooler = cx.spawn(|this, mut cx| async move {
                while let Some(Ok(i)) = futures.next().await {
                    if i.status.is_err() {
                        this.update(&mut cx, |this: &mut Self, cx| {
                            this.current_status = Some(false);
                            cx.notify()
                        })
                        .ok();
                        return;
                    }
                }
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
                _task_pooler,
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
