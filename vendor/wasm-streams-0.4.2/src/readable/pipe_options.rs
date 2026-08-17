use web_sys::AbortSignal;

use super::sys;

/// Options for [`pipe_to_with_options`](super::ReadableStream::pipe_to_with_options).
#[derive(Clone, Debug, Default)]
pub struct PipeOptions {
    raw: sys::PipeOptions,
}

impl PipeOptions {
    /// Creates a blank new set of pipe options.
    ///
    /// Equivalent to [`PipeOptions::default`](Default::default).
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a set of pipe options from a raw [`PipeOptions`](sys::PipeOptions) object.
    #[inline]
    pub fn from_raw(raw: sys::PipeOptions) -> Self {
        Self { raw }
    }

    /// Convert this to a raw [`PipeOptions`](sys::PipeOptions) object.
    #[inline]
    pub fn into_raw(self) -> sys::PipeOptions {
        self.raw
    }

    /// Sets whether the destination writable stream should be closed
    /// when the source readable stream closes.
    pub fn prevent_close(&mut self, prevent_close: bool) -> &mut Self {
        self.raw.set_prevent_close(prevent_close);
        self
    }

    /// Sets whether the source readable stream should be [canceled](https://streams.spec.whatwg.org/#cancel-a-readable-stream)
    /// when the destination writable stream errors.
    pub fn prevent_cancel(&mut self, prevent_cancel: bool) -> &mut Self {
        self.raw.set_prevent_cancel(prevent_cancel);
        self
    }

    /// Sets whether the destination writable stream should be [aborted](https://streams.spec.whatwg.org/#abort-a-writable-stream)
    /// when the source readable stream errors.
    pub fn prevent_abort(&mut self, prevent_abort: bool) -> &mut Self {
        self.raw.set_prevent_abort(prevent_abort);
        self
    }

    /// Sets an abort signal to abort the ongoing pipe operation.
    /// When the signal is aborted, the source readable stream will be canceled
    /// and the destination writable stream will be aborted
    /// unless the respective options [`prevent_cancel`](Self::prevent_cancel)
    /// or [`prevent_abort`](Self::prevent_abort) are set.
    pub fn signal(&mut self, signal: AbortSignal) -> &mut Self {
        self.raw.set_signal(&signal);
        self
    }
}
