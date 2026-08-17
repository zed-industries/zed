use serde::ser::{Serialize, Serializer};
use std::fmt::Display;

use crate::{Error, Signature, Type, Value, value_display_fmt};

/// A helper type to wrap `Option<T>` (GVariant's Maybe type) in [`Value`].
///
/// API is provided to convert from, and to `Option<T>`.
///
/// [`Value`]: enum.Value.html
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Maybe<'a> {
    value: Box<Option<Value<'a>>>,
    signature: Signature,
}

impl<'a> Maybe<'a> {
    /// Get a reference to underlying value.
    pub fn inner(&self) -> &Option<Value<'a>> {
        &self.value
    }

    /// Create a new Just (Some) `Maybe`.
    pub fn just(value: Value<'a>) -> Self {
        let value_signature = value.value_signature().clone();
        let signature = Signature::maybe(value_signature);
        Self {
            signature,
            value: Box::new(Some(value)),
        }
    }

    pub(crate) fn just_full_signature(value: Value<'a>, signature: &Signature) -> Self {
        Self {
            signature: signature.clone(),
            value: Box::new(Some(value)),
        }
    }

    /// Create a new Nothing (None) `Maybe`, given the signature of the type.
    pub fn nothing(value_signature: &Signature) -> Self {
        let signature = Signature::maybe(value_signature.clone());
        Self {
            signature,
            value: Box::new(None),
        }
    }

    pub(crate) fn nothing_full_signature(signature: &Signature) -> Self {
        Self {
            signature: signature.clone(),
            value: Box::new(None),
        }
    }

    /// Get the inner value as a concrete type
    pub fn get<T>(&'a self) -> core::result::Result<Option<T>, Error>
    where
        T: TryFrom<&'a Value<'a>>,
        <T as TryFrom<&'a Value<'a>>>::Error: Into<crate::Error>,
    {
        self.value
            .as_ref()
            .as_ref()
            .map(|v| v.downcast_ref())
            .transpose()
    }

    /// Get the signature of `Maybe`.
    pub fn signature(&self) -> &Signature {
        &self.signature
    }

    /// Get the signature of the potential value in the `Maybe`.
    pub fn value_signature(&self) -> &Signature {
        match self.signature() {
            Signature::Maybe(signature) => signature,
            _ => unreachable!("Invalid `Maybe` signature"),
        }
    }

    pub(crate) fn try_to_owned(&self) -> crate::Result<Maybe<'static>> {
        Ok(Maybe {
            value: Box::new(
                self.value
                    .as_ref()
                    .as_ref()
                    .map(|v| v.try_to_owned().map(Into::into))
                    .transpose()?,
            ),
            signature: self.signature.clone(),
        })
    }

    pub(crate) fn try_into_owned(self) -> crate::Result<Maybe<'static>> {
        Ok(Maybe {
            value: Box::new(
                self.value
                    .map(|v| v.try_into_owned().map(Into::into))
                    .transpose()?,
            ),
            signature: self.signature,
        })
    }

    /// Attempt to clone `self`.
    pub fn try_clone(&self) -> Result<Self, crate::Error> {
        Ok(Maybe {
            value: Box::new(
                self.value
                    .as_ref()
                    .as_ref()
                    .map(|v| v.try_clone())
                    .transpose()?,
            ),
            signature: self.signature.clone(),
        })
    }
}

impl Display for Maybe<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        maybe_display_fmt(self, f, true)
    }
}

pub(crate) fn maybe_display_fmt(
    maybe: &Maybe<'_>,
    f: &mut std::fmt::Formatter<'_>,
    type_annotate: bool,
) -> std::fmt::Result {
    if type_annotate {
        write!(f, "@{} ", maybe.signature())?;
    }

    let (last_inner, depth) = {
        let mut curr = maybe.inner();
        let mut depth = 0;

        while let Some(Value::Maybe(child_maybe)) = curr {
            curr = child_maybe.inner();
            depth += 1;
        }

        (curr, depth)
    };

    if let Some(last_inner) = last_inner {
        // There are no Nothings, so print out the inner value with no prefixes.
        value_display_fmt(last_inner, f, false)?;
    } else {
        // One of the maybes was Nothing, so print out the right number of justs.
        for _ in 0..depth {
            f.write_str("just ")?;
        }
        f.write_str("nothing")?;
    }

    Ok(())
}

impl<'a, T> From<Option<T>> for Maybe<'a>
where
    T: Type + Into<Value<'a>>,
{
    fn from(value: Option<T>) -> Self {
        value
            .map(|v| Self::just(Value::new(v)))
            .unwrap_or_else(|| Self::nothing(T::SIGNATURE))
    }
}

impl<'a, T> From<&Option<T>> for Maybe<'a>
where
    T: Type + Into<Value<'a>> + Clone,
{
    fn from(value: &Option<T>) -> Self {
        value
            .as_ref()
            .map(|v| Self::just(Value::new(v.clone())))
            .unwrap_or_else(|| Self::nothing(T::SIGNATURE))
    }
}

impl<'a> Serialize for Maybe<'a> {
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &*self.value {
            Some(value) => value.serialize_value_as_some(serializer),
            None => serializer.serialize_none(),
        }
    }
}
