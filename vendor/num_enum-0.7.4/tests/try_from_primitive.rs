use ::std::convert::{TryFrom, TryInto};

use ::num_enum::TryFromPrimitive;

// Guard against https://github.com/illicitonion/num_enum/issues/27
mod alloc {}
mod core {}
mod num_enum {}
mod std {}

#[test]
fn simple() {
    #[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
    #[repr(u8)]
    enum Enum {
        Zero,
        One,
        Two,
    }

    let zero: Result<Enum, _> = 0u8.try_into();
    assert_eq!(zero, Ok(Enum::Zero));

    let three: Result<Enum, _> = 3u8.try_into();
    assert_eq!(
        three.unwrap_err().to_string(),
        "No discriminant in enum `Enum` matches the value `3`"
    );
}

#[test]
fn even() {
    #[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
    #[repr(u8)]
    enum Enum {
        Zero = 0,
        Two = 2,
        Four = 4,
    }

    let zero: Result<Enum, _> = 0u8.try_into();
    assert_eq!(zero, Ok(Enum::Zero));

    let one: Result<Enum, _> = 1u8.try_into();
    assert_eq!(
        one.unwrap_err().to_string(),
        "No discriminant in enum `Enum` matches the value `1`"
    );

    let two: Result<Enum, _> = 2u8.try_into();
    assert_eq!(two, Ok(Enum::Two));

    let three: Result<Enum, _> = 3u8.try_into();
    assert_eq!(
        three.unwrap_err().to_string(),
        "No discriminant in enum `Enum` matches the value `3`"
    );

    let four: Result<Enum, _> = 4u8.try_into();
    assert_eq!(four, Ok(Enum::Four));
}

#[test]
fn skipped_value() {
    #[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
    #[repr(u8)]
    enum Enum {
        Zero,
        One,
        Three = 3,
        Four,
    }

    let zero: Result<Enum, _> = 0u8.try_into();
    assert_eq!(zero, Ok(Enum::Zero));

    let one: Result<Enum, _> = 1u8.try_into();
    assert_eq!(one, Ok(Enum::One));

    let two: Result<Enum, _> = 2u8.try_into();
    assert_eq!(
        two.unwrap_err().to_string(),
        "No discriminant in enum `Enum` matches the value `2`"
    );

    let three: Result<Enum, _> = 3u8.try_into();
    assert_eq!(three, Ok(Enum::Three));

    let four: Result<Enum, _> = 4u8.try_into();
    assert_eq!(four, Ok(Enum::Four));
}

#[test]
fn wrong_order() {
    #[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
    #[repr(u8)]
    enum Enum {
        Four = 4,
        Three = 3,
        Zero = 0,
        One, // Zero + 1
    }

    let zero: Result<Enum, _> = 0u8.try_into();
    assert_eq!(zero, Ok(Enum::Zero));

    let one: Result<Enum, _> = 1u8.try_into();
    assert_eq!(one, Ok(Enum::One));

    let two: Result<Enum, _> = 2u8.try_into();
    assert_eq!(
        two.unwrap_err().to_string(),
        "No discriminant in enum `Enum` matches the value `2`"
    );

    let three: Result<Enum, _> = 3u8.try_into();
    assert_eq!(three, Ok(Enum::Three));

    let four: Result<Enum, _> = 4u8.try_into();
    assert_eq!(four, Ok(Enum::Four));
}

#[test]
fn negative_values() {
    #[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
    #[repr(i8)]
    enum Enum {
        MinusTwo = -2,
        MinusOne = -1,
        Zero = 0,
        One = 1,
        Two = 2,
    }

    let minus_two: Result<Enum, _> = (-2i8).try_into();
    assert_eq!(minus_two, Ok(Enum::MinusTwo));

    let minus_one: Result<Enum, _> = (-1i8).try_into();
    assert_eq!(minus_one, Ok(Enum::MinusOne));

    let zero: Result<Enum, _> = 0i8.try_into();
    assert_eq!(zero, Ok(Enum::Zero));

    let one: Result<Enum, _> = 1i8.try_into();
    assert_eq!(one, Ok(Enum::One));

    let two: Result<Enum, _> = 2i8.try_into();
    assert_eq!(two, Ok(Enum::Two));
}

