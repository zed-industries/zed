// Guard against https://github.com/illicitonion/num_enum/issues/27
mod alloc {}
mod core {}
mod num_enum {}
mod std {}

#[test]
fn default() {
    #[derive(Debug, Eq, PartialEq, ::num_enum::Default)]
    #[repr(u8)]
    enum Enum {
        #[allow(unused)]
        Zero = 0,
        #[num_enum(default)]
        NonZero = 1,
    }

    assert_eq!(Enum::NonZero, <Enum as ::core::default::Default>::default());
}

#[test]
fn default_standard_default_attribute() {
    #[derive(Debug, Eq, PartialEq, ::num_enum::Default)]
    #[repr(u8)]
    enum Enum {
        #[allow(unused)]
        Zero = 0,
        #[default]
        NonZero = 1,
    }

    assert_eq!(Enum::NonZero, <Enum as ::core::default::Default>::default());
}
