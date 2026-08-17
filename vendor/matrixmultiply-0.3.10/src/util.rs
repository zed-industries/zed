// Copyright 2016 - 2018 Ulrik Sverdrup "bluss"
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core::cmp::min;

#[derive(Copy, Clone)]
pub struct RangeChunk { i: usize, n: usize, chunk: usize }

/// Create an iterator that splits `n` in chunks of size `chunk`;
/// the last item can be an uneven chunk.
pub fn range_chunk(n: usize, chunk: usize) -> RangeChunk {
    RangeChunk {
        i: 0,
        n: n,
        chunk: chunk,
    }
}

impl Iterator for RangeChunk {
    type Item = (usize, usize);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.n == 0 {
            None
        } else {
            let i = self.i;
            let rem = min(self.n, self.chunk);
            self.i += 1;
            self.n -= rem;
            Some((i, rem))
        }
    }
}

#[inline]
pub fn round_up_to(x: usize, multiple_of: usize) -> usize {
    let (mut d, r) = (x / multiple_of, x % multiple_of);
    if r > 0 { d += 1; }
    d * multiple_of
}

impl RangeChunk {
    #[cfg(feature="threading")]
    /// Split the iterator in `total` parts and only iterate the `index`th part of it.
    /// The iterator must not have started when this is called.
    pub(crate) fn part(self, index: usize, total: usize) -> Self {
        debug_assert_eq!(self.i, 0, "range must be uniterated");
        debug_assert_ne!(total, 0);
        let (n, chunk) = (self.n, self.chunk);

        // round up
        let mut nchunks = n / chunk;
        nchunks += (n % chunk != 0) as usize;

        // chunks per thread
        // round up
        let mut chunks_per = nchunks / total;
        chunks_per += (nchunks % total != 0) as usize;

        let i = chunks_per * index;
        let nn = min(n, (i + chunks_per) * chunk).saturating_sub(i * chunk);

        RangeChunk { i, n: nn, chunk }
    }
}
