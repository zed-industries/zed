// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use symphonia_core::dsp::mdct::Imdct;

use super::window::Windows;

pub struct LappingState {
    pub prev_block_flag: bool,
}

pub struct Dsp {
    /// DSP channels (max. 256 per-spec, but actually limited to 32 by Symphonia).
    pub channels: Vec<DspChannel>,
    /// IMDCT for short-blocks.
    pub imdct_short: Imdct,
    /// IMDCT for long-blocks.
    pub imdct_long: Imdct,
    /// Windows for overlap-add.
    pub windows: Windows,
    /// Lapping state.
    pub lapping_state: Option<LappingState>,
}

impl Dsp {
    pub fn reset(&mut self) {
        for channel in &mut self.channels {
            channel.reset();
        }

        self.lapping_state = None;
    }
}

pub struct DspChannel {
    /// The channel floor buffer.
    pub floor: Vec<f32>,
    /// The channel residue buffer.
    pub residue: Vec<f32>,
    /// Do not decode!
    pub do_not_decode: bool,
    /// The output buffer for the IMDCT.
    imdct: Vec<f32>,
    /// Samples saved from the last IMDCT for overlap-add.
    overlap: Vec<f32>,
    /// Short block size.
    bs0: usize,
    /// Long block size.
    bs1: usize,
}

impl DspChannel {
    pub fn new(bs0_exp: u8, bs1_exp: u8) -> Self {
        let bs0 = 1 << bs0_exp;
        let bs1 = 1 << bs1_exp;

        DspChannel {
            floor: vec![0.0; bs1 >> 1],
            residue: vec![0.0; bs1 >> 1],
            imdct: vec![0.0; bs1],
            overlap: vec![0.0; bs1 >> 1],
            do_not_decode: false,
            bs0,
            bs1,
        }
    }

    pub fn synth(
        &mut self,
        block_flag: bool,
        lap_state: &Option<LappingState>,
        windows: &Windows,
        imdct: &mut Imdct,
        buf: &mut [f32],
    ) {
        // Block size of the current block.
        let bs = if block_flag { self.bs1 } else { self.bs0 };

        // Perform the inverse MDCT on the audio spectrum.
        imdct.imdct(&self.floor[..bs / 2], &mut self.imdct[..bs]);

        // Overlap-add and windowing with the previous buffer.
        if let Some(lap_state) = &lap_state {
            // Window for this block.
            let win = if block_flag && lap_state.prev_block_flag {
                &windows.long
            }
            else {
                &windows.short
            };

            if lap_state.prev_block_flag == block_flag {
                // Both the previous and current blocks are either short or long. In this case,
                // there is a complete overlap between.
                overlap_add(buf, &self.overlap[..bs / 2], &self.imdct[..bs / 2], win);
            }
            else if lap_state.prev_block_flag && !block_flag {
                // The previous block is long and the current block is short.
                let start = (self.bs1 - self.bs0) / 4;
                let end = start + self.bs0 / 2;

                // Unity samples (no overlap).
                buf[..start].copy_from_slice(&self.overlap[..start]);

                // Overlapping samples.
                overlap_add(
                    &mut buf[start..],
                    &self.overlap[start..end],
                    &self.imdct[..self.bs0 / 2],
                    win,
                );
            }
            else {
                // The previous block is short and the current block is long.
                let start = (self.bs1 - self.bs0) / 4;
                let end = start + self.bs0 / 2;

                // Overlapping samples.
                overlap_add(
                    &mut buf[..self.bs0 / 2],
                    &self.overlap[..self.bs0 / 2],
                    &self.imdct[start..end],
                    win,
                );

                // Unity samples (no overlap).
                buf[self.bs0 / 2..].copy_from_slice(&self.imdct[end..self.bs1 / 2]);
            }

            // Clamp the output samples.
            for s in buf.iter_mut() {
                *s = s.clamp(-1.0, 1.0);
            }
        }

        // Save right-half of IMDCT buffer for later.
        self.overlap[..bs / 2].copy_from_slice(&self.imdct[bs / 2..bs]);
    }

    pub fn reset(&mut self) {
        // Clear the overlap buffer. Nothing else is used across packets.
        self.overlap.fill(0.0);
    }
}

#[inline(always)]
fn overlap_add(out: &mut [f32], left: &[f32], right: &[f32], win: &[f32]) {
    assert!(left.len() == right.len());
    assert!(left.len() == win.len());
    assert!(left.len() == out.len());

    let iter = left.iter().zip(right).zip(win.iter().rev()).zip(win).zip(out);

    for ((((&s0, &s1), &w0), &w1), out) in iter {
        *out = s0 * w0 + s1 * w1;
    }
}
