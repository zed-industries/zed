use borsh::{from_slice, to_vec, BorshDeserialize, BorshSerialize};
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
#[borsh(init=init)]
struct A {
    lazy: Option<u64>,
}

impl A {
    pub fn init(&mut self) {
        if let Some(v) = self.lazy.as_mut() {
            *v *= 10;
        }
    }
}
#[test]
fn test_simple_struct() {
    let a = A { lazy: Some(5) };

    let encoded_a = to_vec(&a).unwrap();

    #[cfg(feature = "std")]
    insta::assert_debug_snapshot!(encoded_a);

    let decoded_a = from_slice::<A>(&encoded_a).unwrap();
    let expected_a = A { lazy: Some(50) };
    assert_eq!(expected_a, decoded_a);
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
#[borsh(init=initialization_method)]
enum AEnum {
    A,
    B,
    C,
}

impl AEnum {
    pub fn initialization_method(&mut self) {
        *self = AEnum::C;
    }
}

#[test]
fn test_simple_enum() {
    let a = AEnum::B;
    let encoded_a = to_vec(&a).unwrap();

    #[cfg(feature = "std")]
    insta::assert_debug_snapshot!(encoded_a);

    let decoded_a = from_slice::<AEnum>(&encoded_a).unwrap();
    assert_eq!(AEnum::C, decoded_a);
}
