use crate::value::de::{MapDeserializer, MapRefDeserializer, SeqDeserializer, SeqRefDeserializer};
use crate::value::Value;
use crate::Error;
use serde::de::value::{BorrowedStrDeserializer, StrDeserializer};
use serde::de::{
    Deserialize, DeserializeSeed, Deserializer, EnumAccess, Error as _, VariantAccess, Visitor,
};
use serde::forward_to_deserialize_any;
use serde::ser::{Serialize, SerializeMap, Serializer};
use std::cmp::Ordering;
use std::fmt::{self, Debug, Display};
use std::hash::{Hash, Hasher};
use std::mem;

/// A representation of YAML's `!Tag` syntax, used for enums.
///
/// Refer to the example code on [`TaggedValue`] for an example of deserializing
/// tagged values.
#[derive(Clone)]
pub struct Tag {
    pub(crate) string: String,
}

/// A `Tag` + `Value` representing a tagged YAML scalar, sequence, or mapping.
///
/// ```
/// use serde_yaml::value::TaggedValue;
/// use std::collections::BTreeMap;
///
/// let yaml = "
///     scalar: !Thing x
///     sequence_flow: !Thing [first]
///     sequence_block: !Thing
///       - first
///     mapping_flow: !Thing {k: v}
///     mapping_block: !Thing
///       k: v
/// ";
///
/// let data: BTreeMap<String, TaggedValue> = serde_yaml::from_str(yaml).unwrap();
/// assert!(data["scalar"].tag == "Thing");
/// assert!(data["sequence_flow"].tag == "Thing");
/// assert!(data["sequence_block"].tag == "Thing");
/// assert!(data["mapping_flow"].tag == "Thing");
/// assert!(data["mapping_block"].tag == "Thing");
///
/// // The leading '!' in tags are not significant. The following is also true.
/// assert!(data["scalar"].tag == "!Thing");
/// ```
#[derive(Clone, PartialEq, PartialOrd, Hash, Debug)]
pub struct TaggedValue {
    #[allow(missing_docs)]
    pub tag: Tag,
    #[allow(missing_docs)]
    pub value: Value,
}

impl Tag {
    /// Create tag.
    ///
    /// The leading '!' is not significant. It may be provided, but does not
    /// have to be. The following are equivalent:
    ///
    /// ```
    /// use serde_yaml::value::Tag;
    ///
    /// assert_eq!(Tag::new("!Thing"), Tag::new("Thing"));
    ///
    /// let tag = Tag::new("Thing");
    /// assert!(tag == "Thing");
    /// assert!(tag == "!Thing");
    /// assert!(tag.to_string() == "!Thing");
    ///
    /// let tag = Tag::new("!Thing");
    /// assert!(tag == "Thing");
    /// assert!(tag == "!Thing");
    /// assert!(tag.to_string() == "!Thing");
    /// ```
    ///
    /// Such a tag would serialize to `!Thing` in YAML regardless of whether a
    /// '!' was included in the call to `Tag::new`.
    ///
    /// # Panics
    ///
    /// Panics if `string.is_empty()`. There is no syntax in YAML for an empty
    /// tag.
    pub fn new(string: impl Into<String>) -> Self {
        let tag: String = string.into();
        assert!(!tag.is_empty(), "empty YAML tag is not allowed");
        Tag { string: tag }
    }
}

impl Value {
    pub(crate) fn untag(self) -> Self {
        let mut cur = self;
        while let Value::Tagged(tagged) = cur {
            cur = tagged.value;
        }
        cur
    }

    pub(crate) fn untag_ref(&self) -> &Self {
        let mut cur = self;
        while let Value::Tagged(tagged) = cur {
            cur = &tagged.value;
        }
        cur
    }

    pub(crate) fn untag_mut(&mut self) -> &mut Self {
        let mut cur = self;
        while let Value::Tagged(tagged) = cur {
            cur = &mut tagged.value;
        }
        cur
    }
}

pub(crate) fn nobang(maybe_banged: &str) -> &str {
    match maybe_banged.strip_prefix('!') {
        Some("") | None => maybe_banged,
        Some(unbanged) => unbanged,
    }
}

impl Eq for Tag {}

impl PartialEq for Tag {
    fn eq(&self, other: &Tag) -> bool {
        PartialEq::eq(nobang(&self.string), nobang(&other.string))
    }
}

impl<T> PartialEq<T> for Tag
where
    T: ?Sized + AsRef<str>,
{
    fn eq(&self, other: &T) -> bool {
        PartialEq::eq(nobang(&self.string), nobang(other.as_ref()))
    }
}

impl Ord for Tag {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(nobang(&self.string), nobang(&other.string))
    }
}

impl PartialOrd for Tag {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for Tag {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        nobang(&self.string).hash(hasher);
    }
}

impl Display for Tag {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "!{}", nobang(&self.string))
    }
}

impl Debug for Tag {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(self, formatter)
    }
}

impl Serialize for TaggedValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        struct SerializeTag<'a>(&'a Tag);

        impl<'a> Serialize for SerializeTag<'a> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.collect_str(self.0)
            }
        }

        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry(&SerializeTag(&self.tag), &self.value)?;
        map.end()
    }
}

