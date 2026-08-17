use screencapturekit_sys::stream_error_handler::UnsafeSCStreamError;

// TODO: It might make sense to be a little more precise with lifetimes, than 'static.
// The lifetime could be potentially only as long as the relevant Stream, if the
// handler was dropped correctly together with the stream. For now the handler is never
// dropped and lives forever inside a statically allocated HashMap. See the relevant
// code in screencapturekit-sys crate.
pub trait StreamErrorHandler: Send + Sync + 'static {
    fn on_error(&self);
}

pub(crate) struct StreamErrorHandlerWrapper<T: StreamErrorHandler>(T);

impl<T: StreamErrorHandler> StreamErrorHandlerWrapper<T> {
    pub fn new(error_handler: T) -> Self {
        StreamErrorHandlerWrapper(error_handler)
    }
}

impl<T: StreamErrorHandler> UnsafeSCStreamError for StreamErrorHandlerWrapper<T> {
    fn handle_error(&self) {
        self.0.on_error();
    }
}
