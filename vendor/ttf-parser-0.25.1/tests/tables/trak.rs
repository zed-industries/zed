use ttf_parser::trak::Table;
use crate::{convert, Unit::*};

#[test]
fn empty() {
    let data = convert(&[
        Fixed(1.0), // version
        UInt16(0), // format
        UInt16(0), // horizontal data offset
        UInt16(0), // vertical data offset
        UInt16(0), // padding
    ]);

    let table = Table::parse(&data).unwrap();
    assert_eq!(table.horizontal.tracks.len(), 0);
    assert_eq!(table.horizontal.sizes.len(), 0);
    assert_eq!(table.vertical.tracks.len(), 0);
    assert_eq!(table.vertical.sizes.len(), 0);
}

#[test]
fn basic() {
    let data = convert(&[
        Fixed(1.0), // version
        UInt16(0), // format
        UInt16(12), // horizontal data offset
        UInt16(0), // vertical data offset
        UInt16(0), // padding

        // TrackData
        UInt16(3), // number of tracks
        UInt16(2), // number of sizes
        UInt32(44), // offset to size table

        // TrackTableEntry [0]
        Fixed(-1.0), // track
        UInt16(256), // name index
        UInt16(52), // offset of the two per-size tracking values

        // TrackTableEntry [1]
        Fixed(0.0), // track
        UInt16(258), // name index
        UInt16(60), // offset of the two per-size tracking values

        // TrackTableEntry [2]
        Fixed(1.0), // track
        UInt16(257), // name index
        UInt16(56), // offset of the two per-size tracking values

        // Size [0]
        Fixed(12.0), // points
        // Size [1]
        Fixed(24.0), // points

        // Per-size tracking values.
        Int16(-15),
        Int16(-7),
        Int16(50),
        Int16(20),
        Int16(0),
        Int16(0),
    ]);

    let table = Table::parse(&data).unwrap();

    assert_eq!(table.horizontal.tracks.len(), 3);
    assert_eq!(table.horizontal.tracks.get(0).unwrap().value, -1.0);
    assert_eq!(table.horizontal.tracks.get(1).unwrap().value, 0.0);
    assert_eq!(table.horizontal.tracks.get(2).unwrap().value, 1.0);
    assert_eq!(table.horizontal.tracks.get(0).unwrap().name_index, 256);
    assert_eq!(table.horizontal.tracks.get(1).unwrap().name_index, 258);
    assert_eq!(table.horizontal.tracks.get(2).unwrap().name_index, 257);
    assert_eq!(table.horizontal.tracks.get(0).unwrap().values.len(), 2);
    assert_eq!(table.horizontal.tracks.get(0).unwrap().values.get(0).unwrap(), -15);
    assert_eq!(table.horizontal.tracks.get(0).unwrap().values.get(1).unwrap(), -7);
    assert_eq!(table.horizontal.tracks.get(1).unwrap().values.len(), 2);
    assert_eq!(table.horizontal.tracks.get(1).unwrap().values.get(0).unwrap(), 0);
    assert_eq!(table.horizontal.tracks.get(1).unwrap().values.get(1).unwrap(), 0);
    assert_eq!(table.horizontal.tracks.get(2).unwrap().values.len(), 2);
    assert_eq!(table.horizontal.tracks.get(2).unwrap().values.get(0).unwrap(), 50);
    assert_eq!(table.horizontal.tracks.get(2).unwrap().values.get(1).unwrap(), 20);
    assert_eq!(table.horizontal.sizes.len(), 2);
    assert_eq!(table.horizontal.sizes.get(0).unwrap().0, 12.0);
    assert_eq!(table.horizontal.sizes.get(1).unwrap().0, 24.0);

    assert_eq!(table.vertical.tracks.len(), 0);
    assert_eq!(table.vertical.sizes.len(), 0);
}
