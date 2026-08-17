use borsh::{from_slice, to_vec, BorshDeserialize, BorshSerialize};

use alloc::vec;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
enum MixedWithUnitVariants {
    A(u16),
    B,
    C { x: i32, y: i32 },
    D,
}

#[test]
fn test_mixed_enum() {
    let vars = vec![
        MixedWithUnitVariants::A(13),
        MixedWithUnitVariants::B,
        MixedWithUnitVariants::C { x: 132, y: -17 },
        MixedWithUnitVariants::D,
    ];
    for variant in vars {
        let encoded = to_vec(&variant).unwrap();
        #[cfg(feature = "std")]
        insta::assert_debug_snapshot!(encoded);

        let decoded = from_slice::<MixedWithUnitVariants>(&encoded).unwrap();

        assert_eq!(variant, decoded);
    }
}
