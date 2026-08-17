use crate::codec::ZmqCodec;

use asynchronous_codec::{FramedRead, FramedWrite};
use futures::{AsyncRead, AsyncWrite};

// Enables us to have multiple bounds on the dyn trait in `InnerFramed`
pub trait FrameableRead: AsyncRead + Unpin + Send + Sync {}
impl<T> FrameableRead for T where T: AsyncRead + Unpin + Send + Sync {}
pub trait FrameableWrite: AsyncWrite + Unpin + Send + Sync {}
impl<T> FrameableWrite for T where T: AsyncWrite + Unpin + Send + Sync {}

pub(crate) type ZmqFramedRead = asynchronous_codec::FramedRead<Box<dyn FrameableRead>, ZmqCodec>;
pub(crate) type ZmqFramedWrite = asynchronous_codec::FramedWrite<Box<dyn FrameableWrite>, ZmqCodec>;

/// Equivalent to [`asynchronous_codec::Framed<T, ZmqCodec>`]
pub struct FramedIo {
    pub read_half: ZmqFramedRead,
    pub write_half: ZmqFramedWrite,
}

impl FramedIo {
    pub fn new(read_half: Box<dyn FrameableRead>, write_half: Box<dyn FrameableWrite>) -> Self {
        let read_half = FramedRead::new(read_half, ZmqCodec::new());
        let write_half = FramedWrite::new(write_half, ZmqCodec::new());
        Self {
            read_half,
            write_half,
        }
    }

    pub fn into_parts(self) -> (ZmqFramedRead, ZmqFramedWrite) {
        (self.read_half, self.write_half)
    }
}
