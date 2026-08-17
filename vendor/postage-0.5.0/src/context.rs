use std::task::Waker;
/// The `Context` of an asynchronous task.
///
/// Unlike std::task::Context, this context *optionally* contains a waker.
///
/// This is necessary because try_send and try_recv need to poll,
/// but the task waker should not be saved if the try returns pending.
pub struct Context<'a> {
    waker: Option<&'a Waker>,
}

impl<'a> From<std::task::Context<'a>> for Context<'a> {
    fn from(cx: std::task::Context<'a>) -> Self {
        Self::from_waker(cx.waker())
    }
}

impl<'a> From<&std::task::Context<'a>> for Context<'a> {
    fn from(cx: &std::task::Context<'a>) -> Self {
        Self::from_waker(cx.waker())
    }
}

impl<'a> From<&mut std::task::Context<'a>> for Context<'a> {
    fn from(cx: &mut std::task::Context<'a>) -> Self {
        Self::from_waker(cx.waker())
    }
}

impl<'a> Context<'a> {
    /// Create a new `Context` from a `&Waker`.
    pub fn from_waker(waker: &'a Waker) -> Self {
        Context { waker: Some(waker) }
    }

    /// Create an empty context with no waker.
    pub fn empty() -> Self {
        Self { waker: None }
    }

    /// Returns an optional reference to the `Waker` for the current task.
    pub fn waker(&self) -> Option<&'a Waker> {
        self.waker
    }
}

impl std::fmt::Debug for Context<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Context")
            .field("waker", &self.waker)
            .finish()
    }
}
