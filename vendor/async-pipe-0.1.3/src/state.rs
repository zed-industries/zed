use std::task::Waker;

pub const BUFFER_SIZE: usize = 1024;

pub(crate) struct State {
    pub(crate) reader_waker: Option<Waker>,
    pub(crate) writer_waker: Option<Waker>,
    pub(crate) closed: bool,
    pub(crate) buffer: Vec<u8>,
}
