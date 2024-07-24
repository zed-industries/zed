use futures::{channel::oneshot, FutureExt};
use gpui::{ModelContext, Task};
use std::{marker::PhantomData, time::Duration};

pub struct DebouncedDelay<E: 'static> {
    task: Option<Task<()>>,
    cancel_channel: Option<oneshot::Sender<()>>,
    _phantom_data: PhantomData<E>,
}

impl<E: 'static> DebouncedDelay<E> {
    pub fn new() -> Self {
        Self {
            task: None,
            cancel_channel: None,
            _phantom_data: PhantomData,
        }
    }

    pub fn fire_new<F>(&mut self, delay: Duration, cx: &mut ModelContext<E>, func: F)
    where
        F: 'static + Send + FnOnce(&mut E, &mut ModelContext<E>) -> Task<()>,
    {
        if let Some(channel) = self.cancel_channel.take() {
            _ = channel.send(());
        }

        let (sender, mut receiver) = oneshot::channel::<()>();
        self.cancel_channel = Some(sender);

        let previous_task = self.task.take();
        self.task = Some(cx.spawn(move |model, mut cx| async move {
            let mut timer = cx.background_executor().timer(delay).fuse();
            if let Some(previous_task) = previous_task {
                previous_task.await;
            }

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
