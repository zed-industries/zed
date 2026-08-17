use crate::{
    ser::{ScratchSpace, Serializer},
    vec::{ArchivedVec, VecResolver},
    Archive, Archived, Deserialize, Fallible, Serialize,
};
use bytes::{Bytes, BytesMut};

impl Archive for Bytes {
    type Archived = ArchivedVec<u8>;
    type Resolver = VecResolver;

    #[inline]
    unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
        ArchivedVec::resolve_from_slice(self, pos, resolver, out);
    }
}

impl<S: ScratchSpace + Serializer + ?Sized> Serialize<S> for Bytes {
    #[inline]
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        ArchivedVec::serialize_from_slice(self, serializer)
    }
}

impl<D: Fallible + ?Sized> Deserialize<Bytes, D> for ArchivedVec<Archived<u8>> {
    #[inline]
    fn deserialize(&self, _deserializer: &mut D) -> Result<Bytes, D::Error> {
        let mut result = BytesMut::new();
        result.extend_from_slice(self.as_slice());
        Ok(result.freeze())
    }
}

impl<T: Archive> PartialEq<Bytes> for ArchivedVec<T>
where
    bytes::Bytes: PartialEq<[T]>,
{
    fn eq(&self, other: &Bytes) -> bool {
        other == self.as_slice()
    }
}

#[cfg(test)]
mod tests {
    use crate::{archived_root, ser::Serializer, Deserialize, Infallible};
    use bytes::Bytes;

    #[test]
    fn bytes() {
        use crate::ser::serializers::CoreSerializer;

        let value = Bytes::from(vec![10, 20, 40, 80]);

        let mut serializer = CoreSerializer::<256, 256>::default();
        serializer.serialize_value(&value).unwrap();
        let end = serializer.pos();
        let result = serializer.into_serializer().into_inner();
        let archived = unsafe { archived_root::<Bytes>(&result[0..end]) };
        assert_eq!(archived, &value);

        let deserialized: Bytes = archived.deserialize(&mut Infallible).unwrap();
        assert_eq!(value, deserialized);
    }
}
