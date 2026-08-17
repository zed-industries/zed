use crate::{
    string::{ArchivedString, StringResolver},
    Archive, Deserialize, DeserializeUnsized, Fallible, Serialize, SerializeUnsized,
};
#[cfg(not(feature = "std"))]
use ::alloc::string::{String, ToString};
use ::core::cmp::Ordering;

impl Archive for String {
    type Archived = ArchivedString;
    type Resolver = StringResolver;

    #[inline]
    unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
        ArchivedString::resolve_from_str(self.as_str(), pos, resolver, out);
    }
}

impl<S: Fallible + ?Sized> Serialize<S> for String
where
    str: SerializeUnsized<S>,
{
    #[inline]
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        ArchivedString::serialize_from_str(self.as_str(), serializer)
    }
}

impl<D: Fallible + ?Sized> Deserialize<String, D> for ArchivedString
where
    str: DeserializeUnsized<str, D>,
{
    #[inline]
    fn deserialize(&self, _: &mut D) -> Result<String, D::Error> {
        Ok(self.as_str().to_string())
    }
}

impl PartialEq<String> for ArchivedString {
    #[inline]
    fn eq(&self, other: &String) -> bool {
        PartialEq::eq(self.as_str(), other.as_str())
    }
}

impl PartialEq<ArchivedString> for String {
    #[inline]
    fn eq(&self, other: &ArchivedString) -> bool {
        PartialEq::eq(other.as_str(), self.as_str())
    }
}

impl PartialOrd<ArchivedString> for String {
    #[inline]
    fn partial_cmp(&self, other: &ArchivedString) -> Option<Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl PartialOrd<String> for ArchivedString {
    #[inline]
    fn partial_cmp(&self, other: &String) -> Option<Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}
