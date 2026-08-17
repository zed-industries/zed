// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use symphonia_core::errors::{decode_error, Result};
use symphonia_core::io::ReadBytes;
use symphonia_core::util::bits;

use crate::atoms::{Atom, AtomHeader};

/// Track fragment run atom.
#[allow(dead_code)]
#[derive(Debug)]
pub struct TrunAtom {
    /// Atom header.
    header: AtomHeader,
    /// Extended header flags.
    flags: u32,
    /// Data offset of this run.
    pub data_offset: Option<i32>,
    /// Number of samples in this run.
    pub sample_count: u32,
    /// Sample flags for the first sample only.
    pub first_sample_flags: Option<u32>,
    /// Sample duration for each sample in this run.
    pub sample_duration: Vec<u32>,
    /// Sample size for each sample in this run.
    pub sample_size: Vec<u32>,
    /// Sample flags for each sample in this run.
    pub sample_flags: Vec<u32>,
    /// The total size of all samples in this run. 0 if the sample size flag is not set.
    total_sample_size: u64,
    /// The total duration of all samples in this run. 0 if the sample duration flag is not set.
    total_sample_duration: u64,
}

impl TrunAtom {
    // Track fragment run atom flags.
    const DATA_OFFSET_PRESENT: u32 = 0x1;
    const FIRST_SAMPLE_FLAGS_PRESENT: u32 = 0x4;
    const SAMPLE_DURATION_PRESENT: u32 = 0x100;
    const SAMPLE_SIZE_PRESENT: u32 = 0x200;
    const SAMPLE_FLAGS_PRESENT: u32 = 0x400;
    const SAMPLE_COMPOSITION_TIME_OFFSETS_PRESENT: u32 = 0x800;

    /// Indicates if sample durations are provided.
    pub fn is_sample_duration_present(&self) -> bool {
        self.flags & TrunAtom::SAMPLE_DURATION_PRESENT != 0
    }

    // Indicates if the duration of the first sample is provided.
    pub fn is_first_sample_duration_present(&self) -> bool {
        match self.first_sample_flags {
            Some(flags) => flags & TrunAtom::FIRST_SAMPLE_FLAGS_PRESENT != 0,
            None => false,
        }
    }

    /// Indicates if sample sizes are provided.
    pub fn is_sample_size_present(&self) -> bool {
        self.flags & TrunAtom::SAMPLE_SIZE_PRESENT != 0
    }

    /// Indicates if the size for the first sample is provided.
    pub fn is_first_sample_size_present(&self) -> bool {
        match self.first_sample_flags {
            Some(flags) => flags & TrunAtom::SAMPLE_SIZE_PRESENT != 0,
            None => false,
        }
    }

    /// Indicates if sample flags are provided.
    #[allow(dead_code)]
    pub fn are_sample_flags_present(&self) -> bool {
        self.flags & TrunAtom::SAMPLE_FLAGS_PRESENT != 0
    }

    /// Indicates if sample composition time offsets are provided.
    #[allow(dead_code)]
    pub fn are_sample_composition_time_offsets_present(&self) -> bool {
        self.flags & TrunAtom::SAMPLE_COMPOSITION_TIME_OFFSETS_PRESENT != 0
    }

    /// Gets the total duration of all samples.
    pub fn total_duration(&self, default_dur: u32) -> u64 {
        if self.is_sample_duration_present() {
            self.total_sample_duration
        }
        else {
            // The duration of all samples in the track fragment are not explictly known.
            if self.sample_count > 0 && self.is_first_sample_duration_present() {
                // The first sample has an explictly recorded duration.
                u64::from(self.sample_duration[0])
                    + u64::from(self.sample_count - 1) * u64::from(default_dur)
            }
            else {
                // All samples have the default duration.
                u64::from(self.sample_count) * u64::from(default_dur)
            }
        }
    }

    /// Gets the total size of all samples.
    pub fn total_size(&self, default_size: u32) -> u64 {
        if self.is_sample_size_present() {
            self.total_sample_size
        }
        else if self.sample_count > 0 && self.is_first_sample_size_present() {
            u64::from(self.sample_size[0])
                + u64::from(self.sample_count - 1) * u64::from(default_size)
        }
        else {
            u64::from(self.sample_count) * u64::from(default_size)
        }
    }

    /// Get the timestamp and duration of a sample. The desired sample is specified by the
    /// trun-relative sample number, `sample_num_rel`.
    pub fn sample_timing(&self, sample_num_rel: u32, default_dur: u32) -> (u64, u32) {
        debug_assert!(sample_num_rel < self.sample_count);

        if self.is_sample_duration_present() {
            // All sample durations are unique.
            let ts = if sample_num_rel > 0 {
                self.sample_duration[..sample_num_rel as usize]
                    .iter()
                    .map(|&s| u64::from(s))
                    .sum::<u64>()
            }
            else {
                0
            };

            let dur = self.sample_duration[sample_num_rel as usize];

            (ts, dur)
        }
        else {
            // The duration of all samples in the track fragment are not unique.
            let ts = if sample_num_rel > 0 && self.is_first_sample_duration_present() {
                // The first sample has a unique duration.
                u64::from(self.sample_duration[0])
                    + u64::from(sample_num_rel - 1) * u64::from(default_dur)
            }
            else {
                // Zero or more samples with identical durations.
                u64::from(sample_num_rel) * u64::from(default_dur)
            };

            (ts, default_dur)
        }
    }

