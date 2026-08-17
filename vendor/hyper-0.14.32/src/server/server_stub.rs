use std::fmt;

use crate::common::exec::Exec;

/// A listening HTTP server that accepts connections in both HTTP1 and HTTP2 by default.
///
/// Needs at least one of the `http1` and `http2` features to be activated to actually be useful.
pub struct Server<I, S, E = Exec> {
    _marker: std::marker::PhantomData<(I, S, E)>,
}

impl<I: fmt::Debug, S: fmt::Debug> fmt::Debug for Server<I, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Server").finish()
    }
}
