// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use symphonia_core::errors::{decode_error, Error, Result};

use crate::atoms::{stsz::SampleSize, Co64Atom, MoofAtom, MoovAtom, MvexAtom, StcoAtom, TrafAtom};

use std::ops::Range;
use std::sync::Arc;

/// Sample data information.
pub struct SampleDataDesc {
    /// The starting byte position within the media data of the group of samples that contains the
    /// sample described.
    pub base_pos: u64,
    /// The offset relative to the base position of the sample described.
    pub offset: Option<u64>,
    /// The size of the sample.
    pub size: u32,
}

/// Timing information for one sample.
pub struct SampleTiming {
    /// The timestamp of the sample.
    pub ts: u64,
    /// The duration of the sample.
    pub dur: u32,
}

pub trait StreamSegment: Send + Sync {
    /// Gets the sequence number of this segment.
    fn sequence_num(&self) -> u32;

    /// Gets the first and last sample numbers for the track `track_num`.
    fn track_sample_range(&self, track_num: usize) -> Range<u32>;

    /// Gets the first and last sample timestamps for the track `track_num`.
    fn track_ts_range(&self, track_num: usize) -> Range<u64>;

    /// Get the timestamp and duration for the sample indicated by `sample_num` for the track
    /// `track_num`.
    fn sample_timing(&self, track_num: usize, sample_num: u32) -> Result<Option<SampleTiming>>;

    /// Get the sample number of the sample containing the timestamp indicated by `ts` for track
    // `track_num`.
    fn ts_sample(&self, track_num: usize, ts: u64) -> Result<Option<u32>>;

    /// Get the byte position of the group of samples containing the sample indicated by
    /// `sample_num` for track `track_num`, and it's size.
    ///
    /// Optionally, the offset of the sample relative to the aforementioned byte position can be
    /// returned.
    fn sample_data(
        &self,
        track_num: usize,
        sample_num: u32,
        get_offset: bool,
    ) -> Result<SampleDataDesc>;
}

/// Track-to-stream sequencing information.
#[derive(Copy, Clone, Debug, Default)]
struct SequenceInfo {
    /// The sample number of the first sample of a track in a fragment.
    first_sample: u32,
    /// The timestamp of the first sample of a track in a fragment.
    first_ts: u64,
    /// The total duration of all samples of a track in a fragment.
    total_sample_duration: u64,
    /// The total sample count of a track in a fragment.
    total_sample_count: u32,
    /// If present in the moof segment, this is the index of the track fragment atom for the track
    /// this sequence information is associated with.
    traf_idx: Option<usize>,
}

pub struct MoofSegment {
    moof: MoofAtom,
    mvex: Arc<MvexAtom>,
    seq: Vec<SequenceInfo>,
}

impl MoofSegment {
    /// Instantiate a new segment from a `MoofAtom`.
    pub fn new(moof: MoofAtom, mvex: Arc<MvexAtom>, prev: &dyn StreamSegment) -> MoofSegment {
        let mut seq = Vec::with_capacity(mvex.trexs.len());

        // Calculate the sequence information for each track, even if not present in the fragment.
        for (track_num, trex) in mvex.trexs.iter().enumerate() {
            let mut info = SequenceInfo {
                first_sample: prev.track_sample_range(track_num).end,
                first_ts: prev.track_ts_range(track_num).end,
                ..Default::default()
            };

            // Find the track fragment for the track.
            for (traf_idx, traf) in moof.trafs.iter().enumerate() {
                if trex.track_id != traf.tfhd.track_id {
                    continue;
                }

                // Calculate the total duration of all runs in the fragment for the track.
                let default_dur =
                    traf.tfhd.default_sample_duration.unwrap_or(trex.default_sample_duration);

                for trun in traf.truns.iter() {
                    info.total_sample_duration += trun.total_duration(default_dur);
                }

                info.total_sample_count = traf.total_sample_count;
                info.traf_idx = Some(traf_idx);
            }

            seq.push(info);
        }

        MoofSegment { moof, mvex, seq }
    }

    /// Try to get the Track Fragment atom associated with the track identified by `track_num`.
    fn try_get_traf(&self, track_num: usize) -> Option<&TrafAtom> {
        debug_assert!(track_num < self.seq.len());
        self.seq[track_num].traf_idx.map(|idx| &self.moof.trafs[idx])
    }
}

impl StreamSegment for MoofSegment {
    fn sequence_num(&self) -> u32 {
        self.moof.mfhd.sequence_number
    }

