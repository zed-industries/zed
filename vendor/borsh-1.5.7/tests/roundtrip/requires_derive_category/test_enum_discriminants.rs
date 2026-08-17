use alloc::vec;

use borsh::{from_slice, to_vec, BorshDeserialize, BorshSerialize};

// sequence, no unit enums
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Clone, Copy, Debug)]
#[borsh(use_discriminant = true)]
#[repr(u16)]
enum XY {
    A,
    B = 20,
    C,
    D(u32, u32),
    E = 10,
    F(u64),
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Clone, Copy, Debug)]
#[borsh(use_discriminant = false)]
#[repr(u16)]
enum XYNoDiscriminant {
    A,
    B = 20,
    C,
    D(u32, u32),
    E = 10,
    F(u64),
}

#[test]
fn test_discriminant_serde_no_unit_type() {
    let values = vec![XY::A, XY::B, XY::C, XY::E, XY::D(12, 14), XY::F(35325423)];
    let expected_discriminants = [0u8, 20, 21, 10, 22, 11];

    for (ind, value) in values.iter().enumerate() {
        let data = to_vec(value).unwrap();
        assert_eq!(data[0], expected_discriminants[ind]);
        assert_eq!(from_slice::<XY>(&data).unwrap(), values[ind]);
    }
}

#[test]
fn test_discriminant_serde_no_unit_type_no_use_discriminant() {
    let values = vec![
        XYNoDiscriminant::A,
        XYNoDiscriminant::B,
        XYNoDiscriminant::C,
        XYNoDiscriminant::D(12, 14),
        XYNoDiscriminant::E,
        XYNoDiscriminant::F(35325423),
    ];
    let expected_discriminants = [0u8, 1, 2, 3, 4, 5];

    for (ind, value) in values.iter().enumerate() {
        let data = to_vec(value).unwrap();
        assert_eq!(data[0], expected_discriminants[ind]);
        assert_eq!(from_slice::<XYNoDiscriminant>(&data).unwrap(), values[ind]);
    }
}

// minimal
#[derive(BorshSerialize)]
#[borsh(use_discriminant = true)]
enum MyDiscriminantEnum {
    A = 20,
}

#[derive(BorshSerialize)]
#[borsh(use_discriminant = false)]
enum MyDiscriminantEnumFalse {
    A = 20,
}

#[derive(BorshSerialize)]
enum MyEnumNoDiscriminant {
    A,
}
#[test]
fn test_discriminant_minimal_true() {
    assert_eq!(MyDiscriminantEnum::A as u8, 20);
    assert_eq!(to_vec(&MyDiscriminantEnum::A).unwrap(), vec![20]);
}

#[test]
fn test_discriminant_minimal_false() {
    assert_eq!(MyDiscriminantEnumFalse::A as u8, 20);
    assert_eq!(
        to_vec(&MyEnumNoDiscriminant::A).unwrap(),
        to_vec(&MyDiscriminantEnumFalse::A).unwrap(),
    );
    assert_eq!(to_vec(&MyDiscriminantEnumFalse::A).unwrap(), vec![0]);
}

// sequence
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Clone, Copy, Debug)]
#[borsh(use_discriminant = false)]
enum XNoDiscriminant {
    A,
    B = 20,
    C,
    D,
    E = 10,
    F,
}

#[test]
fn test_discriminant_serde_no_use_discriminant() {
    let values = vec![
        XNoDiscriminant::A,
        XNoDiscriminant::B,
        XNoDiscriminant::C,
        XNoDiscriminant::D,
        XNoDiscriminant::E,
        XNoDiscriminant::F,
    ];
    let expected_discriminants = [0u8, 1, 2, 3, 4, 5];
    for (index, value) in values.iter().enumerate() {
        let data = to_vec(value).unwrap();
        assert_eq!(data[0], expected_discriminants[index]);
        assert_eq!(from_slice::<XNoDiscriminant>(&data).unwrap(), values[index]);
    }
}
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
struct D {
    x: u64,
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
enum C {
    C1,
    C2(u64),
    C3(u64, u64),
    C4 { x: u64, y: u64 },
    C5(D),
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Clone, Copy, Debug)]
#[borsh(use_discriminant = true)]
enum X {
    A,
    B = 20,
    C,
    D,
    E = 10,
    F,
}

#[test]
fn test_discriminant_serialization() {
    let values = vec![X::A, X::B, X::C, X::D, X::E, X::F];
    for value in values {
        assert_eq!(to_vec(&value).unwrap(), [value as u8]);
    }
}

#[test]
fn test_discriminant_deserialization() {
    let values = vec![X::A, X::B, X::C, X::D, X::E, X::F];
    for value in values {
        assert_eq!(from_slice::<X>(&[value as u8]).unwrap(), value,);
    }
}

#[test]
#[should_panic = "Unexpected variant tag: 2"]
fn test_deserialize_invalid_discriminant() {
    from_slice::<X>(&[2]).unwrap();
}

#[test]
fn test_discriminant_serde() {
    let values = vec![X::A, X::B, X::C, X::D, X::E, X::F];
    let expected_discriminants = [0u8, 20, 21, 22, 10, 11];
    for (index, value) in values.iter().enumerate() {
        let data = to_vec(value).unwrap();
        assert_eq!(data[0], expected_discriminants[index]);
        assert_eq!(from_slice::<X>(&data).unwrap(), values[index]);
    }
}
