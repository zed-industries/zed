// Allow tide Results to accept context like other Results do when
// using anyhow.
pub trait TideResultExt {
    fn context<C>(self, cx: C) -> Self
    where
        C: std::fmt::Display + Send + Sync + 'static;

    fn with_context<C, F>(self, f: F) -> Self
    where
        C: std::fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> C;
}

impl<T> TideResultExt for tide::Result<T> {
    fn context<C>(self, cx: C) -> Self
    where
        C: std::fmt::Display + Send + Sync + 'static,
    {
        self.map_err(|e| tide::Error::new(e.status(), e.into_inner().context(cx)))
    }

    fn with_context<C, F>(self, f: F) -> Self
    where
        C: std::fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        self.map_err(|e| tide::Error::new(e.status(), e.into_inner().context(f())))
    }
}
