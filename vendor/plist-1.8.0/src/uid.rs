use std::fmt;

/// A plist `uid` value. These are found exclusively in plists created by `NSKeyedArchiver`.
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Uid {
    value: u64,
}

impl Uid {
    /// Creates a new `Uid` containing the given value.
    pub fn new(value: u64) -> Uid {
        Uid { value }
    }

    /// Returns the value as a `u64`.
    pub fn get(self) -> u64 {
        self.value
    }
}

impl fmt::Debug for Uid {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.value.fmt(f)
    }
}

#[cfg(feature = "serde")]
pub mod serde_impls {
    use serde::{
        de::{Deserialize, Deserializer, Error, Visitor},
        ser::{Serialize, Serializer},
    };
    use std::fmt;

    use crate::Uid;

    pub const UID_NEWTYPE_STRUCT_NAME: &str = "PLIST-UID";

    impl Serialize for Uid {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_newtype_struct(UID_NEWTYPE_STRUCT_NAME, &self.get())
        }
    }

    struct UidNewtypeVisitor;

    impl<'de> Visitor<'de> for UidNewtypeVisitor {
        type Value = Uid;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a plist uid")
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            UidU64Visitor.visit_u64(v)
        }

        fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_u64(UidU64Visitor)
        }
    }

    struct UidU64Visitor;

    impl Visitor<'_> for UidU64Visitor {
        type Value = Uid;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a plist uid")
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(Uid::new(v))
        }
    }

    impl<'de> Deserialize<'de> for Uid {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_newtype_struct(UID_NEWTYPE_STRUCT_NAME, UidNewtypeVisitor)
        }
    }
}
