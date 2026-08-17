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

use std::f32::consts;

use crate::aac::common::*;
use crate::aac::ics::IcsInfo;

const TNS_MAX_ORDER: usize = 20;
const TNS_MAX_LONG_BANDS: [usize; 12] = [31, 31, 34, 40, 42, 51, 46, 46, 42, 42, 42, 39];
const TNS_MAX_SHORT_BANDS: [usize; 12] = [9, 9, 10, 14, 14, 14, 14, 14, 14, 14, 14, 14];

#[derive(Copy, Clone)]
struct TnsCoeffs {
    length: usize,
    order: usize,
    direction: bool,
    coef: [f32; TNS_MAX_ORDER + 1],
}

impl TnsCoeffs {
    fn new() -> Self {
        Self { length: 0, order: 0, direction: false, coef: [0.0; TNS_MAX_ORDER + 1] }
    }

    fn read<B: ReadBitsLtr>(
        &mut self,
        bs: &mut B,
        long_win: bool,
        coef_res: bool,
        max_order: usize,
    ) -> Result<()> {
        self.length = bs.read_bits_leq32(if long_win { 6 } else { 4 })? as usize;
        self.order = bs.read_bits_leq32(if long_win { 5 } else { 3 })? as usize;

        validate!(self.order <= max_order);

        if self.order > 0 {
            self.direction = bs.read_bool()?;

            let coef_compress = bs.read_bool()?;

            // If coef_res is true, then the transmitted resolution of the filter coefficients
            // is 4 bits, otherwise it's 3 (4.6.9.2).
            let mut coef_res_bits = if coef_res { 4 } else { 3 };

            // If true, the most significant bit of the filter coefficient is not transmitted
            // (4.6.9.2).
            if coef_compress {
                coef_res_bits -= 1;
            }

            let sign_mask = 1 << (coef_res_bits - 1);
            let neg_mask = !((1 << coef_res_bits) - 1);

            // Derived from `1 << (coef_res_bits - 1)` before compression.
            let fac_base = if coef_res { 8.0 } else { 4.0 };

            let iqfac = (fac_base - 0.5) / consts::FRAC_PI_2;
            let iqfac_m = (fac_base + 0.5) / consts::FRAC_PI_2;

            let mut tmp: [f32; TNS_MAX_ORDER] = [0.0; TNS_MAX_ORDER];

            for el in tmp[..self.order].iter_mut() {
                let val = bs.read_bits_leq32(coef_res_bits)? as u8;

                // Convert to signed integer.
                let c = f32::from(if (val & sign_mask) != 0 {
                    (val | neg_mask) as i8
                }
                else {
                    val as i8
                });

                *el = (if c >= 0.0 { c / iqfac } else { c / iqfac_m }).sin();
            }

            // Generate LPC coefficients
            let mut b: [f32; TNS_MAX_ORDER + 1] = [0.0; TNS_MAX_ORDER + 1];

            for m in 1..=self.order {
                for i in 1..m {
                    b[i] = self.coef[i - 1] + tmp[m - 1] * self.coef[m - i - 1];
                }

                self.coef[..(m - 1)].copy_from_slice(&b[1..m]);
                self.coef[m - 1] = tmp[m - 1];
            }
        }

        Ok(())
    }
}

#[derive(Copy, Clone)]
pub struct Tns {
    n_filt: [usize; MAX_WINDOWS],
    coeffs: [[TnsCoeffs; 4]; MAX_WINDOWS],
}

impl Tns {
    pub fn read<B: ReadBitsLtr>(bs: &mut B, info: &IcsInfo, is_lc: bool) -> Result<Option<Self>> {
        let tns_data_present = bs.read_bool()?;

        if !tns_data_present {
            return Ok(None);
        }

        // Table 4.156
        let max_order = if !info.long_win {
            7
        }
        else if is_lc {
            12
        }
        else {
            TNS_MAX_ORDER
        };

        let mut n_filt: [usize; MAX_WINDOWS] = [0; MAX_WINDOWS];
        let mut coeffs: [[TnsCoeffs; 4]; MAX_WINDOWS] = [[TnsCoeffs::new(); 4]; MAX_WINDOWS];

        for w in 0..info.num_windows {
            n_filt[w] = bs.read_bits_leq32(if info.long_win { 2 } else { 1 })? as usize;

            let coef_res = if n_filt[w] != 0 { bs.read_bool()? } else { false };

            for filt in 0..n_filt[w] {
                coeffs[w][filt].read(bs, info.long_win, coef_res, max_order)?;
            }
        }

        Ok(Some(Self { n_filt, coeffs }))
    }

    pub fn synth(
        &self,
        info: &IcsInfo,
        bands: &[usize],
        rate_idx: usize,
        coeffs: &mut [f32; 1024],
    ) {
        let tns_max_bands = (if info.long_win {
            TNS_MAX_LONG_BANDS[rate_idx]
        }
        else {
            TNS_MAX_SHORT_BANDS[rate_idx]
        })
        .min(info.max_sfb);

        for w in 0..info.num_windows {
            let mut bottom = bands.len() - 1;

            for f in 0..self.n_filt[w] {
                let top = bottom;

                bottom = top.saturating_sub(self.coeffs[w][f].length);

                let order = self.coeffs[w][f].order;

                if order == 0 {
                    continue;
                }

                let start = w * 128 + bands[bottom.min(tns_max_bands)];
                let end = w * 128 + bands[top.min(tns_max_bands)];

                let lpc = &self.coeffs[w][f].coef;

                if !self.coeffs[w][f].direction {
                    for (m, i) in (start..end).enumerate() {
                        for j in 0..order.min(m) {
                            coeffs[i] -= coeffs[i - j - 1] * lpc[j];
                        }
                    }
                }
                else {
                    for (m, i) in (start..end).rev().enumerate() {
                        for j in 0..order.min(m) {
                            coeffs[i] -= coeffs[i + j + 1] * lpc[j];
                        }
                    }
                }
            }
        }
    }
}
