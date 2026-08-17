use serde::{de, ser};
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};

pub(crate) const NAME: &str = "$__toml_private_Spanned";
pub(crate) const START: &str = "$__toml_private_start";
pub(crate) const END: &str = "$__toml_private_end";
pub(crate) const VALUE: &str = "$__toml_private_value";

/// A spanned value, indicating the range at which it is defined in the source.
///
/// ```
/// use serde_derive::Deserialize;
/// use toml::Spanned;
///
/// #[derive(Deserialize)]
/// struct Value {
///     s: Spanned<String>,
/// }
///
/// let t = "s = \"value\"\n";
///
/// let u: Value = toml::from_str(t).unwrap();
///
/// assert_eq!(u.s.start(), 4);
/// assert_eq!(u.s.end(), 11);
/// assert_eq!(u.s.get_ref(), "value");
/// assert_eq!(u.s.into_inner(), String::from("value"));
/// ```
#[derive(Clone, Debug)]
pub struct Spanned<T> {
    /// The start range.
    start: usize,
    /// The end range (exclusive).
    end: usize,
    /// The spanned value.
    value: T,
}

impl<T> Spanned<T> {
    /// Access the start of the span of the contained value.
    pub fn start(&self) -> usize {
        self.start
    }

    /// Access the end of the span of the contained value.
    pub fn end(&self) -> usize {
        self.end
    }

    /// Get the span of the contained value.
    pub fn span(&self) -> (usize, usize) {
        (self.start, self.end)
    }

    /// Consumes the spanned value and returns the contained value.
    pub fn into_inner(self) -> T {
        self.value
    }

    /// Returns a reference to the contained value.
    pub fn get_ref(&self) -> &T {
        &self.value
    }

    /// Returns a mutable reference to the contained value.
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl Borrow<str> for Spanned<String> {
    fn borrow(&self) -> &str {
        self.get_ref()
    }
}

impl<T: PartialEq> PartialEq for Spanned<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl<T: Eq> Eq for Spanned<T> {}

impl<T: Hash> Hash for Spanned<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl<T: PartialOrd> PartialOrd for Spanned<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<T: Ord> Ord for Spanned<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl<'de, T> de::Deserialize<'de> for Spanned<T>
where
    T: de::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Spanned<T>, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct SpannedVisitor<T>(::std::marker::PhantomData<T>);

        impl<'de, T> de::Visitor<'de> for SpannedVisitor<T>
        where
            T: de::Deserialize<'de>,
        {
            type Value = Spanned<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a TOML spanned")
            }

            fn visit_map<V>(self, mut visitor: V) -> Result<Spanned<T>, V::Error>
            where
                V: de::MapAccess<'de>,
            {
                if visitor.next_key()? != Some(START) {
                    return Err(de::Error::custom("spanned start key not found"));
                }

                let start: usize = visitor.next_value()?;

                if visitor.next_key()? != Some(END) {
                    return Err(de::Error::custom("spanned end key not found"));
                }

                let end: usize = visitor.next_value()?;

                if visitor.next_key()? != Some(VALUE) {
                    return Err(de::Error::custom("spanned value key not found"));
                }

                let value: T = visitor.next_value()?;

                Ok(Spanned { start, end, value })
            }
        }

        let visitor = SpannedVisitor(::std::marker::PhantomData);

        static FIELDS: [&str; 3] = [START, END, VALUE];
        deserializer.deserialize_struct(NAME, &FIELDS, visitor)
    }
}

impl<T: ser::Serialize> ser::Serialize for Spanned<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self.value.serialize(serializer)
    }
}
