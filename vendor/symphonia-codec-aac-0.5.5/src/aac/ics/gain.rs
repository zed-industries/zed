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
#[allow(dead_code)]
pub struct GainControl {
    max_band: u8,
}

impl GainControl {
    pub fn read<B: ReadBitsLtr>(bs: &mut B) -> Result<Option<Self>> {
        let gain_control_data_present = bs.read_bool()?;

        if !gain_control_data_present {
            return Ok(None);
        }

        /*
        self.max_band = bs.read_bits_leq32(2)? as u8;
        if window_sequence == ONLY_LONG_SEQUENCE {
            for bd in 0..max_band
            ...
        }
        Ok(Some(Self { }))
        */

        unsupported_error("aac: gain control data")
    }
}
