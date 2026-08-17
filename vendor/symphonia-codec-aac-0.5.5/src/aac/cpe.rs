// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// Previous Author: Kostya Shishkov <kostya.shiskov@gmail.com>
//
// This source file includes code originally written for the NihAV
// project. With the author's permission, it has been relicensed for,
// and ported to the Symphonia project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use symphonia_core::audio::{AudioBuffer, Signal};
use symphonia_core::errors::{decode_error, Result};
use symphonia_core::io::ReadBitsLtr;

use crate::aac::common::*;
use crate::aac::dsp;
use crate::aac::ics;
use crate::common::M4AType;

#[derive(Clone)]
pub struct ChannelPair {
    pub is_pair: bool,
    pub channel: usize,
    ms_mask_present: u8,
    ms_used: [[bool; MAX_SFBS]; MAX_WINDOWS],
    ics0: ics::Ics,
    ics1: ics::Ics,
    lcg: Lcg,
}

impl ChannelPair {
    pub fn new(is_pair: bool, channel: usize, sbinfo: GASubbandInfo) -> Self {
        Self {
            is_pair,
            channel,
            ms_mask_present: 0,
            ms_used: [[false; MAX_SFBS]; MAX_WINDOWS],
            ics0: ics::Ics::new(sbinfo),
            ics1: ics::Ics::new(sbinfo),
            lcg: Lcg::new(0x1f2e3d4c), // Use the same seed as ffmpeg for symphonia-check.
        }
    }

    pub fn reset(&mut self) {
        self.ics0.reset();
        self.ics1.reset();
    }

    pub fn decode_ga_sce<B: ReadBitsLtr>(&mut self, bs: &mut B, m4atype: M4AType) -> Result<()> {
        self.ics0.decode(bs, &mut self.lcg, m4atype, false)?;
        Ok(())
    }

    pub fn decode_ga_cpe<B: ReadBitsLtr>(&mut self, bs: &mut B, m4atype: M4AType) -> Result<()> {
        let common_window = bs.read_bool()?;

        if common_window {
            // Decode the common ICS info block into the first channel.
            self.ics0.info.decode(bs)?;

            // Mid-side stereo mask decoding.
            self.ms_mask_present = bs.read_bits_leq32(2)? as u8;

            match self.ms_mask_present {
                0 | 2 => {
                    // If mid-side mask present is 0, then mid-side coding is never used. If the
                    // value is 2, then mid-side coding is always used.
                    let is_used = self.ms_mask_present == 2;

                    for g in 0..self.ics0.info.window_groups {
                        for sfb in 0..self.ics0.info.max_sfb {
                            self.ms_used[g][sfb] = is_used;
                        }
                    }
                }
                1 => {
                    // If mid-side mask present is 1, then read a bit for each band indicating if
                    // the band uses mid-side coding.
                    for g in 0..self.ics0.info.window_groups {
                        for sfb in 0..self.ics0.info.max_sfb {
                            self.ms_used[g][sfb] = bs.read_bool()?;
                        }
                    }
                }
                3 => return decode_error("aac: invalid mid-side mask"),
                _ => unreachable!(),
            }

            // Copy the common ICS info decoded in the first channel to the second channel.
            self.ics1.info.copy_from_common(&self.ics0.info);
        }

        self.ics0.decode(bs, &mut self.lcg, m4atype, common_window)?;
        self.ics1.decode(bs, &mut self.lcg, m4atype, common_window)?;

        // Joint-stereo decoding
        if common_window {
            let bands = self.ics0.get_bands();

            let mut g = 0;

            for w in 0..self.ics0.info.num_windows {
                if w > 0 && !self.ics0.info.scale_factor_grouping[w - 1] {
                    g += 1;
                }

                for sfb in 0..self.ics0.info.max_sfb {
                    let start = w * 128 + bands[sfb];
                    let end = w * 128 + bands[sfb + 1];

                    if self.ics1.is_intensity(g, sfb) {
                        // Intensity stereo
                        // Section 4.6.8.2.3
                        let invert = self.ms_mask_present == 1 && self.ms_used[g][sfb];
                        let dir = if self.ics1.get_intensity_dir(g, sfb) { 1.0 } else { -1.0 };
                        let factor = if invert { -1.0 } else { 1.0 };

                        let scale = dir * factor * self.ics1.scales[g][sfb];

                        let left = &self.ics0.coeffs[start..end];
                        let right = &mut self.ics1.coeffs[start..end];

                        for (l, r) in left.iter().zip(right) {
                            *r = scale * l;
                        }
                    }
                    else if self.ics0.is_noise(g, sfb) || self.ics1.is_noise(g, sfb) {
                        // Perceptual noise substitution, do not do joint-stereo decoding.
                        // Section 4.6.13.3
                    }
                    else if self.ms_used[g][sfb] {
                        // Mid-side stereo.
                        let mid = &mut self.ics0.coeffs[start..end];
                        let side = &mut self.ics1.coeffs[start..end];

                        for (m, s) in mid.iter_mut().zip(side) {
                            let tmp = *m - *s;
                            *m += *s;
                            *s = tmp;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub fn synth_audio(
        &mut self,
        dsp: &mut dsp::Dsp,
        abuf: &mut AudioBuffer<f32>,
        rate_idx: usize,
    ) {
        self.ics0.synth_channel(dsp, rate_idx, abuf.chan_mut(self.channel));

        if self.is_pair {
            self.ics1.synth_channel(dsp, rate_idx, abuf.chan_mut(self.channel + 1));
        }
    }
}
