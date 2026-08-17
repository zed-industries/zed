use crate::common::ChannelCount;
use crate::Sample;

/// Iterator that converts from a certain channel count to another.
#[derive(Clone, Debug)]
pub struct ChannelCountConverter<I>
where
    I: Iterator<Item = Sample>,
{
    input: I,
    from: ChannelCount,
    to: ChannelCount,
    sample_repeat: Option<Sample>,
    next_output_sample_pos: u16,
}

impl<I> ChannelCountConverter<I>
where
    I: Iterator<Item = Sample>,
{
    /// Initializes the iterator.
    ///
    /// # Panic
    ///
    /// Panics if `from` or `to` are equal to 0.
    ///
    #[inline]
    pub fn new(input: I, from: ChannelCount, to: ChannelCount) -> ChannelCountConverter<I> {
        ChannelCountConverter {
            input,
            from,
            to,
            sample_repeat: None,
            next_output_sample_pos: 0,
        }
    }

    /// Destroys this iterator and returns the underlying iterator.
    #[inline]
    pub fn into_inner(self) -> I {
        self.input
    }

    /// Get mutable access to the iterator
    #[inline]
    pub fn inner_mut(&mut self) -> &mut I {
        &mut self.input
    }
}

impl<I> Iterator for ChannelCountConverter<I>
where
    I: Iterator<Item = Sample>,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.next_output_sample_pos {
            0 => {
                // save first sample for mono -> stereo conversion
                let value = self.input.next();
                self.sample_repeat = value;
                value
            }
            x if x < self.from.get() => self.input.next(),
            1 => self.sample_repeat,
            _ => Some(0.0),
        };

        if result.is_some() {
            self.next_output_sample_pos += 1;
        }

        if self.next_output_sample_pos == self.to.get() {
            self.next_output_sample_pos = 0;

            if self.from > self.to {
                for _ in self.to.get()..self.from.get() {
                    self.input.next(); // discarding extra input
                }
            }
        }

        result
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (min, max) = self.input.size_hint();

        let consumed = std::cmp::min(self.from.get(), self.next_output_sample_pos) as usize;

        let min = ((min + consumed) / self.from.get() as usize * self.to.get() as usize)
            .saturating_sub(self.next_output_sample_pos as usize);

        let max = max.map(|max| {
            ((max + consumed) / self.from.get() as usize * self.to.get() as usize)
                .saturating_sub(self.next_output_sample_pos as usize)
        });

        (min, max)
    }
}

impl<I> ExactSizeIterator for ChannelCountConverter<I> where I: ExactSizeIterator<Item = Sample> {}

#[cfg(test)]
mod test {
    use super::ChannelCountConverter;
    use crate::common::ChannelCount;
    use crate::math::nz;
    use crate::Sample;

    #[test]
    fn remove_channels() {
        let input = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let output =
            ChannelCountConverter::new(input.into_iter(), nz!(3), nz!(2)).collect::<Vec<_>>();
        assert_eq!(output, [1.0, 2.0, 4.0, 5.0]);

        let input = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let output =
            ChannelCountConverter::new(input.into_iter(), nz!(4), nz!(1)).collect::<Vec<_>>();
        assert_eq!(output, [1.0, 5.0]);
    }

    #[test]
    fn add_channels() {
        let input = vec![1.0, 2.0, 3.0, 4.0];
        let output =
            ChannelCountConverter::new(input.into_iter(), nz!(1), nz!(2)).collect::<Vec<_>>();
        assert_eq!(output, [1.0, 1.0, 2.0, 2.0, 3.0, 3.0, 4.0, 4.0]);

        let input = vec![1.0, 2.0];
        let output =
            ChannelCountConverter::new(input.into_iter(), nz!(1), nz!(4)).collect::<Vec<_>>();
        assert_eq!(output, [1.0, 1.0, 0.0, 0.0, 2.0, 2.0, 0.0, 0.0]);

        let input = vec![1.0, 2.0, 3.0, 4.0];
        let output =
            ChannelCountConverter::new(input.into_iter(), nz!(2), nz!(4)).collect::<Vec<_>>();
        assert_eq!(output, [1.0, 2.0, 0.0, 0.0, 3.0, 4.0, 0.0, 0.0]);
    }

    #[test]
    fn size_hint() {
        fn test(input: &[Sample], from: ChannelCount, to: ChannelCount) {
            let mut converter = ChannelCountConverter::new(input.iter().copied(), from, to);
            let count = converter.clone().count();
            for left_in_iter in (0..=count).rev() {
                println!("left_in_iter = {left_in_iter}");
                assert_eq!(converter.size_hint(), (left_in_iter, Some(left_in_iter)));
                converter.next();
            }
            assert_eq!(converter.size_hint(), (0, Some(0)));
        }

        test(&[1.0, 2.0, 3.0], nz!(1), nz!(2));
        test(&[1.0, 2.0, 3.0, 4.0], nz!(2), nz!(4));
        test(&[1.0, 2.0, 3.0, 4.0], nz!(4), nz!(2));
        test(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0], nz!(3), nz!(8));
        test(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0], nz!(4), nz!(1));
    }

    #[test]
    fn len_more() {
        let input = vec![1.0, 2.0, 3.0, 4.0];
        let output = ChannelCountConverter::new(input.into_iter(), nz!(2), nz!(3));
        assert_eq!(output.len(), 6);
    }

    #[test]
    fn len_less() {
        let input = vec![1.0, 2.0, 3.0, 4.0];
        let output = ChannelCountConverter::new(input.into_iter(), nz!(2), nz!(1));
        assert_eq!(output.len(), 2);
    }
}