#[test]
fn discriminant_expressions() {
    const ONE: u8 = 1;

    #[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
    #[repr(u8)]
    enum Enum {
        Zero,
        One = ONE,
        Two,
        Four = 4u8,
        Five,
        Six = ONE + ONE + 2u8 + 2,
    }

    let zero: Result<Enum, _> = 0u8.try_into();
    assert_eq!(zero, Ok(Enum::Zero));

    let one: Result<Enum, _> = 1u8.try_into();
    assert_eq!(one, Ok(Enum::One));

    let two: Result<Enum, _> = 2u8.try_into();
    assert_eq!(two, Ok(Enum::Two));

    let three: Result<Enum, _> = 3u8.try_into();
    assert_eq!(
        three.unwrap_err().to_string(),
        "No discriminant in enum `Enum` matches the value `3`",
    );

    let four: Result<Enum, _> = 4u8.try_into();
    assert_eq!(four, Ok(Enum::Four));

    let five: Result<Enum, _> = 5u8.try_into();
    assert_eq!(five, Ok(Enum::Five));

    let six: Result<Enum, _> = 6u8.try_into();
    assert_eq!(six, Ok(Enum::Six));
}

#[cfg(feature = "complex-expressions")]
mod complex {
    use num_enum::TryFromPrimitive;
    use std::convert::TryInto;

    const ONE: u8 = 1;

    #[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
    #[repr(u8)]
    enum Enum {
        Zero,
        One = ONE,
        Two,
        Four = 4u8,
        Five,
        Six = ONE + ONE + 2u8 + 2,
        Seven = (7, 2).0,
    }

    #[test]
    fn different_values() {
        let zero: Result<Enum, _> = 0u8.try_into();
        assert_eq!(zero, Ok(Enum::Zero));

        let one: Result<Enum, _> = 1u8.try_into();
        assert_eq!(one, Ok(Enum::One));

        let two: Result<Enum, _> = 2u8.try_into();
        assert_eq!(two, Ok(Enum::Two));

        let three: Result<Enum, _> = 3u8.try_into();
        assert_eq!(
            three.unwrap_err().to_string(),
            "No discriminant in enum `Enum` matches the value `3`",
        );

        let four: Result<Enum, _> = 4u8.try_into();
        assert_eq!(four, Ok(Enum::Four));

        let five: Result<Enum, _> = 5u8.try_into();
        assert_eq!(five, Ok(Enum::Five));

        let six: Result<Enum, _> = 6u8.try_into();
        assert_eq!(six, Ok(Enum::Six));

        let seven: Result<Enum, _> = 7u8.try_into();
        assert_eq!(seven, Ok(Enum::Seven));
    }

    #[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
    #[repr(u8)]
    enum EnumWithExclusiveRange {
        Zero = 0,
        #[num_enum(alternatives = [2..4])]
        OneOrTwoOrThree,
    }

    #[test]
    fn different_values_with_exclusive_range() {
        let zero: Result<EnumWithExclusiveRange, _> = 0u8.try_into();
        assert_eq!(zero, Ok(EnumWithExclusiveRange::Zero));

        let one: Result<EnumWithExclusiveRange, _> = 1u8.try_into();
        assert_eq!(one, Ok(EnumWithExclusiveRange::OneOrTwoOrThree));

        let two: Result<EnumWithExclusiveRange, _> = 2u8.try_into();
        assert_eq!(two, Ok(EnumWithExclusiveRange::OneOrTwoOrThree));

        let three: Result<EnumWithExclusiveRange, _> = 3u8.try_into();
        assert_eq!(three, Ok(EnumWithExclusiveRange::OneOrTwoOrThree));

        let four: Result<EnumWithExclusiveRange, _> = 4u8.try_into();
        assert_eq!(
            four.unwrap_err().to_string(),
            "No discriminant in enum `EnumWithExclusiveRange` matches the value `4`",
        );
    }

    #[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
    #[repr(u8)]
    enum EnumWithInclusiveRange {
        Zero = 0,
        #[num_enum(alternatives = [2..=3])]
        OneOrTwoOrThree,
    }

