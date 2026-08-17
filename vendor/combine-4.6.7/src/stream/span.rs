use crate::lib::marker::PhantomData;

use crate::{
    error::{ParseErrorInto, ParseResult, StreamErrorInto},
    stream::{ResetStream, StreamErrorFor},
    Positioned, RangeStream, RangeStreamOnce, StreamOnce,
};

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span<P> {
    pub start: P,
    pub end: P,
}

impl<P> From<P> for Span<P>
where
    P: Clone,
{
    #[inline]
    fn from(p: P) -> Self {
        Self {
            start: p.clone(),
            end: p,
        }
    }
}

impl<P> Span<P> {
    pub fn map<Q>(self, mut f: impl FnMut(P) -> Q) -> Span<Q> {
        Span {
            start: f(self.start),
            end: f(self.end),
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct Stream<S, E>(pub S, PhantomData<fn(E) -> E>);

impl<S, E> From<S> for Stream<S, E> {
    fn from(stream: S) -> Self {
        Stream(stream, PhantomData)
    }
}

impl<S, E> ResetStream for Stream<S, E>
where
    S: ResetStream + Positioned,
    S::Token: PartialEq,
    S::Range: PartialEq,
    E: crate::error::ParseError<S::Token, S::Range, Span<S::Position>>,
    S::Error: ParseErrorInto<S::Token, S::Range, S::Position>,
    <S::Error as crate::error::ParseError<S::Token, S::Range, S::Position>>::StreamError:
        StreamErrorInto<S::Token, S::Range>,
{
    type Checkpoint = S::Checkpoint;

    #[inline]
    fn checkpoint(&self) -> Self::Checkpoint {
        self.0.checkpoint()
    }

    #[inline]
    fn reset(&mut self, checkpoint: Self::Checkpoint) -> Result<(), Self::Error> {
        self.0
            .reset(checkpoint)
            .map_err(ParseErrorInto::into_other_error)
    }
}

impl<S, E> StreamOnce for Stream<S, E>
where
    S: StreamOnce + Positioned,
    S::Token: PartialEq,
    S::Range: PartialEq,
    E: crate::error::ParseError<S::Token, S::Range, Span<S::Position>>,
    S::Error: ParseErrorInto<S::Token, S::Range, S::Position>,
    <S::Error as crate::error::ParseError<S::Token, S::Range, S::Position>>::StreamError:
        StreamErrorInto<S::Token, S::Range>,
{
    type Token = S::Token;
    type Range = S::Range;
    type Position = Span<S::Position>;
    type Error = E;

    #[inline]
    fn uncons(&mut self) -> Result<Self::Token, StreamErrorFor<Self>> {
        self.0.uncons().map_err(StreamErrorInto::into_other_error)
    }

    #[inline]
    fn is_partial(&self) -> bool {
        self.0.is_partial()
    }
}

impl<S, E> RangeStreamOnce for Stream<S, E>
where
    S: RangeStream,
    S::Token: PartialEq,
    S::Range: PartialEq,
    E: crate::error::ParseError<S::Token, S::Range, Span<S::Position>>,
    S::Error: ParseErrorInto<S::Token, S::Range, S::Position>,
    <S::Error as crate::error::ParseError<S::Token, S::Range, S::Position>>::StreamError:
        StreamErrorInto<S::Token, S::Range>,
{
    #[inline]
    fn uncons_range(&mut self, size: usize) -> Result<Self::Range, StreamErrorFor<Self>> {
        self.0
            .uncons_range(size)
            .map_err(StreamErrorInto::into_other_error)
    }

    #[inline]
    fn uncons_while<F>(&mut self, f: F) -> Result<Self::Range, StreamErrorFor<Self>>
    where
        F: FnMut(Self::Token) -> bool,
    {
        self.0
            .uncons_while(f)
            .map_err(StreamErrorInto::into_other_error)
    }

    #[inline]
    fn uncons_while1<F>(&mut self, f: F) -> ParseResult<Self::Range, StreamErrorFor<Self>>
    where
        F: FnMut(Self::Token) -> bool,
    {
        self.0
            .uncons_while1(f)
            .map_err(StreamErrorInto::into_other_error)
    }

    #[inline]
    fn distance(&self, end: &Self::Checkpoint) -> usize {
        self.0.distance(end)
    }

    fn range(&self) -> Self::Range {
        self.0.range()
    }
}

impl<S, E> Positioned for Stream<S, E>
where
    S: StreamOnce + Positioned,
    S::Token: PartialEq,
    S::Range: PartialEq,
    E: crate::error::ParseError<S::Token, S::Range, Span<S::Position>>,
    S::Error: ParseErrorInto<S::Token, S::Range, S::Position>,
    <S::Error as crate::error::ParseError<S::Token, S::Range, S::Position>>::StreamError:
        StreamErrorInto<S::Token, S::Range>,
{
    fn position(&self) -> Span<S::Position> {
        Span::from(self.0.position())
    }
}