    fn sample_timing(&self, track_num: usize, sample_num: u32) -> Result<Option<SampleTiming>> {
        // Get the track fragment associated with track_num.
        let traf = match self.try_get_traf(track_num) {
            Some(traf) => traf,
            None => return Ok(None),
        };

        let mut sample_num_rel = sample_num - self.seq[track_num].first_sample;
        let mut trun_ts_offset = self.seq[track_num].first_ts;

        let default_dur = traf
            .tfhd
            .default_sample_duration
            .unwrap_or(self.mvex.trexs[track_num].default_sample_duration);

        for trun in traf.truns.iter() {
            // If the sample is contained within the this track run, get the timing of of the
            // sample.
            if sample_num_rel < trun.sample_count {
                let (ts, dur) = trun.sample_timing(sample_num_rel, default_dur);
                return Ok(Some(SampleTiming { ts: trun_ts_offset + ts, dur }));
            }

            let trun_dur = trun.total_duration(default_dur);

            sample_num_rel -= trun.sample_count;
            trun_ts_offset += trun_dur;
        }

        Ok(None)
    }

    fn ts_sample(&self, track_num: usize, ts: u64) -> Result<Option<u32>> {
        // Get the track fragment associated with track_num.
        let traf = match self.try_get_traf(track_num) {
            Some(traf) => traf,
            None => return Ok(None),
        };

        let mut sample_num = self.seq[track_num].first_sample;
        let mut ts_accum = self.seq[track_num].first_ts;

        let default_dur = traf
            .tfhd
            .default_sample_duration
            .unwrap_or(self.mvex.trexs[track_num].default_sample_duration);

        for trun in traf.truns.iter() {
            // Get the total duration of this track run.
            let trun_dur = trun.total_duration(default_dur);

            // If the timestamp after the track run is greater than the desired timestamp, then the
            // desired sample must be in this run of samples.
            if ts_accum + trun_dur > ts {
                sample_num += trun.ts_sample(ts - ts_accum, default_dur);
                return Ok(Some(sample_num));
            }

            sample_num += trun.sample_count;
            ts_accum += trun_dur;
        }

        Ok(None)
    }

    fn sample_data(
        &self,
        track_num: usize,
        sample_num: u32,
        get_offset: bool,
    ) -> Result<SampleDataDesc> {
        // Get the track fragment associated with track_num.
        let traf = self.try_get_traf(track_num).unwrap();

        // If an explicit anchor-point is set, then use that for the position, otherwise use the
        // first-byte of the enclosing moof atom.
        let traf_base_pos = match traf.tfhd.base_data_offset {
            Some(pos) => pos,
            _ => self.moof.moof_base_pos,
        };

        let mut sample_num_rel = sample_num - self.seq[track_num].first_sample;
        let mut trun_offset = traf_base_pos;

        let default_size =
            traf.tfhd.default_sample_size.unwrap_or(self.mvex.trexs[track_num].default_sample_size);

        for trun in traf.truns.iter() {
            // If a data offset is present for this track fragment run, then calculate the new base
            // position for the run. When a data offset is not present, do nothing because this run
            // follows the previous run.
            if let Some(offset) = trun.data_offset {
                // The offset for the run is relative to the anchor-point defined in the track
                // fragment header.
                trun_offset = if offset.is_negative() {
                    traf_base_pos - u64::from(offset.wrapping_abs() as u32)
                }
                else {
                    traf_base_pos + offset as u64
                };
            }

            if sample_num_rel < trun.sample_count {
                let (offset, size) = if get_offset {
                    // Get the size and offset of the sample.
                    let (offset, size) = trun.sample_offset(sample_num_rel, default_size);
                    (Some(offset), size)
                }
                else {
                    // Just get the size of the sample.
                    let size = trun.sample_size(sample_num_rel, default_size);
                    (None, size)
                };

                return Ok(SampleDataDesc { base_pos: trun_offset, size, offset });
            }

            // Get the total size of the track fragment run.
            let trun_size = trun.total_size(default_size);

            sample_num_rel -= trun.sample_count;
            trun_offset += trun_size;
        }

        decode_error("isomp4: invalid sample index")
    }

    fn track_sample_range(&self, track_num: usize) -> Range<u32> {
        debug_assert!(track_num < self.seq.len());

        let track = &self.seq[track_num];
        track.first_sample..track.first_sample + track.total_sample_count
    }

    fn track_ts_range(&self, track_num: usize) -> Range<u64> {
        debug_assert!(track_num < self.seq.len());

        let track = &self.seq[track_num];
        track.first_ts..track.first_ts + track.total_sample_duration
    }
}

fn get_chunk_offset(
    stco: &Option<StcoAtom>,
    co64: &Option<Co64Atom>,
    chunk: usize,
) -> Result<Option<u64>> {
    // Get the offset from either the stco or co64 atoms.
    if let Some(stco) = stco.as_ref() {
        // 32-bit offset
        if let Some(offset) = stco.chunk_offsets.get(chunk) {
            Ok(Some(u64::from(*offset)))
        }
        else {
            decode_error("isomp4: missing stco entry")
        }
    }
    else if let Some(co64) = co64.as_ref() {
        // 64-bit offset
        if let Some(offset) = co64.chunk_offsets.get(chunk) {
            Ok(Some(*offset))
        }
        else {
            decode_error("isomp4: missing co64 entry")
        }
    }
    else {
        // This should never happen because it is mandatory to have either a stco or co64 atom.
        decode_error("isomp4: missing stco or co64 atom")
    }
}

