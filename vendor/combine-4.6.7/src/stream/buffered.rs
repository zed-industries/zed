use alloc::collections::VecDeque;

use crate::{
    error::StreamError,
    stream::{ParseError, Positioned, ResetStream, StreamErrorFor, StreamOnce},
};

/// `Stream` which buffers items from an instance of `StreamOnce` into a ring buffer.
/// Instances of `StreamOnce` which is not able to implement `ResetStream` (such as `ReadStream`) may
/// use this as a way to implement `ResetStream` and become a full `Stream` instance.
///
/// The drawback is that the buffer only stores a limited number of items which limits how many
/// tokens that can be reset and replayed. If a `buffered::Stream` is reset past this limit an error
/// will be returned when `uncons` is next called.
///
/// NOTE: If this stream is used in conjunction with an error enhancing stream such as
/// `easy::Stream` (also via the `easy_parser` method) it is recommended that the `buffered::Stream`
/// instance wraps the `easy::Stream` instance instead of the other way around.
///
/// ```ignore
/// // DO
/// buffered::Stream::new(easy::Stream(..), ..)
/// // DON'T
/// easy::Stream(buffered::Stream::new(.., ..))
/// parser.easy_parse(buffered::Stream::new(..));
/// ```
#[derive(Debug, PartialEq)]
pub struct Stream<Input>
where
    Input: StreamOnce + Positioned,
{
    offset: usize,
    iter: Input,
    buffer_offset: usize,
    buffer: VecDeque<(Input::Token, Input::Position)>,
}

impl<Input> ResetStream for Stream<Input>
where
    Input: Positioned,
{
    type Checkpoint = usize;

    fn checkpoint(&self) -> Self::Checkpoint {
        self.offset
    }

    fn reset(&mut self, checkpoint: Self::Checkpoint) -> Result<(), Self::Error> {
        if checkpoint < self.buffer_offset - self.buffer.len() {
            // We have backtracked to far
            Err(Self::Error::from_error(
                self.position(),
                StreamErrorFor::<Self>::message_static_message("Backtracked to far"),
            ))
        } else {
            self.offset = checkpoint;
            Ok(())
        }
    }
}

impl<Input> Stream<Input>
where
    Input: StreamOnce + Positioned,
    Input::Position: Clone,
    Input::Token: Clone,
{
    /// Constructs a new `BufferedStream` from a `StreamOnce` instance with a `lookahead`
    /// number of elements that can be stored in the buffer.
    pub fn new(iter: Input, lookahead: usize) -> Stream<Input> {
        Stream {
            offset: 0,
            iter,
            buffer_offset: 0,
            buffer: VecDeque::with_capacity(lookahead),
        }
    }
}

impl<Input> Positioned for Stream<Input>
where
    Input: StreamOnce + Positioned,
{
    #[inline]
    fn position(&self) -> Self::Position {
        if self.offset >= self.buffer_offset {
            self.iter.position()
        } else if self.offset < self.buffer_offset - self.buffer.len() {
            self.buffer
                .front()
                .expect("At least 1 element in the buffer")
                .1
                .clone()
        } else {
            self.buffer[self.buffer.len() - (self.buffer_offset - self.offset)]
                .1
                .clone()
        }
    }
}

impl<Input> StreamOnce for Stream<Input>
where
    Input: StreamOnce + Positioned,
    Input::Token: Clone,
{
    type Token = Input::Token;
    type Range = Input::Range;
    type Position = Input::Position;
    type Error = Input::Error;

    #[inline]
    fn uncons(&mut self) -> Result<Input::Token, StreamErrorFor<Self>> {
        if self.offset >= self.buffer_offset {
            let position = self.iter.position();
            let token = self.iter.uncons()?;
            self.buffer_offset += 1;
            // We want the VecDeque to only keep the last .capacity() elements so we need to remove
            // an element if it gets to large
            if self.buffer.len() == self.buffer.capacity() {
                self.buffer.pop_front();
            }
            self.buffer.push_back((token.clone(), position));
            self.offset += 1;
            Ok(token)
        } else if self.offset < self.buffer_offset - self.buffer.len() {
            // We have backtracked to far
            Err(StreamError::message_static_message("Backtracked to far"))
        } else {
            let value = self.buffer[self.buffer.len() - (self.buffer_offset - self.offset)]
                .0
                .clone();
            self.offset += 1;
            Ok(value)
        }
    }

    fn is_partial(&self) -> bool {
        self.iter.is_partial()
    }
}
