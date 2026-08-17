//! A [Tracking Table](
//! https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6trak.html) implementation.

use crate::parser::{Fixed, FromData, LazyArray16, Offset, Offset16, Offset32, Stream};

#[derive(Clone, Copy, Debug)]
struct TrackTableRecord {
    value: Fixed,
    name_id: u16,
    offset: Offset16, // Offset from start of the table.
}

impl FromData for TrackTableRecord {
    const SIZE: usize = 8;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        Some(TrackTableRecord {
            value: s.read::<Fixed>()?,
            name_id: s.read::<u16>()?,
            offset: s.read::<Offset16>()?,
        })
    }
}

/// A single track.
#[derive(Clone, Copy, Debug)]
pub struct Track<'a> {
    /// A track value.
    pub value: f32,
    /// The `name` table index for the track's name.
    pub name_index: u16,
    /// A list of tracking values for each size.
    pub values: LazyArray16<'a, i16>,
}

/// A list of tracks.
#[derive(Clone, Copy, Default, Debug)]
pub struct Tracks<'a> {
    data: &'a [u8], // the whole table
    records: LazyArray16<'a, TrackTableRecord>,
    sizes_count: u16,
}

impl<'a> Tracks<'a> {
    /// Returns a track at index.
    pub fn get(&self, index: u16) -> Option<Track<'a>> {
        let record = self.records.get(index)?;
        let mut s = Stream::new(self.data.get(record.offset.to_usize()..)?);
        Some(Track {
            value: record.value.0,
            values: s.read_array16::<i16>(self.sizes_count)?,
            name_index: record.name_id,
        })
    }

    /// Returns the number of tracks.
    pub fn len(&self) -> u16 {
        self.records.len()
    }

    /// Checks if there are any tracks.
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}

impl<'a> IntoIterator for Tracks<'a> {
    type Item = Track<'a>;
    type IntoIter = TracksIter<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        TracksIter {
            tracks: self,
            index: 0,
        }
    }
}

/// An iterator over [`Tracks`].
#[allow(missing_debug_implementations)]
pub struct TracksIter<'a> {
    tracks: Tracks<'a>,
    index: u16,
}

impl<'a> Iterator for TracksIter<'a> {
    type Item = Track<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.tracks.len() {
            self.index += 1;
            self.tracks.get(self.index - 1)
        } else {
            None
        }
    }
}

/// A track data.
#[derive(Clone, Copy, Default, Debug)]
pub struct TrackData<'a> {
    /// A list of tracks.
    pub tracks: Tracks<'a>,
    /// A list of sizes.
    pub sizes: LazyArray16<'a, Fixed>,
}

impl<'a> TrackData<'a> {
    fn parse(offset: usize, data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new_at(data, offset)?;
        let tracks_count = s.read::<u16>()?;
        let sizes_count = s.read::<u16>()?;
        let size_table_offset = s.read::<Offset32>()?; // Offset from start of the table.

        let tracks = Tracks {
            data,
            records: s.read_array16::<TrackTableRecord>(tracks_count)?,
            sizes_count,
        };

        // TODO: Isn't the size table is directly after the tracks table?!
        //       Why we need an offset then?
        let sizes = {
            let mut s = Stream::new_at(data, size_table_offset.to_usize())?;
            s.read_array16::<Fixed>(sizes_count)?
        };

        Some(TrackData { tracks, sizes })
    }
}

/// A [Tracking Table](
/// https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6trak.html).
#[derive(Clone, Copy, Debug)]
pub struct Table<'a> {
    /// Horizontal track data.
    pub horizontal: TrackData<'a>,
    /// Vertical track data.
    pub vertical: TrackData<'a>,
}

impl<'a> Table<'a> {
    /// Parses a table from raw data.
    pub fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);

        let version = s.read::<u32>()?;
        if version != 0x00010000 {
            return None;
        }

        let format = s.read::<u16>()?;
        if format != 0 {
            return None;
        }

        let hor_offset = s.read::<Option<Offset16>>()?;
        let ver_offset = s.read::<Option<Offset16>>()?;
        s.skip::<u16>(); // reserved

        let horizontal = if let Some(offset) = hor_offset {
            TrackData::parse(offset.to_usize(), data)?
        } else {
            TrackData::default()
        };

        let vertical = if let Some(offset) = ver_offset {
            TrackData::parse(offset.to_usize(), data)?
        } else {
            TrackData::default()
        };

        Some(Table {
            horizontal,
            vertical,
        })
    }
}
