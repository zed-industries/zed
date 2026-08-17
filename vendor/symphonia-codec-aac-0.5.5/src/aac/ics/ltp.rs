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

use symphonia_core::errors::{unsupported_error, Result};
use symphonia_core::io::ReadBitsLtr;

#[derive(Clone, Copy)]
pub struct LtpData {}

impl LtpData {
    pub fn read<B: ReadBitsLtr>(bs: &mut B) -> Result<Option<Self>> {
        let predictor_data_present = bs.read_bool()?;

        if !predictor_data_present {
            return Ok(None);
        }

        /*
        if is_main {
            let predictor_reset                         = bs.read_bit()?;
            if predictor_reset {
                let predictor_reset_group_number        = bs.read_bits_leq32(5)?;
            }
            for sfb in 0..max_sfb.min(PRED_SFB_MAX) {
                prediction_used[sfb]                    = bs.read_bit()?;
            }
        }
        else {
            let ltp_data_present                        = bs.read_bit()?;
            if ltp_data_present {
                //ltp data
            }
            if common_window {
                let ltp_data_present                    = bs.read_bit()?;
                if ltp_data_present {
                    //ltp data
                }
            }
        }
        Ok(Some(Self { }))
        */

        unsupported_error("aac: predictor data")
    }
}
