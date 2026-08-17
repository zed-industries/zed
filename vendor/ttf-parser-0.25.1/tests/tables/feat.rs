#![allow(clippy::bool_assert_comparison)]

use ttf_parser::feat::Table;
use crate::{convert, Unit::*};

#[test]
fn basic() {
    let data = convert(&[
        Fixed(1.0), // version
        UInt16(4), // number of features
        UInt16(0), // reserved
        UInt32(0), // reserved

        // Feature Name [0]
        UInt16(0), // feature
        UInt16(1), // number of settings
        UInt32(60), // offset to settings table
        UInt16(0), // flags: none
        UInt16(260), // name index

        // Feature Name [1]
        UInt16(1), // feature
        UInt16(1), // number of settings
        UInt32(64), // offset to settings table
        UInt16(0), // flags: none
        UInt16(256), // name index

        // Feature Name [2]
        UInt16(3), // feature
        UInt16(3), // number of settings
        UInt32(68), // offset to settings table
        Raw(&[0x80, 0x00]), // flags: exclusive
        UInt16(262), // name index

        // Feature Name [3]
        UInt16(6), // feature
        UInt16(2), // number of settings
        UInt32(80), // offset to settings table
        Raw(&[0xC0, 0x01]), // flags: exclusive and other
        UInt16(258), // name index

        // Setting Name [0]
        UInt16(0), // setting
        UInt16(261), // name index

        // Setting Name [1]
        UInt16(2), // setting
        UInt16(257), // name index

        // Setting Name [2]
        UInt16(0), // setting
        UInt16(268), // name index
        UInt16(3), // setting
        UInt16(264), // name index
        UInt16(4), // setting
        UInt16(265), // name index

        // Setting Name [3]
        UInt16(0), // setting
        UInt16(259), // name index
        UInt16(1), // setting
        UInt16(260), // name index
    ]);

    let table = Table::parse(&data).unwrap();
    assert_eq!(table.names.len(), 4);

    let feature0 = table.names.get(0).unwrap();
    assert_eq!(feature0.feature, 0);
    assert_eq!(feature0.setting_names.len(), 1);
    assert_eq!(feature0.exclusive, false);
    assert_eq!(feature0.name_index, 260);

    let feature2 = table.names.get(2).unwrap();
    assert_eq!(feature2.feature, 3);
    assert_eq!(feature2.setting_names.len(), 3);
    assert_eq!(feature2.exclusive, true);

    assert_eq!(feature2.setting_names.get(1).unwrap().setting, 3);
    assert_eq!(feature2.setting_names.get(1).unwrap().name_index, 264);

    let feature3 = table.names.get(3).unwrap();
    assert_eq!(feature3.default_setting_index, 1);
    assert_eq!(feature3.exclusive, true);
}
