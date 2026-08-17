use crate::{
    ser::{ScratchSpace, Serializer},
    vec::{ArchivedVec, VecResolver},
    Archive, Archived, Deserialize, Fallible, Serialize,
};
use arrayvec::ArrayVec;

impl<T, const CAP: usize> Archive for ArrayVec<T, CAP>
where
    T: Archive,
{
    type Archived = ArchivedVec<Archived<T>>;
    type Resolver = VecResolver;

    #[inline]
    unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
        ArchivedVec::resolve_from_slice(self.as_slice(), pos, resolver, out);
    }
}

impl<T, S: ScratchSpace + Serializer + ?Sized, const CAP: usize> Serialize<S> for ArrayVec<T, CAP>
where
    T: Serialize<S>,
{
    #[inline]
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        ArchivedVec::serialize_from_slice(self.as_slice(), serializer)
    }
}

impl<T, D: Fallible + ?Sized, const CAP: usize> Deserialize<ArrayVec<T, CAP>, D>
    for ArchivedVec<Archived<T>>
where
    T: Archive,
    Archived<T>: Deserialize<T, D>,
{
    #[inline]
    fn deserialize(&self, deserializer: &mut D) -> Result<ArrayVec<T, CAP>, D::Error> {
        let mut result = ArrayVec::new();
        for item in self.as_slice() {
            result.push(item.deserialize(deserializer)?);
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::{archived_root, ser::Serializer, Deserialize, Infallible};
    use arrayvec::ArrayVec;

    #[test]
    fn array_vec() {
        use crate::ser::serializers::CoreSerializer;

        let value: ArrayVec<i32, 4> = ArrayVec::from([10, 20, 40, 80]);

        let mut serializer = CoreSerializer::<256, 256>::default();
        serializer.serialize_value(&value).unwrap();
        let end = serializer.pos();
        let result = serializer.into_serializer().into_inner();
        let archived = unsafe { archived_root::<ArrayVec<i32, 4>>(&result[0..end]) };
        assert_eq!(archived.as_slice(), &[10, 20, 40, 80]);

        let deserialized: ArrayVec<i32, 4> = archived.deserialize(&mut Infallible).unwrap();
        assert_eq!(value, deserialized);
    }
}
