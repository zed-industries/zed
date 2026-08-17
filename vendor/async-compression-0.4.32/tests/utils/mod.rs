#![allow(dead_code, unused_imports, unused_macros)] // Different tests use a different subset of functions

mod input_stream;
#[cfg(feature = "tokio")]
mod tokio_ext;
mod track_closed;
mod track_eof;
#[macro_use]
mod test_cases;

pub mod algos;
pub mod impls;

pub use self::{input_stream::InputStream, track_closed::TrackClosed, track_eof::TrackEof};
pub use compression_core::Level;
pub use futures::{executor::block_on, pin_mut, stream::Stream};
pub use std::{future::Future, io::Result, iter::FromIterator, pin::Pin};

pub fn one_to_six_stream() -> InputStream {
    InputStream::new(vec![vec![1, 2, 3], vec![4, 5, 6]])
}

pub fn one_to_six() -> &'static [u8] {
    &[1, 2, 3, 4, 5, 6]
}
