use nom::error::{Error as NomError, ErrorKind, ParseError};
use nom::{
    AsBytes, Compare, CompareResult, Err as ErrorMode, ExtendInto, IResult, InputIter, InputLength,
    InputTake, InputTakeAtPosition, Needed, Offset, Parser, Slice,
};
use std::borrow::{Cow, ToOwned};
use std::fmt::{self, Display, Formatter};
use std::ops::{Deref, RangeFrom, RangeTo};

pub use nom;

pub trait Location {
    fn location(&self) -> usize;
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Located<'i, I>
where
    I: ?Sized,
{
    data: &'i I,
    location: usize,
}

impl<'i, I> Located<'i, I>
where
    I: ?Sized,
{
    pub fn into_data(self) -> &'i I {
        self.data
    }
}

impl<'i, I> AsBytes for Located<'i, I>
where
    I: ?Sized,
    &'i I: AsBytes,
{
    fn as_bytes(&self) -> &[u8] {
        self.data.as_bytes()
    }
}

impl<'i, I> AsRef<I> for Located<'i, I>
where
    I: ?Sized,
{
    fn as_ref(&self) -> &I {
        self.data
    }
}

impl<'i, I> Clone for Located<'i, I>
where
    I: ?Sized,
{
    fn clone(&self) -> Self {
        Located {
            data: self.data,
            location: self.location,
        }
    }
}

impl<'i, 'u, I, U> Compare<&'u U> for Located<'i, I>
where
    I: ?Sized,
    U: ?Sized,
    &'i I: Compare<&'u U>,
    &'u U: Into<Located<'u, U>>,
{
    fn compare(&self, other: &'u U) -> CompareResult {
        self.data.compare(other.into().data)
    }

    fn compare_no_case(&self, other: &'u U) -> CompareResult {
        self.data.compare_no_case(other.into().data)
    }
}

impl<'i, I> Copy for Located<'i, I> where I: ?Sized {}

impl<'i, I> Deref for Located<'i, I>
where
    I: ?Sized,
{
    type Target = I;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<'i, I> Display for Located<'i, I>
where
    I: Display + ?Sized,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self.data, f)
    }
}

impl<'i, I> ExtendInto for Located<'i, I>
where
    I: ?Sized,
    &'i I: ExtendInto,
{
    type Item = <&'i I as ExtendInto>::Item;
    type Extender = <&'i I as ExtendInto>::Extender;

    fn new_builder(&self) -> Self::Extender {
        self.data.new_builder()
    }

    fn extend_into(&self, extender: &mut Self::Extender) {
        self.data.extend_into(extender)
    }
}

impl<'i, I> From<Located<'i, I>> for Cow<'i, I>
where
    I: ?Sized + ToOwned,
{
    fn from(fragment: Located<'i, I>) -> Self {
        Cow::Borrowed(fragment.data)
    }
}

impl<'i, I> From<&'i I> for Located<'i, I>
where
    I: ?Sized,
{
    fn from(data: &'i I) -> Self {
        Located { data, location: 0 }
    }
}

impl<'i, I> InputIter for Located<'i, I>
where
    I: ?Sized,
    &'i I: InputIter,
{
    type Item = <&'i I as InputIter>::Item;
    type Iter = <&'i I as InputIter>::Iter;
    type IterElem = <&'i I as InputIter>::IterElem;

    fn iter_indices(&self) -> Self::Iter {
        self.data.iter_indices()
    }

    fn iter_elements(&self) -> Self::IterElem {
        self.data.iter_elements()
    }

    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.data.position(predicate)
    }

    fn slice_index(&self, count: usize) -> Result<usize, Needed> {
        self.data.slice_index(count)
    }
}

impl<'i, I> InputLength for Located<'i, I>
where
    I: ?Sized,
    &'i I: InputLength,
{
    fn input_len(&self) -> usize {
        self.data.input_len()
    }
}

impl<'i, I> InputTake for Located<'i, I>
where
    I: ?Sized,
    Self: Slice<RangeFrom<usize>> + Slice<RangeTo<usize>>,
{
    fn take(&self, count: usize) -> Self {
        self.slice(..count)
    }

    fn take_split(&self, count: usize) -> (Self, Self) {
        (self.slice(count..), self.slice(..count))
    }
}