    #[test]
    fn different_values_with_inclusive_range() {
        let zero: Result<EnumWithInclusiveRange, _> = 0u8.try_into();
        assert_eq!(zero, Ok(EnumWithInclusiveRange::Zero));

        let one: Result<EnumWithInclusiveRange, _> = 1u8.try_into();
        assert_eq!(one, Ok(EnumWithInclusiveRange::OneOrTwoOrThree));

        let two: Result<EnumWithInclusiveRange, _> = 2u8.try_into();
        assert_eq!(two, Ok(EnumWithInclusiveRange::OneOrTwoOrThree));

        let three: Result<EnumWithInclusiveRange, _> = 3u8.try_into();
        assert_eq!(three, Ok(EnumWithInclusiveRange::OneOrTwoOrThree));

        let four: Result<EnumWithInclusiveRange, _> = 4u8.try_into();
        assert_eq!(
            four.unwrap_err().to_string(),
            "No discriminant in enum `EnumWithInclusiveRange` matches the value `4`",
        );
    }
}

#[test]
fn missing_trailing_comma() {
    #[rustfmt::skip]
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
enum Enum {
    Zero,
    One
}

    let zero: Result<Enum, _> = 0u8.try_into();
    assert_eq!(zero, Ok(Enum::Zero));

    let one: Result<Enum, _> = 1u8.try_into();
    assert_eq!(one, Ok(Enum::One));

    let two: Result<Enum, _> = 2u8.try_into();
    assert_eq!(
        two.unwrap_err().to_string(),
        "No discriminant in enum `Enum` matches the value `2`"
    );
}

#[test]
fn ignores_extra_attributes() {
    #[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
    #[allow(unused)]
    #[repr(u8)]
    enum Enum {
        Zero,
        #[allow(unused)]
        One,
    }

    let zero: Result<Enum, _> = 0u8.try_into();
    assert_eq!(zero, Ok(Enum::Zero));

    let one: Result<Enum, _> = 1u8.try_into();
    assert_eq!(one, Ok(Enum::One));

    let two: Result<Enum, _> = 2u8.try_into();
    assert_eq!(
        two.unwrap_err().to_string(),
        "No discriminant in enum `Enum` matches the value `2`"
    );
}

#[test]
fn visibility_is_fine() {
    #[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
    #[repr(u8)]
    pub(crate) enum Enum {
        Zero,
        One,
    }

    let zero: Result<Enum, _> = 0u8.try_into();
    assert_eq!(zero, Ok(Enum::Zero));

    let one: Result<Enum, _> = 1u8.try_into();
    assert_eq!(one, Ok(Enum::One));

    let two: Result<Enum, _> = 2u8.try_into();
    assert_eq!(
        two.unwrap_err().to_string(),
        "No discriminant in enum `Enum` matches the value `2`"
    );
}

#[test]
fn error_variant_is_allowed() {
    #[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
    #[repr(u8)]
    pub enum Enum {
        Ok,
        Error,
    }

    let ok: Result<Enum, _> = 0u8.try_into();
    assert_eq!(ok, Ok(Enum::Ok));

    let err: Result<Enum, _> = 1u8.try_into();
    assert_eq!(err, Ok(Enum::Error));

    let unknown: Result<Enum, _> = 2u8.try_into();
    assert_eq!(
        unknown.unwrap_err().to_string(),
        "No discriminant in enum `Enum` matches the value `2`"
    );
}

#[test]
fn alternative_values() {
    #[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
    #[repr(i8)]
    enum Enum {
        Zero = 0,
        #[num_enum(alternatives = [-1, 2, 3])]
        OneTwoThreeOrMinusOne = 1,
    }

    let minus_one: Result<Enum, _> = (-1i8).try_into();
    assert_eq!(minus_one, Ok(Enum::OneTwoThreeOrMinusOne));

    let zero: Result<Enum, _> = 0i8.try_into();
    assert_eq!(zero, Ok(Enum::Zero));

    let one: Result<Enum, _> = 1i8.try_into();
    assert_eq!(one, Ok(Enum::OneTwoThreeOrMinusOne));

    let two: Result<Enum, _> = 2i8.try_into();
    assert_eq!(two, Ok(Enum::OneTwoThreeOrMinusOne));

    let three: Result<Enum, _> = 3i8.try_into();
    assert_eq!(three, Ok(Enum::OneTwoThreeOrMinusOne));

    let four: Result<Enum, _> = 4i8.try_into();
    assert_eq!(
        four.unwrap_err().to_string(),
        "No discriminant in enum `Enum` matches the value `4`"
    );
}

