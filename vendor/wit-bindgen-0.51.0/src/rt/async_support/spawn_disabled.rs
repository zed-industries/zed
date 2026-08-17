use crate::rt::async_support::BoxFuture;
use std::task::{Context, Poll};

#[derive(Default)]
pub struct Tasks<'a> {
    future: Option<BoxFuture<'a>>,
}

impl<'a> Tasks<'a> {
    pub fn new(root: BoxFuture<'a>) -> Tasks<'a> {
        Tasks { future: Some(root) }
    }

    pub fn poll_next(&mut self, cx: &mut Context<'_>) -> Poll<Option<()>> {
        if let Some(future) = self.future.as_mut() {
            if future.as_mut().poll(cx).is_ready() {
                self.future = None;
            }
        }
        if self.is_empty() {
            Poll::Ready(None)
        } else {
            Poll::Pending
        }
    }

    pub fn is_empty(&self) -> bool {
        self.future.is_none()
    }
}
