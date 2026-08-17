//! Serde `Serialize` and `Deserialize` implementations for `MaybeOwned`.
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use {MaybeOwned, MaybeOwnedMut};

macro_rules! serde_impls {
    ($Name:ident) => {
        impl<'a, T: Serialize> Serialize for $Name<'a, T> {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                match self {
                    Self::Owned(v) => v.serialize(serializer),
                    Self::Borrowed(v) => v.serialize(serializer),
                }
            }
        }

        impl<'a, 'de, T: Deserialize<'de>> Deserialize<'de> for $Name<'a, T> {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                T::deserialize(deserializer).map(Self::Owned)
            }
        }
    };
}

serde_impls!(MaybeOwned);
serde_impls!(MaybeOwnedMut);
