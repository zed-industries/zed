use crate::{
    ser::{ScratchSpace, Serializer},
    string::{ArchivedString, StringResolver},
    Archive, Deserialize, Fallible, Serialize,
};
use smol_str::SmolStr;

impl Archive for SmolStr {
    type Archived = ArchivedString;
    type Resolver = StringResolver;

    #[inline]
    unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
        ArchivedString::resolve_from_str(self, pos, resolver, out);
    }
}

impl<S: ScratchSpace + Serializer + ?Sized> Serialize<S> for SmolStr {
    #[inline]
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        ArchivedString::serialize_from_str(self, serializer)
    }
}

impl<D: Fallible + ?Sized> Deserialize<SmolStr, D> for ArchivedString {
    #[inline]
    fn deserialize(&self, _deserializer: &mut D) -> Result<SmolStr, D::Error> {
        Ok(SmolStr::new(self.as_str()))
    }
}

impl PartialEq<SmolStr> for ArchivedString {
    fn eq(&self, other: &SmolStr) -> bool {
        other.as_str() == self.as_str()
    }
}

#[cfg(test)]
mod tests {
    use crate::{archived_root, ser::Serializer, Deserialize, Infallible};
    use smol_str::SmolStr;

    #[test]
    fn smolstr() {
        use crate::ser::serializers::CoreSerializer;

        let value = SmolStr::new("smol_str");

        let mut serializer = CoreSerializer::<256, 256>::default();
        serializer.serialize_value(&value).unwrap();
        let end = serializer.pos();
        let result = serializer.into_serializer().into_inner();
        let archived = unsafe { archived_root::<SmolStr>(&result[0..end]) };
        assert_eq!(archived, &value);

        let deserialized: SmolStr = archived.deserialize(&mut Infallible).unwrap();
        assert_eq!(value, deserialized);
    }
}