#[test]
fn default_value() {
    #[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
    #[repr(u8)]
    enum Enum {
        Zero = 0,
        One = 1,
        #[num_enum(default)]
        Other = 2,
    }

    let zero: Result<Enum, _> = 0u8.try_into();
    assert_eq!(zero, Ok(Enum::Zero));

    let one: Result<Enum, _> = 1u8.try_into();
    assert_eq!(one, Ok(Enum::One));

    let two: Result<Enum, _> = 2u8.try_into();
    assert_eq!(two, Ok(Enum::Other));

    let max_value: Result<Enum, _> = u8::max_value().try_into();
    assert_eq!(
        max_value.unwrap_err().to_string(),
        "No discriminant in enum `Enum` matches the value `255`"
    );
}

#[test]
fn alternative_values_and_default_value() {
    #[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
    #[repr(u8)]
    enum Enum {
        #[num_enum(default)]
        Zero = 0,
        One = 1,
        #[num_enum(alternatives = [3])]
        TwoOrThree = 2,
        Four = 4,
    }

    let zero: Result<Enum, _> = 0u8.try_into();
    assert_eq!(zero, Ok(Enum::Zero));

    let one: Result<Enum, _> = 1u8.try_into();
    assert_eq!(one, Ok(Enum::One));

    let two: Result<Enum, _> = 2u8.try_into();
    assert_eq!(two, Ok(Enum::TwoOrThree));

    let three: Result<Enum, _> = 3u8.try_into();
    assert_eq!(three, Ok(Enum::TwoOrThree));

    let four: Result<Enum, _> = 4u8.try_into();
    assert_eq!(four, Ok(Enum::Four));

    let five: Result<Enum, _> = 5u8.try_into();
    assert_eq!(
        five.unwrap_err().to_string(),
        "No discriminant in enum `Enum` matches the value `5`"
    );
}

#[test]
fn try_from_primitive_number() {
    #[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
    #[repr(u8)]
    enum Enum {
        #[num_enum(default)]
        Whatever = 0,
    }

    // #[derive(FromPrimitive)] generates implementations for the following traits:
    //
    // - `TryFromPrimitive<T>`
    // - `TryFrom<T>`

    let try_from_primitive = Enum::try_from_primitive(0_u8);
    assert_eq!(try_from_primitive, Ok(Enum::Whatever));

    let try_from = Enum::try_from(0_u8);
    assert_eq!(try_from, Ok(Enum::Whatever));
}

#[test]
fn custom_error() {
    #[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
    #[num_enum(error_type(name = CustomError, constructor = CustomError::new))]
    #[repr(u8)]
    enum FirstNumber {
        Zero,
        One,
        Two,
    }

    #[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
    #[num_enum(error_type(constructor = CustomError::new, name = CustomError))]
    #[repr(u8)]
    enum SecondNumber {
        Zero,
        One,
        Two,
    }

    #[derive(Debug, PartialEq, Eq)]
    struct CustomError {
        bad_value: u8,
    }

    impl CustomError {
        fn new(value: u8) -> CustomError {
            CustomError { bad_value: value }
        }
    }

    let zero: Result<FirstNumber, _> = 0u8.try_into();
    assert_eq!(zero, Ok(FirstNumber::Zero));

    let three: Result<FirstNumber, _> = 3u8.try_into();
    assert_eq!(three.unwrap_err(), CustomError { bad_value: 3u8 });

    let three: Result<SecondNumber, _> = 3u8.try_into();
    assert_eq!(three.unwrap_err(), CustomError { bad_value: 3u8 });
}

// #[derive(FromPrimitive)] generates implementations for the following traits:
//
// - `FromPrimitive<T>`
// - `From<T>`
// - `TryFromPrimitive<T>`
// - `TryFrom<T>`
