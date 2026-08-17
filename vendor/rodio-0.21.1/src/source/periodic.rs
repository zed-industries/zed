use std::time::Duration;

use super::SeekError;
use crate::{
    common::{ChannelCount, SampleRate},
    Source,
};

/// Internal function that builds a `PeriodicAccess` object.
pub fn periodic<I, F>(source: I, period: Duration, modifier: F) -> PeriodicAccess<I, F>
where
    I: Source,
{
    // TODO: handle the fact that the samples rate can change
    let update_frequency = (period.as_secs_f32()
        * (source.sample_rate().get() as f32)
        * (source.channels().get() as f32)) as u32;

    PeriodicAccess {
        input: source,
        modifier,
        // Can overflow when subtracting if this is 0
        update_frequency: update_frequency.max(1),
        samples_until_update: 1,
    }
}

/// Calls a function on a source every time a period elapsed.
#[derive(Clone, Debug)]
pub struct PeriodicAccess<I, F> {
    // The inner source.
    input: I,

    // Closure that gets access to `inner`.
    modifier: F,

    // The frequency with which local_volume should be updated by remote_volume
    update_frequency: u32,

    // How many samples remain until it is time to update local_volume with remote_volume.
    samples_until_update: u32,
}

impl<I, F> PeriodicAccess<I, F>
where
    I: Source,

    F: FnMut(&mut I),
{
    /// Returns a reference to the inner source.
    #[inline]
    pub fn inner(&self) -> &I {
        &self.input
    }

    /// Returns a mutable reference to the inner source.
    #[inline]
    pub fn inner_mut(&mut self) -> &mut I {
        &mut self.input
    }

    /// Returns the inner source.
    #[inline]
    pub fn into_inner(self) -> I {
        self.input
    }
}

impl<I, F> Iterator for PeriodicAccess<I, F>
where
    I: Source,

    F: FnMut(&mut I),
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        self.samples_until_update -= 1;
        if self.samples_until_update == 0 {
            (self.modifier)(&mut self.input);
            self.samples_until_update = self.update_frequency;
        }

        self.input.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.input.size_hint()
    }
}

impl<I, F> Source for PeriodicAccess<I, F>
where
    I: Source,

    F: FnMut(&mut I),
{
    #[inline]
    fn current_span_len(&self) -> Option<usize> {
        self.input.current_span_len()
    }

    #[inline]
    fn channels(&self) -> ChannelCount {
        self.input.channels()
    }

    #[inline]
    fn sample_rate(&self) -> SampleRate {
        self.input.sample_rate()
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        self.input.total_duration()
    }

    #[inline]
    fn try_seek(&mut self, pos: Duration) -> Result<(), SeekError> {
        self.input.try_seek(pos)
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::time::Duration;

    use crate::buffer::SamplesBuffer;
    use crate::math::nz;
    use crate::source::Source;

    #[test]
    fn stereo_access() {
        // Stereo, 1Hz audio buffer
        let inner = SamplesBuffer::new(nz!(2), nz!(1), vec![10.0, -10.0, 10.0, -10.0, 20.0, -20.0]);

        let cnt = RefCell::new(0);

        let mut source = inner.periodic_access(Duration::from_millis(1000), |_src| {
            *cnt.borrow_mut() += 1;
        });

        assert_eq!(*cnt.borrow(), 0);
        // Always called on first access!
        assert_eq!(source.next(), Some(10.0));
        assert_eq!(*cnt.borrow(), 1);
        // Called every 1 second afterwards
        assert_eq!(source.next(), Some(-10.0));
        assert_eq!(*cnt.borrow(), 1);
        assert_eq!(source.next(), Some(10.0));
        assert_eq!(*cnt.borrow(), 2);
        assert_eq!(source.next(), Some(-10.0));
        assert_eq!(*cnt.borrow(), 2);
        assert_eq!(source.next(), Some(20.0));
        assert_eq!(*cnt.borrow(), 3);
        assert_eq!(source.next(), Some(-20.0));
        assert_eq!(*cnt.borrow(), 3);
    }

    #[test]
    fn fast_access_overflow() {
        // 1hz is lower than 0.5 samples per 5ms
        let inner = SamplesBuffer::new(nz!(1), nz!(1), vec![10.0, -10.0, 10.0, -10.0, 20.0, -20.0]);
        let mut source = inner.periodic_access(Duration::from_millis(5), |_src| {});

        source.next();
        source.next(); // Would overflow here.
    }
}