impl<'i, I> InputTakeAtPosition for Located<'i, I>
where
    I: ?Sized,
    &'i I: InputIter + InputLength + InputTakeAtPosition,
    Self: Slice<RangeFrom<usize>> + Slice<RangeTo<usize>>,
{
    type Item = <&'i I as InputIter>::Item;

    fn split_at_position_complete<P, E>(&self, predicate: P) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
        E: ParseError<Self>,
    {
        match self.split_at_position(predicate) {
            Err(ErrorMode::Incomplete(_)) => Ok(self.take_split(self.input_len())),
            result => result,
        }
    }

    fn split_at_position<P, E>(&self, predicate: P) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
        E: ParseError<Self>,
    {
        match self.data.position(predicate) {
            Some(n) => Ok(self.take_split(n)),
            None => Err(ErrorMode::Incomplete(Needed::new(1))),
        }
    }

    fn split_at_position1<P, E>(&self, predicate: P, kind: ErrorKind) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
        E: ParseError<Self>,
    {
        match self.data.position(predicate) {
            Some(0) => Err(ErrorMode::Error(E::from_error_kind(*self, kind))),
            Some(n) => Ok(self.take_split(n)),
            None => Err(ErrorMode::Incomplete(Needed::new(1))),
        }
    }

    fn split_at_position1_complete<P, E>(
        &self,
        predicate: P,
        kind: ErrorKind,
    ) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
        E: ParseError<Self>,
    {
        match self.data.position(predicate) {
            Some(0) => Err(ErrorMode::Error(E::from_error_kind(*self, kind))),
            Some(n) => Ok(self.take_split(n)),
            None => {
                if self.data.input_len() == 0 {
                    Err(ErrorMode::Error(E::from_error_kind(*self, kind)))
                }
                else {
                    Ok(self.take_split(self.input_len()))
                }
            }
        }
    }
}

impl<'i, I> Location for Located<'i, I>
where
    I: ?Sized,
{
    fn location(&self) -> usize {
        self.location
    }
}

impl<'i, I> Offset for Located<'i, I>
where
    I: ?Sized,
{
    fn offset(&self, other: &Self) -> usize {
        other.location.saturating_sub(self.location)
    }
}

impl<'i, I, R> Slice<R> for Located<'i, I>
where
    I: ?Sized,
    &'i I: AsBytes + Offset + Slice<R> + Slice<RangeTo<usize>>,
{
    fn slice(&self, range: R) -> Self {
        let sliced = self.data.slice(range);
        let offset = self.data.offset(&sliced);
        Located {
            data: sliced,
            location: self.location + offset,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Stateful<I, T> {
    data: I,
    pub state: T,
}

impl<I, T> Stateful<I, T> {
    pub fn new(data: I, state: T) -> Self {
        Stateful { data, state }
    }

    fn clone_map<F>(&self, mut f: F) -> Self
    where
        T: Clone,
        F: FnMut(&I) -> I,
    {
        Stateful {
            data: f(&self.data),
            state: self.state.clone(),
        }
    }

    fn clone_map_result<E, F>(&self, f: F) -> IResult<Self, Self, E>
    where
        E: ParseError<Self>,
        T: Clone,
        F: FnOnce(&I) -> IResult<I, I>,
    {
        let map_error = |error: NomError<I>| {
            E::from_error_kind(
                Stateful {
                    data: error.input,
                    state: self.state.clone(),
                },
                error.code,
            )
        };
        f(&self.data)
            .map(|(remaining, output)| {
                (
                    Stateful {
                        data: remaining,
                        state: self.state.clone(),
                    },
                    Stateful {
                        data: output,
                        state: self.state.clone(),
                    },
                )
            })
            .map_err(|error| match error {
                ErrorMode::Error(error) => ErrorMode::Error(map_error(error)),
                ErrorMode::Failure(error) => ErrorMode::Failure(map_error(error)),
                ErrorMode::Incomplete(needed) => ErrorMode::Incomplete(needed),
            })
    }
}

impl<I, T> AsBytes for Stateful<I, T>
where
    I: AsBytes,
{
    fn as_bytes(&self) -> &[u8] {
        self.data.as_bytes()
    }
}

impl<I, T> AsRef<I> for Stateful<I, T> {
    fn as_ref(&self) -> &I {
        &self.data
    }
}

impl<I, T, U> Compare<U> for Stateful<I, T>
where
    I: Compare<U>,
{
    fn compare(&self, other: U) -> CompareResult {
        self.data.compare(other)
    }

    fn compare_no_case(&self, other: U) -> CompareResult {
        self.data.compare_no_case(other)
    }
}

impl<I, T> Deref for Stateful<I, T> {
    type Target = I;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<I, T> Display for Stateful<I, T>
where
    I: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.data, f)
    }
}

impl<I, T> ExtendInto for Stateful<I, T>
where
    I: ExtendInto,
{
    type Item = I::Item;
    type Extender = I::Extender;

    fn new_builder(&self) -> Self::Extender {
        self.data.new_builder()
    }

    fn extend_into(&self, extender: &mut Self::Extender) {
        self.data.extend_into(extender)
    }
}

impl<I, T> InputIter for Stateful<I, T>
where
    I: InputIter,
{
    type Item = I::Item;
    type Iter = I::Iter;
    type IterElem = I::IterElem;

    fn iter_indices(&self) -> Self::Iter {
        self.data.iter_indices()
    }

    fn iter_elements(&self) -> Self::IterElem {
        self.data.iter_elements()
    }

    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.data.position(predicate)
    }

    fn slice_index(&self, count: usize) -> Result<usize, Needed> {
        self.data.slice_index(count)
    }
}