pub struct MoovSegment {
    moov: MoovAtom,
}

impl MoovSegment {
    /// Instantiate a segment from the provide moov atom.
    pub fn new(moov: MoovAtom) -> MoovSegment {
        MoovSegment { moov }
    }
}

impl StreamSegment for MoovSegment {
    fn sequence_num(&self) -> u32 {
        // The segment defined by the moov atom is always 0.
        0
    }

    fn sample_timing(&self, track_num: usize, sample_num: u32) -> Result<Option<SampleTiming>> {
        // Get the trak atom associated with track_num.
        debug_assert!(track_num < self.moov.traks.len());

        let trak = &self.moov.traks[track_num];

        // Find the sample timing. Note, complexity of O(N).
        let timing = trak.mdia.minf.stbl.stts.find_timing_for_sample(sample_num);

        if let Some((ts, dur)) = timing {
            Ok(Some(SampleTiming { ts, dur }))
        }
        else {
            Ok(None)
        }
    }

    fn ts_sample(&self, track_num: usize, ts: u64) -> Result<Option<u32>> {
        // Get the trak atom associated with track_num.
        debug_assert!(track_num < self.moov.traks.len());

        let trak = &self.moov.traks[track_num];

        // Find the sample timestamp. Note, complexity of O(N).
        Ok(trak.mdia.minf.stbl.stts.find_sample_for_timestamp(ts))
    }

    fn sample_data(
        &self,
        track_num: usize,
        sample_num: u32,
        get_offset: bool,
    ) -> Result<SampleDataDesc> {
        // Get the trak atom associated with track_num.
        debug_assert!(track_num < self.moov.traks.len());

        let trak = &self.moov.traks[track_num];

        // Get the constituent tables.
        let stsz = &trak.mdia.minf.stbl.stsz;
        let stsc = &trak.mdia.minf.stbl.stsc;
        let stco = &trak.mdia.minf.stbl.stco;
        let co64 = &trak.mdia.minf.stbl.co64;

        // Find the sample-to-chunk mapping. Note, complexity of O(log N).
        let group = stsc
            .find_entry_for_sample(sample_num)
            .ok_or(Error::DecodeError("invalid sample index"))?;

        // Index of the sample relative to the chunk group.
        let sample_in_group = sample_num - group.first_sample;

        // Index of the chunk containing the sample relative to the chunk group.
        let chunk_in_group = sample_in_group / group.samples_per_chunk;

        // Index of the chunk containing the sample relative to the entire stream.
        let chunk_in_stream = group.first_chunk + chunk_in_group;

        // Get the byte position of the first sample of the chunk containing the sample.
        let base_pos = get_chunk_offset(stco, co64, chunk_in_stream as usize)?.unwrap();

        // Determine the absolute sample byte position if requested by calculating the offset of
        // the sample from the base position of the chunk.
        let offset = if get_offset {
            // Index of the sample relative to the chunk containing the sample.
            let sample_in_chunk = sample_in_group - (chunk_in_group * group.samples_per_chunk);

            // Calculat the byte offset of the sample relative to the chunk containing it.
            let offset = match stsz.sample_sizes {
                SampleSize::Constant(size) => {
                    // Constant size samples can be calculated directly.
                    u64::from(sample_in_chunk) * u64::from(size)
                }
                SampleSize::Variable(ref entries) => {
                    // For variable size samples, sum the sizes of all the samples preceeding the
                    // desired sample in the chunk.
                    let chunk_first_sample = (sample_num - sample_in_chunk) as usize;

                    if let Some(samples) = entries.get(chunk_first_sample..sample_num as usize) {
                        samples.iter().map(|&size| u64::from(size)).sum()
                    }
                    else {
                        return decode_error("isomp4: missing one or more stsz entries");
                    }
                }
            };

            Some(offset)
        }
        else {
            None
        };

        // Get the size in bytes of the sample.
        let size = match stsz.sample_sizes {
            SampleSize::Constant(size) => size,
            SampleSize::Variable(ref entries) => {
                if let Some(size) = entries.get(sample_num as usize) {
                    *size
                }
                else {
                    return decode_error("isomp4: missing stsz entry");
                }
            }
        };

        Ok(SampleDataDesc { base_pos, size, offset })
    }

    fn track_sample_range(&self, track_num: usize) -> Range<u32> {
        debug_assert!(track_num < self.moov.traks.len());

        0..self.moov.traks[track_num].mdia.minf.stbl.stsz.sample_count
    }

    fn track_ts_range(&self, track_num: usize) -> Range<u64> {
        debug_assert!(track_num < self.moov.traks.len());

        0..self.moov.traks[track_num].mdia.minf.stbl.stts.total_duration
    }
}
