use core::ops::Deref;

use super::{Storage, StrInner};

use serde::de::{Deserialize, Deserializer, Error, Unexpected};
use serde::{Serialize, Serializer};

impl<S: Storage> Serialize for StrInner<S> {
    fn serialize<Ser>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error>
    where
        Ser: Serializer,
    {
        let s: &str = Deref::deref(self);
        s.serialize(serializer)
    }
}

impl<'de, S: Storage + Deserialize<'de>> Deserialize<'de> for StrInner<S> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let inner = <S as Deserialize>::deserialize(deserializer)?;
        Self::from_inner(inner).map_err(|err| {
            D::Error::invalid_value(
                Unexpected::Bytes(err.inner.as_ref()),
                &format!("Expected utf-8 str: {}", err.e).as_str(),
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{Str, StrMut};
    use serde_test::{assert_tokens, Token};

    #[test]
    fn test_de_ser_str() {
        const S: &str = "Hello, world!";
        assert_tokens(&Str::from_static(S), &[Token::BorrowedStr(S)]);
    }

    #[test]
    fn test_de_ser_str_mut() {
        const S: &str = "Hello, world!";
        assert_tokens(&StrMut::from(S), &[Token::BorrowedStr(S)]);
    }
}
