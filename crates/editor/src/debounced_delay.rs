use std::time::Duration;

use futures::{channel::oneshot, FutureExt};
use gpui::{Task, ViewContext};

use crate::Editor;

#[derive(Debug)]
pub struct DebouncedDelay {
    task: Option<Task<()>>,
    cancel_channel: Option<oneshot::Sender<()>>,
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
            _ = channel.send(());
        }

        let (sender, mut receiver) = oneshot::channel::<()>();
        self.cancel_channel = Some(sender);

        drop(self.task.take());
        self.task = Some(cx.spawn(move |model, mut cx| async move {
            let mut timer = cx.background_executor().timer(delay).fuse();
            futures::select_biased! {
                _ = receiver => return,
                _ = timer => {}
            }

            if let Ok(task) = model.update(&mut cx, |project, cx| (func)(project, cx)) {
                task.await;
            }
        }));
    }
}
