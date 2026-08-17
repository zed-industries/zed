// Copyright 2006 The Android Open Source Project
// Copyright 2020 Yevhenii Reizner
//
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use alloc::vec;
use alloc::vec::Vec;
use core::convert::TryFrom;
use core::num::NonZeroU16;

use crate::color::AlphaU8;
use crate::LengthU32;

pub type AlphaRun = Option<NonZeroU16>;

/// Sparse array of run-length-encoded alpha (supersampling coverage) values.
///
/// Sparseness allows us to independently compose several paths into the
/// same AlphaRuns buffer.
pub struct AlphaRuns {
    pub runs: Vec<AlphaRun>,
    pub alpha: Vec<u8>,
}

impl AlphaRuns {
    pub fn new(width: LengthU32) -> Self {
        let mut runs = AlphaRuns {
            runs: vec![None; (width.get() + 1) as usize],
            alpha: vec![0; (width.get() + 1) as usize],
        };
        runs.reset(width);
        runs
    }

    /// Returns 0-255 given 0-256.
    pub fn catch_overflow(alpha: u16) -> AlphaU8 {
        debug_assert!(alpha <= 256);
        (alpha - (alpha >> 8)) as u8
    }

    /// Returns true if the scanline contains only a single run, of alpha value 0.
    pub fn is_empty(&self) -> bool {
        debug_assert!(self.runs[0].is_some());
        match self.runs[0] {
            Some(run) => self.alpha[0] == 0 && self.runs[usize::from(run.get())].is_none(),
            None => true,
        }
    }

    /// Reinitialize for a new scanline.
    pub fn reset(&mut self, width: LengthU32) {
        let run = u16::try_from(width.get()).unwrap();
        self.runs[0] = NonZeroU16::new(run);
        self.runs[width.get() as usize] = None;
        self.alpha[0] = 0;
    }

    /// Insert into the buffer a run starting at (x-offset_x).
    ///
    /// if start_alpha > 0
    ///     one pixel with value += start_alpha,
    ///         max 255
    /// if middle_count > 0
    ///     middle_count pixels with value += max_value
    /// if stop_alpha > 0
    ///     one pixel with value += stop_alpha
    ///
    /// Returns the offset_x value that should be passed on the next call,
    /// assuming we're on the same scanline. If the caller is switching
    /// scanlines, then offset_x should be 0 when this is called.
    pub fn add(
        &mut self,
        x: u32,
        start_alpha: AlphaU8,
        mut middle_count: usize,
        stop_alpha: AlphaU8,
        max_value: u8,
        offset_x: usize,
    ) -> usize {
        let mut x = x as usize;

        let mut runs_offset = offset_x;
        let mut alpha_offset = offset_x;
        let mut last_alpha_offset = offset_x;
        x -= offset_x;

        if start_alpha != 0 {
            Self::break_run(
                &mut self.runs[runs_offset..],
                &mut self.alpha[alpha_offset..],
                x,
                1,
            );
            // I should be able to just add alpha[x] + start_alpha.
            // However, if the trailing edge of the previous span and the leading
            // edge of the current span round to the same super-sampled x value,
            // I might overflow to 256 with this add, hence the funny subtract (crud).
            let tmp = u16::from(self.alpha[alpha_offset + x]) + u16::from(start_alpha);
            debug_assert!(tmp <= 256);
            // was (tmp >> 7), but that seems wrong if we're trying to catch 256
            self.alpha[alpha_offset + x] = (tmp - (tmp >> 8)) as u8;

            runs_offset += x + 1;
            alpha_offset += x + 1;
            x = 0;
        }

        if middle_count != 0 {
            Self::break_run(
                &mut self.runs[runs_offset..],
                &mut self.alpha[alpha_offset..],
                x,
                middle_count,
            );
            alpha_offset += x;
            runs_offset += x;
            x = 0;
            loop {
                let a = Self::catch_overflow(
                    u16::from(self.alpha[alpha_offset]) + u16::from(max_value),
                );
                self.alpha[alpha_offset] = a;

                let n = usize::from(self.runs[runs_offset].unwrap().get());
                debug_assert!(n <= middle_count);
                alpha_offset += n;
                runs_offset += n;
                middle_count -= n;

                if middle_count == 0 {
                    break;
                }
            }

            last_alpha_offset = alpha_offset;
        }

        if stop_alpha != 0 {
            Self::break_run(
                &mut self.runs[runs_offset..],
                &mut self.alpha[alpha_offset..],
                x,
                1,
            );
            alpha_offset += x;
            self.alpha[alpha_offset] += stop_alpha;
            last_alpha_offset = alpha_offset;
        }

        // new offset_x
        last_alpha_offset
    }

    /// Break the runs in the buffer at offsets x and x+count, properly
    /// updating the runs to the right and left.
    ///
    /// i.e. from the state AAAABBBB, run-length encoded as A4B4,
    /// break_run(..., 2, 5) would produce AAAABBBB rle as A2A2B3B1.
    /// Allows add() to sum another run to some of the new sub-runs.
    /// i.e. adding ..CCCCC. would produce AADDEEEB, rle as A2D2E3B1.
    fn break_run(runs: &mut [AlphaRun], alpha: &mut [u8], mut x: usize, count: usize) {
        debug_assert!(count > 0);

        let orig_x = x;
        let mut runs_offset = 0;
        let mut alpha_offset = 0;

        while x > 0 {
            let n = usize::from(runs[runs_offset].unwrap().get());
            debug_assert!(n > 0);

            if x < n {
                alpha[alpha_offset + x] = alpha[alpha_offset];
                runs[runs_offset + 0] = NonZeroU16::new(x as u16);
                runs[runs_offset + x] = NonZeroU16::new((n - x) as u16);
                break;
            }
            runs_offset += n;
            alpha_offset += n;
            x -= n;
        }

        runs_offset = orig_x;
        alpha_offset = orig_x;
        x = count;

        loop {
            let n = usize::from(runs[runs_offset].unwrap().get());
            debug_assert!(n > 0);

            if x < n {
                alpha[alpha_offset + x] = alpha[alpha_offset];
                runs[runs_offset + 0] = NonZeroU16::new(x as u16);
                runs[runs_offset + x] = NonZeroU16::new((n - x) as u16);
                break;
            }

            x -= n;
            if x == 0 {
                break;
            }

            runs_offset += n;
            alpha_offset += n;
        }
    }

    /// Cut (at offset x in the buffer) a run into two shorter runs with
    /// matching alpha values.
    ///
    /// Used by the RectClipBlitter to trim a RLE encoding to match the
    /// clipping rectangle.
    pub fn break_at(alpha: &mut [AlphaU8], runs: &mut [AlphaRun], mut x: i32) {
        let mut alpha_i = 0;
        let mut run_i = 0;
        while x > 0 {
            let n = runs[run_i].unwrap().get();
            let n_usize = usize::from(n);
            let n_i32 = i32::from(n);
            if x < n_i32 {
                alpha[alpha_i + x as usize] = alpha[alpha_i];
                runs[0] = NonZeroU16::new(x as u16);
                runs[x as usize] = NonZeroU16::new((n_i32 - x) as u16);
                break;
            }

            run_i += n_usize;
            alpha_i += n_usize;
            x -= n_i32;
        }
    }
}