    /// Get the size of a sample. The desired sample is specified by the trun-relative sample
    /// number, `sample_num_rel`.
    pub fn sample_size(&self, sample_num_rel: u32, default_size: u32) -> u32 {
        debug_assert!(sample_num_rel < self.sample_count);

        if self.is_sample_size_present() {
            self.sample_size[sample_num_rel as usize]
        }
        else if sample_num_rel == 0 && self.is_first_sample_size_present() {
            self.sample_size[0]
        }
        else {
            default_size
        }
    }

    /// Get the byte offset and size of a sample. The desired sample is specified by the
    /// trun-relative sample number, `sample_num_rel`.
    pub fn sample_offset(&self, sample_num_rel: u32, default_size: u32) -> (u64, u32) {
        debug_assert!(sample_num_rel < self.sample_count);

        if self.is_sample_size_present() {
            // All sample sizes are unique.
            let offset = if sample_num_rel > 0 {
                self.sample_size[..sample_num_rel as usize]
                    .iter()
                    .map(|&s| u64::from(s))
                    .sum::<u64>()
            }
            else {
                0
            };

            (offset, self.sample_size[sample_num_rel as usize])
        }
        else {
            // The size of all samples in the track are not unique.
            let offset = if sample_num_rel > 0 && self.is_first_sample_size_present() {
                // The first sample has a unique size.
                u64::from(self.sample_size[0])
                    + u64::from(sample_num_rel - 1) * u64::from(default_size)
            }
            else {
                // Zero or more identically sized samples.
                u64::from(sample_num_rel) * u64::from(default_size)
            };

            (offset, default_size)
        }
    }

    /// Get the sample number (relative to the trun) of the sample that contains timestamp `ts`.
    pub fn ts_sample(&self, ts_rel: u64, default_dur: u32) -> u32 {
        let mut sample_num = 0;
        let mut ts_delta = ts_rel;

        if self.is_sample_duration_present() {
            // If the sample durations are present, then each sample duration is independently
            // stored. Sum sample durations until the delta is reached.
            for &dur in &self.sample_duration {
                if u64::from(dur) > ts_delta {
                    break;
                }

                ts_delta -= u64::from(dur);
                sample_num += 1;
            }
        }
        else {
            if self.sample_count > 0 && self.is_first_sample_duration_present() {
                // The first sample duration is unique.
                let first_sample_dur = u64::from(self.sample_duration[0]);

                if ts_delta >= first_sample_dur {
                    ts_delta -= first_sample_dur;
                    sample_num += 1;
                }
                else {
                    ts_delta -= ts_delta;
                }
            }

            sample_num += ts_delta.checked_div(u64::from(default_dur)).unwrap_or(0) as u32;
        }

        sample_num
    }
}

impl Atom for TrunAtom {
    fn header(&self) -> AtomHeader {
        self.header
    }

    fn read<B: ReadBytes>(reader: &mut B, header: AtomHeader) -> Result<Self> {
        let (_, flags) = AtomHeader::read_extra(reader)?;

        let sample_count = reader.read_be_u32()?;

        let data_offset = match flags & TrunAtom::DATA_OFFSET_PRESENT {
            0 => None,
            _ => Some(bits::sign_extend_leq32_to_i32(reader.read_be_u32()?, 32)),
        };

        let first_sample_flags = match flags & TrunAtom::FIRST_SAMPLE_FLAGS_PRESENT {
            0 => None,
            _ => Some(reader.read_be_u32()?),
        };

        // If the first-sample-flags-present flag is set, then the sample-flags-present flag should
        // not be set. The samples after the first shall use the default sample flags defined in the
        // tfhd or mvex atoms.
        if first_sample_flags.is_some() && (flags & TrunAtom::SAMPLE_FLAGS_PRESENT != 0) {
            return decode_error(
                "isomp4: sample-flag-present and first-sample-flags-present flags are set",
            );
        }

        let mut sample_duration = Vec::new();
        let mut sample_size = Vec::new();
        let mut sample_flags = Vec::new();

        let mut total_sample_size = 0;
        let mut total_sample_duration = 0;

        // TODO: Apply a limit.
        for _ in 0..sample_count {
            if (flags & TrunAtom::SAMPLE_DURATION_PRESENT) != 0 {
                let duration = reader.read_be_u32()?;
                total_sample_duration += u64::from(duration);
                sample_duration.push(duration);
            }

            if (flags & TrunAtom::SAMPLE_SIZE_PRESENT) != 0 {
                let size = reader.read_be_u32()?;
                total_sample_size += u64::from(size);
                sample_size.push(size);
            }

            if (flags & TrunAtom::SAMPLE_FLAGS_PRESENT) != 0 {
                sample_flags.push(reader.read_be_u32()?);
            }

            // Ignoring composition time for now since it's a video thing...
            if (flags & TrunAtom::SAMPLE_COMPOSITION_TIME_OFFSETS_PRESENT) != 0 {
                // For version 0, this is a u32.
                // For version 1, this is a i32.
                let _ = reader.read_be_u32()?;
            }
        }

        Ok(TrunAtom {
            header,
            flags,
            data_offset,
            sample_count,
            first_sample_flags,
            sample_duration,
            sample_size,
            sample_flags,
            total_sample_size,
            total_sample_duration,
        })
    }
}