impl<I, T> InputLength for Stateful<I, T>
where
    I: InputLength,
{
    fn input_len(&self) -> usize {
        self.data.input_len()
    }
}

impl<I, T> InputTake for Stateful<I, T>
where
    I: InputTake,
    T: Clone,
{
    fn take(&self, count: usize) -> Self {
        self.clone_map(move |data| data.take(count))
    }

    fn take_split(&self, count: usize) -> (Self, Self) {
        let (left, right) = self.data.take_split(count);
        (
            Stateful {
                data: left,
                state: self.state.clone(),
            },
            Stateful {
                data: right,
                state: self.state.clone(),
            },
        )
    }
}

impl<I, T> InputTakeAtPosition for Stateful<I, T>
where
    I: InputTakeAtPosition,
    T: Clone,
{
    type Item = <I as InputTakeAtPosition>::Item;

    fn split_at_position_complete<P, E>(&self, predicate: P) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
        E: ParseError<Self>,
    {
        self.clone_map_result(move |data| data.split_at_position_complete(predicate))
    }

    fn split_at_position<P, E>(&self, predicate: P) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
        E: ParseError<Self>,
    {
        self.clone_map_result(move |data| data.split_at_position(predicate))
    }

    fn split_at_position1<P, E>(&self, predicate: P, kind: ErrorKind) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
        E: ParseError<Self>,
    {
        self.clone_map_result(move |data| data.split_at_position1(predicate, kind))
    }

    fn split_at_position1_complete<P, E>(
        &self,
        predicate: P,
        kind: ErrorKind,
    ) -> IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
        E: ParseError<Self>,
    {
        self.clone_map_result(move |data| data.split_at_position1_complete(predicate, kind))
    }
}

impl<I, T> Location for Stateful<I, T>
where
    I: Location,
{
    fn location(&self) -> usize {
        self.data.location()
    }
}

impl<I, T> Offset for Stateful<I, T>
where
    I: Offset,
{
    fn offset(&self, other: &Self) -> usize {
        self.data.offset(&other.data)
    }
}

impl<I, T, R> Slice<R> for Stateful<I, T>
where
    I: Slice<R>,
    T: Clone,
{
    fn slice(&self, range: R) -> Self {
        let data = self.data.slice(range);
        Stateful {
            data,
            state: self.state.clone(),
        }
    }
}

pub fn bof<I, E>(input: I) -> IResult<I, I, E>
where
    I: Clone + Location,
    E: ParseError<I>,
{
    if input.location() == 0 {
        Ok((input.clone(), input))
    }
    else {
        Err(ErrorMode::Error(E::from_error_kind(input, ErrorKind::Eof)))
    }
}

pub fn span<I, O, E, F>(mut parser: F) -> impl FnMut(I) -> IResult<I, ((usize, usize), O), E>
where
    I: Clone + Location,
    E: ParseError<I>,
    F: Parser<I, O, E>,
{
    move |input: I| {
        let start = input.location();
        parser.parse(input).map(move |(remaining, output)| {
            let end = remaining.location();
            (remaining, ((start, end.saturating_sub(start)), output))
        })
    }
}