impl<'de> Deserialize<'de> for TaggedValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TaggedValueVisitor;

        impl<'de> Visitor<'de> for TaggedValueVisitor {
            type Value = TaggedValue;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a YAML value with a !Tag")
            }

            fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
            where
                A: EnumAccess<'de>,
            {
                let (tag, contents) = data.variant_seed(TagStringVisitor)?;
                let value = contents.newtype_variant()?;
                Ok(TaggedValue { tag, value })
            }
        }

        deserializer.deserialize_any(TaggedValueVisitor)
    }
}

impl<'de> Deserializer<'de> for TaggedValue {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_enum(self)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        drop(self);
        visitor.visit_unit()
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string bytes
        byte_buf option unit unit_struct newtype_struct seq tuple tuple_struct
        map struct enum identifier
    }
}

impl<'de> EnumAccess<'de> for TaggedValue {
    type Error = Error;
    type Variant = Value;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Error>
    where
        V: DeserializeSeed<'de>,
    {
        let tag = StrDeserializer::<Error>::new(nobang(&self.tag.string));
        let value = seed.deserialize(tag)?;
        Ok((value, self.value))
    }
}

impl<'de> VariantAccess<'de> for Value {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Error> {
        Deserialize::deserialize(self)
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Error>
    where
        T: DeserializeSeed<'de>,
    {
        seed.deserialize(self)
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        if let Value::Sequence(v) = self {
            Deserializer::deserialize_any(SeqDeserializer::new(v), visitor)
        } else {
            Err(Error::invalid_type(self.unexpected(), &"tuple variant"))
        }
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        if let Value::Mapping(v) = self {
            Deserializer::deserialize_any(MapDeserializer::new(v), visitor)
        } else {
            Err(Error::invalid_type(self.unexpected(), &"struct variant"))
        }
    }
}

impl<'de> Deserializer<'de> for &'de TaggedValue {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_enum(self)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string bytes
        byte_buf option unit unit_struct newtype_struct seq tuple tuple_struct
        map struct enum identifier
    }
}

impl<'de> EnumAccess<'de> for &'de TaggedValue {
    type Error = Error;
    type Variant = &'de Value;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Error>
    where
        V: DeserializeSeed<'de>,
    {
        let tag = BorrowedStrDeserializer::<Error>::new(nobang(&self.tag.string));
        let value = seed.deserialize(tag)?;
        Ok((value, &self.value))
    }
}

impl<'de> VariantAccess<'de> for &'de Value {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Error> {
        Deserialize::deserialize(self)
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Error>
    where
        T: DeserializeSeed<'de>,
    {
        seed.deserialize(self)
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        if let Value::Sequence(v) = self {
            Deserializer::deserialize_any(SeqRefDeserializer::new(v), visitor)
        } else {
            Err(Error::invalid_type(self.unexpected(), &"tuple variant"))
        }
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        if let Value::Mapping(v) = self {
            Deserializer::deserialize_any(MapRefDeserializer::new(v), visitor)
        } else {
            Err(Error::invalid_type(self.unexpected(), &"struct variant"))
        }
    }
}

pub(crate) struct TagStringVisitor;

impl<'de> Visitor<'de> for TagStringVisitor {
    type Value = Tag;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a YAML tag string")
    }

    fn visit_str<E>(self, string: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_string(string.to_owned())
    }

    fn visit_string<E>(self, string: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if string.is_empty() {
            return Err(E::custom("empty YAML tag is not allowed"));
        }
        Ok(Tag::new(string))
    }
}

impl<'de> DeserializeSeed<'de> for TagStringVisitor {
    type Value = Tag;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(self)
    }
}

pub(crate) enum MaybeTag<T> {
    Tag(String),
    NotTag(T),
}

pub(crate) fn check_for_tag<T>(value: &T) -> MaybeTag<String>
where
    T: ?Sized + Display,
{
    enum CheckForTag {
        Empty,
        Bang,
        Tag(String),
        NotTag(String),
    }

    impl fmt::Write for CheckForTag {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            if s.is_empty() {
                return Ok(());
            }
            match self {
                CheckForTag::Empty => {
                    if s == "!" {
                        *self = CheckForTag::Bang;
                    } else {
                        *self = CheckForTag::NotTag(s.to_owned());
                    }
                }
                CheckForTag::Bang => {
                    *self = CheckForTag::Tag(s.to_owned());
                }
                CheckForTag::Tag(string) => {
                    let mut string = mem::take(string);
                    string.push_str(s);
                    *self = CheckForTag::NotTag(string);
                }
                CheckForTag::NotTag(string) => {
                    string.push_str(s);
                }
            }
            Ok(())
        }
    }

    let mut check_for_tag = CheckForTag::Empty;
    fmt::write(&mut check_for_tag, format_args!("{}", value)).unwrap();
    match check_for_tag {
        CheckForTag::Empty => MaybeTag::NotTag(String::new()),
        CheckForTag::Bang => MaybeTag::NotTag("!".to_owned()),
        CheckForTag::Tag(string) => MaybeTag::Tag(string),
        CheckForTag::NotTag(string) => MaybeTag::NotTag(string),
    }
}
