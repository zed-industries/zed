use std::cmp;
use std::mem;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use super::SeekError;
use crate::common::{ChannelCount, SampleRate};
use crate::math::nz;
use crate::Source;

/// Internal function that builds a `Buffered` object.
#[inline]
pub fn buffered<I>(input: I) -> Buffered<I>
where
    I: Source,
{
    let total_duration = input.total_duration();
    let first_span = extract(input);

    Buffered {
        current_span: first_span,
        position_in_span: 0,
        total_duration,
    }
}

/// Iterator that at the same time extracts data from the iterator and stores it in a buffer.
pub struct Buffered<I>
where
    I: Source,
{
    /// Immutable reference to the next span of data. Cannot be `Span::Input`.
    current_span: Arc<Span<I>>,

    /// The position in number of samples of this iterator inside `current_span`.
    position_in_span: usize,

    /// Obtained once at creation and never modified again.
    total_duration: Option<Duration>,
}

enum Span<I>
where
    I: Source,
{
    /// Data that has already been extracted from the iterator. Also contains a pointer to the
    /// next span.
    Data(SpanData<I>),

    /// No more data.
    End,

    /// Unextracted data. The `Option` should never be `None` and is only here for easier data
    /// processing.
    Input(Mutex<Option<I>>),
}

struct SpanData<I>
where
    I: Source,
{
    data: Vec<I::Item>,
    channels: ChannelCount,
    rate: SampleRate,
    next: Mutex<Arc<Span<I>>>,
}

impl<I> Drop for SpanData<I>
where
    I: Source,
{
    fn drop(&mut self) {
        // This is necessary to prevent stack overflows deallocating long chains of the mutually
        // recursive `Span` and `SpanData` types. This iteratively traverses as much of the
        // chain as needs to be deallocated, and repeatedly "pops" the head off the list. This
        // solves the problem, as when the time comes to actually deallocate the `SpanData`,
        // the `next` field will contain a `Span::End`, or an `Arc` with additional references,
        // so the depth of recursive drops will be bounded.
        while let Ok(arc_next) = self.next.get_mut() {
            if let Some(next_ref) = Arc::get_mut(arc_next) {
                // This allows us to own the next Span.
                let next = mem::replace(next_ref, Span::End);
                if let Span::Data(next_data) = next {
                    // Swap the current SpanData with the next one, allowing the current one
                    // to go out of scope.
                    *self = next_data;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }
}

/// Builds a span from the input iterator.
fn extract<I>(mut input: I) -> Arc<Span<I>>
where
    I: Source,
{
    let span_len = input.current_span_len();

    if span_len == Some(0) {
        return Arc::new(Span::End);
    }

    let channels = input.channels();
    let rate = input.sample_rate();
    let data: Vec<I::Item> = input
        .by_ref()
        .take(cmp::min(span_len.unwrap_or(32768), 32768))
        .collect();

    if data.is_empty() {
        return Arc::new(Span::End);
    }

    Arc::new(Span::Data(SpanData {
        data,
        channels,
        rate,
        next: Mutex::new(Arc::new(Span::Input(Mutex::new(Some(input))))),
    }))
}

impl<I> Buffered<I>
where
    I: Source,
{
    /// Advances to the next span.
    fn next_span(&mut self) {
        let next_span = {
            let mut next_span_ptr = match &*self.current_span {
                Span::Data(SpanData { next, .. }) => next.lock().unwrap(),
                _ => unreachable!(),
            };

            let next_span = match &**next_span_ptr {
                Span::Data(_) => next_span_ptr.clone(),
                Span::End => next_span_ptr.clone(),
                Span::Input(input) => {
                    let input = input.lock().unwrap().take().unwrap();
                    extract(input)
                }
            };

            *next_span_ptr = next_span.clone();
            next_span
        };

        self.current_span = next_span;
        self.position_in_span = 0;
    }
}

impl<I> Iterator for Buffered<I>
where
    I: Source,
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        let current_sample;
        let advance_span;

        match &*self.current_span {
            Span::Data(SpanData { data, .. }) => {
                current_sample = Some(data[self.position_in_span]);
                self.position_in_span += 1;
                advance_span = self.position_in_span >= data.len();
            }

            Span::End => {
                current_sample = None;
                advance_span = false;
            }

            Span::Input(_) => unreachable!(),
        };

        if advance_span {
            self.next_span();
        }

        current_sample
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        // TODO:
        (0, None)
    }
}

// TODO: uncomment when `size_hint` is fixed
/*impl<I> ExactSizeIterator for Amplify<I> where I: Source + ExactSizeIterator, I::Item: Sample {
}*/

impl<I> Source for Buffered<I>
where
    I: Source,
{
    #[inline]
    fn current_span_len(&self) -> Option<usize> {
        match &*self.current_span {
            Span::Data(SpanData { data, .. }) => Some(data.len() - self.position_in_span),
            Span::End => Some(0),
            Span::Input(_) => unreachable!(),
        }
    }

    #[inline]
    fn channels(&self) -> ChannelCount {
        match *self.current_span {
            Span::Data(SpanData { channels, .. }) => channels,
            Span::End => nz!(1),
            Span::Input(_) => unreachable!(),
        }
    }

    #[inline]
    fn sample_rate(&self) -> SampleRate {
        match *self.current_span {
            Span::Data(SpanData { rate, .. }) => rate,
            Span::End => nz!(44100),
            Span::Input(_) => unreachable!(),
        }
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        self.total_duration
    }

    /// Can not support seek, in the end state we lose the underlying source
    /// which makes seeking back impossible.
    #[inline]
    fn try_seek(&mut self, _: Duration) -> Result<(), SeekError> {
        Err(SeekError::NotSupported {
            underlying_source: std::any::type_name::<Self>(),
        })
    }
}

impl<I> Clone for Buffered<I>
where
    I: Source,
{
    #[inline]
    fn clone(&self) -> Buffered<I> {
        Buffered {
            current_span: self.current_span.clone(),
            position_in_span: self.position_in_span,
            total_duration: self.total_duration,
        }
    }
}
