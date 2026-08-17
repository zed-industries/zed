// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use symphonia_core::audio::Channels;

/// Get the mapping 0 channel listing for the given number of channels.
pub fn vorbis_channels_to_channels(num_channels: u8) -> Option<Channels> {
    let channels = match num_channels {
        1 => Channels::FRONT_LEFT,
        2 => Channels::FRONT_LEFT | Channels::FRONT_RIGHT,
        3 => Channels::FRONT_LEFT | Channels::FRONT_CENTRE | Channels::FRONT_RIGHT,
        4 => {
            Channels::FRONT_LEFT
                | Channels::FRONT_RIGHT
                | Channels::REAR_LEFT
                | Channels::REAR_RIGHT
        }
        5 => {
            Channels::FRONT_LEFT
                | Channels::FRONT_CENTRE
                | Channels::FRONT_RIGHT
                | Channels::REAR_LEFT
                | Channels::REAR_RIGHT
        }
        6 => {
            Channels::FRONT_LEFT
                | Channels::FRONT_CENTRE
                | Channels::FRONT_RIGHT
                | Channels::REAR_LEFT
                | Channels::REAR_RIGHT
                | Channels::LFE1
        }
        7 => {
            Channels::FRONT_LEFT
                | Channels::FRONT_CENTRE
                | Channels::FRONT_RIGHT
                | Channels::SIDE_LEFT
                | Channels::SIDE_RIGHT
                | Channels::REAR_CENTRE
                | Channels::LFE1
        }
        8 => {
            Channels::FRONT_LEFT
                | Channels::FRONT_CENTRE
                | Channels::FRONT_RIGHT
                | Channels::SIDE_LEFT
                | Channels::SIDE_RIGHT
                | Channels::REAR_LEFT
                | Channels::REAR_RIGHT
                | Channels::LFE1
        }
        _ => return None,
    };

    Some(channels)
}
