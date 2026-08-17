use ::num_enum::UnsafeFromPrimitive;

// Guard against https://github.com/illicitonion/num_enum/issues/27
mod alloc {}
mod core {}
mod num_enum {}
mod std {}

#[test]
fn has_unsafe_from_primitive_number() {
    #[derive(Debug, Eq, PartialEq, UnsafeFromPrimitive)]
    #[repr(u8)]
    enum Enum {
        Zero,
        One,
    }

    unsafe {
        assert_eq!(Enum::unchecked_transmute_from(0_u8), Enum::Zero);
        assert_eq!(Enum::unchecked_transmute_from(1_u8), Enum::One);
    }
}

#[test]
fn has_unsafe_from_primitive_number_with_alternatives_and_default_which_are_ignored() {
    #[derive(Debug, Eq, PartialEq, UnsafeFromPrimitive)]
    #[repr(u8)]
    enum Enum {
        Zero,
        One,
        #[num_enum(alternatives = [3, 4])]
        Some,
        #[num_enum(default)]
        Many = 5,
    }

    unsafe {
        assert_eq!(Enum::unchecked_transmute_from(0_u8), Enum::Zero);
        assert_eq!(Enum::unchecked_transmute_from(1_u8), Enum::One);
        assert_eq!(Enum::unchecked_transmute_from(2_u8), Enum::Some);
        assert_eq!(Enum::unchecked_transmute_from(5_u8), Enum::Many);
        // Any other conversions would be undefined behavior.
    }

    #[allow(deprecated)]
    unsafe {
        assert_eq!(Enum::from_unchecked(0_u8), Enum::Zero);
        assert_eq!(Enum::from_unchecked(1_u8), Enum::One);
        assert_eq!(Enum::from_unchecked(2_u8), Enum::Some);
        assert_eq!(Enum::from_unchecked(5_u8), Enum::Many);
    }
}

#[test]
fn has_unsafe_from_primitive_number_with_alternatives_and_std_default_which_are_ignored() {
    #[derive(Debug, Default, Eq, PartialEq, UnsafeFromPrimitive)]
    #[repr(u8)]
    enum Enum {
        Zero,
        One,
        #[num_enum(alternatives = [3, 4])]
        Some,
        #[default]
        Many = 5,
    }

    unsafe {
        assert_eq!(Enum::unchecked_transmute_from(0_u8), Enum::Zero);
        assert_eq!(Enum::unchecked_transmute_from(1_u8), Enum::One);
        assert_eq!(Enum::unchecked_transmute_from(2_u8), Enum::Some);
        assert_eq!(Enum::unchecked_transmute_from(5_u8), Enum::Many);
        // Any other conversions would be undefined behavior.
    }

    #[allow(deprecated)]
    unsafe {
        assert_eq!(Enum::from_unchecked(0_u8), Enum::Zero);
        assert_eq!(Enum::from_unchecked(1_u8), Enum::One);
        assert_eq!(Enum::from_unchecked(2_u8), Enum::Some);
        assert_eq!(Enum::from_unchecked(5_u8), Enum::Many);
    }
}
