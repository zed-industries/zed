//! ensure serde is working as expected

use super::*;

#[test]
fn test_serde() {
    #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq)]
    struct MyTypes {
        f1: Fixed,
        f2: F26Dot6,
        f3: F2Dot14,
        gid: GlyphId16,
        date: LongDateTime,
        name_id: NameId,
        offset: Offset16,
        tag: Tag,
        u24: Uint24,
        version1: MajorMinor,
        version2: Version16Dot16,
    }

    let my_instance = MyTypes {
        f1: Fixed::from_f64(521.5),
        f2: F26Dot6::from_f64(-1001.1),
        f3: F2Dot14::from_f32(1.2),
        gid: GlyphId16::new(69),
        date: LongDateTime::new(1_234_569_101),
        name_id: NameId::new(8214),
        offset: Offset16::new(42),
        tag: Tag::new(b"cool"),
        u24: Uint24::new(16_777_215),
        version1: MajorMinor::new(10, 5),
        version2: Version16Dot16::VERSION_2_5,
    };

    let dumped = serde_json::to_string(&my_instance).unwrap();
    let loaded: MyTypes = serde_json::from_str(&dumped).unwrap();
    assert_eq!(my_instance, loaded)
}
