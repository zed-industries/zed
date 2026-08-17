//! The [tracking (trak)](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6trak.html) table.

include!("../../generated/generated_trak.rs");

impl<'a> TrackData<'a> {
    /// Returns the size table for this set of tracking data.
    ///
    /// The `offset_data` parameter comes from the [`Trak`] table.
    pub fn size_table(
        &self,
        offset_data: FontData<'a>,
    ) -> Result<&'a [BigEndian<Fixed>], ReadError> {
        let mut cursor = offset_data
            .split_off(self.size_table_offset() as usize)
            .ok_or(ReadError::OutOfBounds)?
            .cursor();
        cursor.read_array(self.n_sizes() as usize)
    }
}

impl TrackTableEntry {
    /// Returns the list of per-size tracking values for this entry.
    ///
    /// The `offset_data` parameter comes from the [`Trak`] table and `n_sizes`
    /// parameter comes from the parent [`TrackData`] table.
    pub fn per_size_values<'a>(
        &self,
        offset_data: FontData<'a>,
        n_sizes: u16,
    ) -> Result<&'a [BigEndian<i16>], ReadError> {
        let mut cursor = offset_data
            .split_off(self.offset() as usize)
            .ok_or(ReadError::OutOfBounds)?
            .cursor();
        cursor.read_array(n_sizes as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use font_test_data::bebuffer::BeBuffer;

    #[test]
    fn parse_header() {
        let table_data = example_track_table();
        let trak = Trak::read(FontData::new(&table_data)).unwrap();
        assert_eq!(trak.version(), MajorMinor::VERSION_1_0);
        let _ = trak.horiz().unwrap().unwrap();
        assert!(trak.vert().is_none());
    }

    #[test]
    fn parse_tracks() {
        let table_data = example_track_table();
        let trak = Trak::read(FontData::new(&table_data)).unwrap();
        let horiz = trak.horiz().unwrap().unwrap();
        let track_table = horiz.track_table();
        let expected_tracks = [
            (Fixed::from_i32(-1), NameId::new(256), 52),
            (Fixed::from_i32(0), NameId::new(258), 60),
            (Fixed::from_i32(1), NameId::new(257), 56),
        ];
        let tracks = track_table
            .iter()
            .map(|track| (track.track(), track.name_index(), track.offset()))
            .collect::<Vec<_>>();
        assert_eq!(tracks, expected_tracks);
        let expected_per_size_tracking_values = [[-15i16, -7], [0, 0], [50, 20]];
        for (track, expected_values) in track_table.iter().zip(expected_per_size_tracking_values) {
            let values = track
                .per_size_values(trak.offset_data(), horiz.n_sizes())
                .unwrap()
                .iter()
                .map(|v| v.get())
                .collect::<Vec<_>>();
            assert_eq!(values, expected_values);
        }
    }

    #[test]
    fn parse_per_size_values() {
        let table_data = example_track_table();
        let trak = Trak::read(FontData::new(&table_data)).unwrap();
        let horiz = trak.horiz().unwrap().unwrap();
        let track_table = horiz.track_table();
        let expected_per_size_tracking_values = [[-15i16, -7], [0, 0], [50, 20]];
        for (track, expected_values) in track_table.iter().zip(expected_per_size_tracking_values) {
            let values = track
                .per_size_values(trak.offset_data(), horiz.n_sizes())
                .unwrap()
                .iter()
                .map(|v| v.get())
                .collect::<Vec<_>>();
            assert_eq!(values, expected_values);
        }
    }

    #[test]
    fn parse_sizes() {
        let table_data = example_track_table();
        let trak = Trak::read(FontData::new(&table_data)).unwrap();
        let horiz = trak.horiz().unwrap().unwrap();
        let size_table = horiz
            .size_table(trak.offset_data())
            .unwrap()
            .iter()
            .map(|v| v.get())
            .collect::<Vec<_>>();
        let expected_sizes = [Fixed::from_i32(12), Fixed::from_i32(24)];
        assert_eq!(size_table, expected_sizes);
    }

    #[test]
    fn insufficient_data() {
        let mut table_data = example_track_table();
        // drop the last byte from the final per size value entry
        table_data.pop();
        let trak = Trak::read(FontData::new(&table_data)).unwrap();
        let horiz = trak.horiz().unwrap().unwrap();
        let track_table = horiz.track_table();
        // The values for the second track will fail to parse
        let expected_per_size_tracking_values = [Some([-15i16, -7]), None, Some([50, 20])];
        for (track, expected_values) in track_table.iter().zip(expected_per_size_tracking_values) {
            let values = track
                .per_size_values(trak.offset_data(), horiz.n_sizes())
                .ok()
                .map(|values| values.iter().map(|v| v.get()).collect::<Vec<_>>());
            assert_eq!(
                values,
                expected_values.map(|value| value.into_iter().collect())
            );
        }
    }

    #[test]
    fn bad_offset() {
        let mut table_data = example_track_table();
        // modify offset of first track table entry to be OOB
        table_data[26] = 255;
        let trak = Trak::read(FontData::new(&table_data)).unwrap();
        let horiz = trak.horiz().unwrap().unwrap();
        let track_table = horiz.track_table();
        assert!(matches!(
            track_table[0].per_size_values(trak.offset_data(), horiz.n_sizes()),
            Err(ReadError::OutOfBounds)
        ));
    }

    /// From <https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6trak.html>
    fn example_track_table() -> Vec<u8> {
        let mut buf = BeBuffer::new();
        // header
        buf = buf.push(MajorMinor::VERSION_1_0);
        buf = buf.extend([0u16, 12, 0, 0]);
        // TrackData
        buf = buf.extend([3u16, 2]);
        buf = buf.push(44u32);
        // Three sorted TrackTableEntry records
        buf = buf.push(0xFFFF0000u32).extend([256u16, 52]);
        buf = buf.push(0x00000000u32).extend([258u16, 60]);
        buf = buf.push(0x00010000u32).extend([257u16, 56]);
        // Size subtable
        buf = buf.push(0x000C0000u32);
        buf = buf.push(0x00180000u32);
        // Per-size tracking data values
        buf = buf.extend([-15i16, -7, 50, 20, 0, 0]);
        buf.to_vec()
    }
}
