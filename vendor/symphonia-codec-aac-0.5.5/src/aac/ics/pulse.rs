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

use symphonia_core::errors::Result;
use symphonia_core::io::ReadBitsLtr;

use crate::aac::common::{MAX_SFBS, MAX_WINDOWS};

#[inline(always)]
fn iquant(val: f32) -> f32 {
    if val < 0.0 {
        -((-val).powf(4.0 / 3.0))
    }
    else {
        val.powf(4.0 / 3.0)
    }
}

#[inline(always)]
fn requant(val: f32, scale: f32) -> f32 {
    if scale == 0.0 {
        return 0.0;
    }
    let bval = val / scale;
    if bval >= 0.0 {
        val.powf(3.0 / 4.0)
    }
    else {
        -((-val).powf(3.0 / 4.0))
    }
}

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub struct Pulse {
    number_pulse: usize,
    pulse_start_sfb: usize,
    pulse_offset: [u8; 4],
    pulse_amp: [u8; 4],
}

impl Pulse {
    pub fn read<B: ReadBitsLtr>(bs: &mut B) -> Result<Option<Self>> {
        let pulse_data_present = bs.read_bool()?;

        if !pulse_data_present {
            return Ok(None);
        }

        let number_pulse = bs.read_bits_leq32(2)? as usize + 1;
        let pulse_start_sfb = bs.read_bits_leq32(6)? as usize;

        let mut pulse_offset: [u8; 4] = [0; 4];
        let mut pulse_amp: [u8; 4] = [0; 4];

        for i in 0..number_pulse {
            pulse_offset[i] = bs.read_bits_leq32(5)? as u8;
            pulse_amp[i] = bs.read_bits_leq32(4)? as u8;
        }

        Ok(Some(Self { number_pulse, pulse_start_sfb, pulse_offset, pulse_amp }))
    }

    pub fn synth(
        &self,
        bands: &[usize],
        scales: &[[f32; MAX_SFBS]; MAX_WINDOWS],
        coeffs: &mut [f32; 1024],
    ) {
        if self.pulse_start_sfb >= bands.len() - 1 {
            return;
        }

        let mut k = bands[self.pulse_start_sfb];

        let mut band = self.pulse_start_sfb;

        for pno in 0..self.number_pulse {
            k += self.pulse_offset[pno] as usize;

            if k >= 1024 {
                return;
            }

            while bands[band + 1] <= k {
                band += 1;
            }

            let scale = scales[0][band];

            let mut base = coeffs[k];

            if base != 0.0 {
                base = requant(coeffs[k], scale);
            }

            if base > 0.0 {
                base += f32::from(self.pulse_amp[pno]);
            }
            else {
                base -= f32::from(self.pulse_amp[pno]);
            }
            coeffs[k] = iquant(base) * scale;
        }
    }
}
