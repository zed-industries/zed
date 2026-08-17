#![cfg(feature = "with-serde")]

#[test]
fn test_serialization() {
    use num_format::{
        Buffer, CustomFormat, CustomFormatBuilder, Error, ErrorKind, Grouping, Locale,
    };
    use serde::{Deserialize, Serialize};

    fn serializable<T: Serialize>() {}

    serializable::<Buffer>();
    serializable::<CustomFormat>();
    serializable::<CustomFormatBuilder>();
    serializable::<Error>();
    serializable::<ErrorKind>();
    serializable::<Grouping>();
    serializable::<Locale>();

    fn deserializable<'de, T: Deserialize<'de>>() {}

    deserializable::<Buffer>();
    deserializable::<CustomFormat>();
    deserializable::<CustomFormatBuilder>();
    deserializable::<Error>();
    deserializable::<ErrorKind>();
    deserializable::<Grouping>();
    deserializable::<Locale>();
}

#[cfg(feature = "with-system-locale")]
#[test]
fn test_serialization_system() {
    use num_format::SystemLocale;
    use serde::{Deserialize, Serialize};

    fn serializable<T: Serialize>() {}
    serializable::<SystemLocale>();

    fn deserializable<'de, T: Deserialize<'de>>() {}
    deserializable::<SystemLocale>();
}
