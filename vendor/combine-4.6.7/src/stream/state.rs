use crate::{
    error::ParseResult,
    stream::{Positioned, RangeStreamOnce, ResetStream, StreamErrorFor, StreamOnce},
};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct Stream<S, U> {
    pub stream: S,
    pub state: U,
}

impl<S, U> Positioned for Stream<S, U>
where
    S: Positioned,
{
    #[inline]
    fn position(&self) -> Self::Position {
        self.stream.position()
    }
}

impl<S, U> ResetStream for Stream<S, U>
where
    S: ResetStream,
{
    type Checkpoint = S::Checkpoint;

    #[inline]
    fn checkpoint(&self) -> Self::Checkpoint {
        self.stream.checkpoint()
    }

    #[inline]
    fn reset(&mut self, checkpoint: Self::Checkpoint) -> Result<(), Self::Error> {
        self.stream.reset(checkpoint)
    }
}

impl<S, U> StreamOnce for Stream<S, U>
where
    S: StreamOnce,
{
    type Token = S::Token;
    type Range = S::Range;
    type Position = S::Position;
    type Error = S::Error;

    #[inline]
    fn uncons(&mut self) -> Result<S::Token, StreamErrorFor<Self>> {
        self.stream.uncons()
    }

    fn is_partial(&self) -> bool {
        self.stream.is_partial()
    }
}

impl<S, U> RangeStreamOnce for Stream<S, U>
where
    S: RangeStreamOnce,
{
    #[inline]
    fn uncons_range(&mut self, size: usize) -> Result<Self::Range, StreamErrorFor<Self>> {
        self.stream.uncons_range(size)
    }

    #[inline]
    fn uncons_while<F>(&mut self, f: F) -> Result<Self::Range, StreamErrorFor<Self>>
    where
        F: FnMut(Self::Token) -> bool,
    {
        self.stream.uncons_while(f)
    }

    fn uncons_while1<F>(&mut self, f: F) -> ParseResult<Self::Range, StreamErrorFor<Self>>
    where
        F: FnMut(Self::Token) -> bool,
    {
        self.stream.uncons_while1(f)
    }

    #[inline]
    fn distance(&self, end: &Self::Checkpoint) -> usize {
        self.stream.distance(end)
    }

    #[inline]
    fn range(&self) -> Self::Range {
        self.stream.range()
    }
}
