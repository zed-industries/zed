use core::num::NonZeroUsize;

use crate::error::Needed;
#[cfg(feature = "unstable-recover")]
#[cfg(feature = "std")]
use crate::stream::Recover;
use crate::stream::{Checkpoint, Offset, Stream, StreamIsPartial};

/// Bit-level stream state over a byte stream.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Bits<I>(pub I, pub usize);

impl<I> Stream for Bits<I>
where
    I: Stream<Token = u8> + Clone,
{
    type Token = bool;
    type Slice = (I::Slice, usize, usize);

    type IterOffsets = BitOffsets<I>;
    type Checkpoint = Checkpoint<Bits<I::Checkpoint>, Self>;

    #[inline(always)]
    fn iter_offsets(&self) -> Self::IterOffsets {
        BitOffsets {
            i: self.clone(),
            o: 0,
        }
    }

    #[inline(always)]
    fn eof_offset(&self) -> usize {
        let offset = self.0.eof_offset() * 8;
        if offset == 0 {
            0
        } else {
            offset - self.1
        }
    }

    #[inline(always)]
    fn next_token(&mut self) -> Option<Self::Token> {
        next_bit(self)
    }

    #[inline(always)]
    fn peek_token(&self) -> Option<Self::Token> {
        peek_bit(self)
    }

    #[inline(always)]
    fn offset_for<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Token) -> bool,
    {
        self.iter_offsets()
            .find_map(|(o, b)| predicate(b).then_some(o))
    }

    #[inline(always)]
    fn offset_at(&self, tokens: usize) -> Result<usize, Needed> {
        if let Some(needed) = tokens
            .checked_sub(self.eof_offset())
            .and_then(NonZeroUsize::new)
        {
            Err(Needed::Size(needed))
        } else {
            Ok(tokens)
        }
    }

    #[inline(always)]
    fn next_slice(&mut self, offset: usize) -> Self::Slice {
        let byte_offset = (offset + self.1) / 8;
        let end_offset = (offset + self.1) % 8;
        let s = self.0.next_slice(byte_offset);
        let start_offset = self.1;
        self.1 = end_offset;
        (s, start_offset, end_offset)
    }

    #[inline(always)]
    fn peek_slice(&self, offset: usize) -> Self::Slice {
        let byte_offset = (offset + self.1) / 8;
        let end_offset = (offset + self.1) % 8;
        let s = self.0.peek_slice(byte_offset);
        let start_offset = self.1;
        (s, start_offset, end_offset)
    }

    #[inline(always)]
    fn checkpoint(&self) -> Self::Checkpoint {
        Checkpoint::<_, Self>::new(Bits(self.0.checkpoint(), self.1))
    }

    #[inline(always)]
    fn reset(&mut self, checkpoint: &Self::Checkpoint) {
        self.0.reset(&checkpoint.inner.0);
        self.1 = checkpoint.inner.1;
    }

    fn trace(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:#?}")
    }
}

/// Iterator for [bit][crate::binary::bits] stream ([`Bits`])
pub struct BitOffsets<I> {
    i: Bits<I>,
    o: usize,
}

impl<I> Iterator for BitOffsets<I>
where
    I: Stream<Token = u8> + Clone,
{
    type Item = (usize, bool);

    fn next(&mut self) -> Option<Self::Item> {
        let b = next_bit(&mut self.i)?;
        let o = self.o;
        self.o += 1;
        Some((o, b))
    }
}

fn next_bit<I>(i: &mut Bits<I>) -> Option<bool>
where
    I: Stream<Token = u8> + Clone,
{
    if i.eof_offset() == 0 {
        return None;
    }

    let offset = i.1;
    let mut next_i = i.0.clone();
    let byte = next_i.next_token()?;
    let bit = (byte >> offset) & 0x1 == 0x1;

    let next_offset = offset + 1;
    if next_offset == 8 {
        i.0 = next_i;
        i.1 = 0;
    } else {
        i.1 = next_offset;
    }
    Some(bit)
}

fn peek_bit<I>(i: &Bits<I>) -> Option<bool>
where
    I: Stream<Token = u8> + Clone,
{
    if i.eof_offset() == 0 {
        return None;
    }

    let offset = i.1;
    let mut next_i = i.0.clone();
    let byte = next_i.next_token()?;
    Some((byte >> offset) & 0x1 == 0x1)
}

#[cfg(feature = "unstable-recover")]
#[cfg(feature = "std")]
impl<I, E> Recover<E> for Bits<I>
where
    I: Recover<E>,
    I: Stream<Token = u8> + Clone,
{
    #[inline(always)]
    fn record_err(
        &mut self,
        _token_start: &Self::Checkpoint,
        _err_start: &Self::Checkpoint,
        err: E,
    ) -> Result<(), E> {
        Err(err)
    }

    #[inline(always)]
    fn is_recovery_supported() -> bool {
        false
    }
}

impl<I> StreamIsPartial for Bits<I>
where
    I: StreamIsPartial,
{
    type PartialState = I::PartialState;

    #[inline]
    fn complete(&mut self) -> Self::PartialState {
        self.0.complete()
    }

    #[inline]
    fn restore_partial(&mut self, state: Self::PartialState) {
        self.0.restore_partial(state);
    }

    #[inline(always)]
    fn is_partial_supported() -> bool {
        I::is_partial_supported()
    }

    #[inline(always)]
    fn is_partial(&self) -> bool {
        self.0.is_partial()
    }
}

impl<I> Offset for Bits<I>
where
    I: Offset,
{
    #[inline(always)]
    fn offset_from(&self, start: &Self) -> usize {
        self.0.offset_from(&start.0) * 8 + self.1 - start.1
    }
}

impl<I> Offset<<Bits<I> as Stream>::Checkpoint> for Bits<I>
where
    I: Stream<Token = u8> + Clone,
{
    #[inline(always)]
    fn offset_from(&self, other: &<Bits<I> as Stream>::Checkpoint) -> usize {
        self.checkpoint().offset_from(other)
    }
}

impl<I: Clone> crate::error::ErrorConvert<crate::error::InputError<Bits<I>>>
    for crate::error::InputError<I>
{
    #[inline]
    fn convert(self) -> crate::error::InputError<Bits<I>> {
        self.map_input(|i| Bits(i, 0))
    }
}

impl<I: Clone> crate::error::ErrorConvert<crate::error::InputError<I>>
    for crate::error::InputError<Bits<I>>
{
    #[inline]
    fn convert(self) -> crate::error::InputError<I> {
        self.map_input(|Bits(i, _o)| i)
    }
}
