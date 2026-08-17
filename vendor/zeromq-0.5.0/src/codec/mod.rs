//! Implements a codec for ZMQ, providing a way to convert from a byte-oriented
//! io device to a protocal comprised of [`Message`] frames. See [`FramedIo`]

mod command;
mod error;
mod framed;
mod greeting;
pub(crate) mod mechanism;
mod zmq_codec;

pub(crate) use command::{ZmqCommand, ZmqCommandName};
pub(crate) use error::{CodecError, CodecResult};
pub(crate) use framed::{FrameableRead, FrameableWrite, FramedIo, ZmqFramedRead, ZmqFramedWrite};
pub(crate) use greeting::{ZmqGreeting, ZmtpVersion};
pub(crate) use zmq_codec::ZmqCodec;

use crate::message::ZmqMessage;
use crate::{ZmqError, ZmqResult};
use futures::task::noop_waker;
use futures::Sink;

use std::pin::Pin;
use std::task::{Context, Poll};

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone)]
pub enum Message {
    Greeting(ZmqGreeting),
    Command(ZmqCommand),
    Message(ZmqMessage),
}

pub(crate) trait TrySend {
    fn try_send(self: Pin<&mut Self>, item: Message) -> ZmqResult<()>;
}

impl TrySend for ZmqFramedWrite {
    fn try_send(mut self: Pin<&mut Self>, item: Message) -> ZmqResult<()> {
        let waker = noop_waker();
        let mut cx = Context::from_waker(&waker);
        match self.as_mut().poll_ready(&mut cx) {
            Poll::Ready(Ok(())) => {
                self.as_mut().start_send(item)?;
                let _ = self.as_mut().poll_flush(&mut cx); // ignore result just hope that it flush eventually
                Ok(())
            }
            Poll::Ready(Err(e)) => Err(e.into()),
            Poll::Pending => Err(ZmqError::BufferFull("Sink is full")),
        }
    }
}
