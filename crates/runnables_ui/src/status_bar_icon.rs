use futures::channel::mpsc::unbounded;
use futures::channel::mpsc::UnboundedReceiver;
use futures::channel::mpsc::UnboundedSender;
use futures::select_biased;
use futures::stream::FusedStream;
use futures::stream::FuturesUnordered;
use futures::stream::StreamExt;
use gpui::ModelContext;
use gpui::{AppContext, Context as _, Model, Task};
use runnable::Handle;
use ui::Color;

type Succeeded = bool;
/// Tracks status of collapsed runnables panel;
/// tl;dr: it implements that bit where the status bar icon changes color depending on
/// the state of a runnable.
pub(super) struct StatusIconTracker {
    /// Tracks the state of currently executing runnables;
    /// None -> none of the runnables have failed, though there are still runnables underway.
    /// Some(true) -> all of the runnables have succeeded.
    /// Some(false) -> at least one of the runnables has failed.
    current_status: Option<Succeeded>,
    /// We keep around a handle to the status updater in case the user reopens the panel - in that case, we want to stop polling previous set of the runnables.
    /// That is achieved by creating new `RunnablesStatusBarIcon`, thus we want to stop polling in the old one (once it's dropped).
    /// We also don't start it until we have at least one runnable running.
    _runnable_poller: Option<Task<()>>,
    tx: UnboundedSender<Handle>,
    rx: Option<UnboundedReceiver<Handle>>,
}

impl StatusIconTracker {
    pub(crate) fn new<'a>(runnables: Vec<Handle>, cx: &mut AppContext) -> Model<Self> {
        cx.new_model(|cx| {
            let (tx, rx) = unbounded::<Handle>();
            let mut ret = Self {
                current_status: None,
                _runnable_poller: None,
                tx,
                rx: Some(rx),
            };
            if !runnables.is_empty() {
                for runnable in runnables {
                    ret.tx.unbounded_send(runnable).unwrap();
                }
                ret.start_poller(cx);
            }
            ret
        })
    }

    fn start_poller(&mut self, cx: &mut ModelContext<Self>) {
        if let Some(mut rx) = self.rx.take() {
            self._runnable_poller = Some(cx.spawn(|this, mut cx| async move {
                let mut futures = FuturesUnordered::new();
                loop {
                    select_biased! {
                        new_handle = rx.next() => {
                            if let Some(new_handle) = new_handle {
                                this.update(&mut cx, |this: &mut Self, _cx| {
                                    this.current_status.take();
                                }).ok();
                                let mut handle_completion = new_handle.completion_rx().clone();
                                futures.push(async move {
                                    handle_completion.next().await
                                });
                            }

                        },
                        completion_status = futures.next() => {
                            if let Some(completion_status) = completion_status {
                                if completion_status.as_ref().map_or(false, |completion_status| !completion_status) {
                                    this.update(&mut cx, |this: &mut Self, cx| {
                                        this.current_status = Some(false);
                                        cx.notify()
                                    })
                                    .ok();
                                    return;
                                } else if completion_status.map_or(false, |completion_status| completion_status) && futures.is_empty() {
                                    this.update(&mut cx, |this: &mut Self, cx| {
                                        this.current_status = Some(true);
                                        cx.notify()
                                    })
                                    .ok();
                                }
                                dbg!(futures.len());
                            }
                        },
                        complete => {
                            dbg!(futures.len(), rx.is_terminated());
                        }

                    }
                }
            }));
        }
    }

    pub(crate) fn color(&self) -> Option<Color> {
        if self._runnable_poller.is_none() {
            return None;
        }
        let color = match self.current_status {
            Some(true) => Color::Success,
            Some(false) => Color::Error,
            None => Color::Modified,
        };
        Some(color)
    }
    pub(crate) fn push(&mut self, handle: Handle, cx: &mut ModelContext<Self>) {
        self.start_poller(cx);
        let _ = self.tx.unbounded_send(handle);
    }
}
