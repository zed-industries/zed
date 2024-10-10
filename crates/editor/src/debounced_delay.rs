use std::{ops::ControlFlow, time::Duration};

use futures::{channel::oneshot, FutureExt};
use gpui::{Task, ViewContext};

use crate::Editor;

pub struct DebouncedDelay {
    task: Option<Task<()>>,
    cancel_channel: Option<oneshot::Sender<ControlFlow<()>>>,
}

impl DebouncedDelay {
    pub fn new() -> DebouncedDelay {
        DebouncedDelay {
            task: None,
            cancel_channel: None,
        }
    }

    pub fn fire_new<F>(&mut self, delay: Duration, cx: &mut ViewContext<Editor>, func: F)
    where
        F: 'static + Send + FnOnce(&mut Editor, &mut ViewContext<Editor>) -> Task<()>,
    {
        if let Some(channel) = self.cancel_channel.take() {
            channel.send(ControlFlow::Break(())).ok();
        }

        let (sender, mut receiver) = oneshot::channel::<ControlFlow<()>>();
        self.cancel_channel = Some(sender);

        drop(self.task.take());
        self.task = Some(cx.spawn(move |model, mut cx| async move {
            let mut timer = cx.background_executor().timer(delay).fuse();
            futures::select_biased! {
                interrupt = receiver => {
                    match interrupt {
                        Ok(ControlFlow::Break(())) | Err(_) => return,
                        Ok(ControlFlow::Continue(())) => {},
                    }
                }
                _ = timer => {}
            }

            if let Ok(task) = model.update(&mut cx, |project, cx| (func)(project, cx)) {
                task.await;
            }
        }));
    }

    pub fn start_now(&mut self) -> Option<Task<()>> {
        if let Some(channel) = self.cancel_channel.take() {
            channel.send(ControlFlow::Continue(())).ok();
        }
        self.task.take()
    }
}
