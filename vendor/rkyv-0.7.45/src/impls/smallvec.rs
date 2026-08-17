use crate::{
    ser::{ScratchSpace, Serializer},
    vec::{ArchivedVec, VecResolver},
    Archive, Archived, Deserialize, Fallible, Serialize,
};
use smallvec::{Array, SmallVec};

impl<A: Array> Archive for SmallVec<A>
where
    A::Item: Archive,
{
    type Archived = ArchivedVec<Archived<A::Item>>;
    type Resolver = VecResolver;

    #[inline]
    unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
        ArchivedVec::resolve_from_slice(self.as_slice(), pos, resolver, out);
    }
}

impl<A: Array, S: ScratchSpace + Serializer + ?Sized> Serialize<S> for SmallVec<A>
where
    A::Item: Serialize<S>,
{
    #[inline]
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        ArchivedVec::serialize_from_slice(self.as_slice(), serializer)
    }
}

impl<A: Array, D: Fallible + ?Sized> Deserialize<SmallVec<A>, D> for ArchivedVec<Archived<A::Item>>
where
    A::Item: Archive,
    Archived<A::Item>: Deserialize<A::Item, D>,
{
    #[inline]
    fn deserialize(&self, deserializer: &mut D) -> Result<SmallVec<A>, D::Error> {
        let mut result = SmallVec::new();
        for item in self.as_slice() {
            result.push(item.deserialize(deserializer)?);
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::{archived_root, ser::Serializer, Deserialize, Infallible};
    use smallvec::{smallvec, SmallVec};

    #[test]
    fn small_vec() {
        use crate::ser::serializers::CoreSerializer;

        let value: SmallVec<[i32; 10]> = smallvec![10, 20, 40, 80];

        let mut serializer = CoreSerializer::<256, 256>::default();
        serializer.serialize_value(&value).unwrap();
        let end = serializer.pos();
        let result = serializer.into_serializer().into_inner();
        let archived = unsafe { archived_root::<SmallVec<[i32; 10]>>(&result[0..end]) };
        assert_eq!(archived.as_slice(), &[10, 20, 40, 80]);

        let deserialized: SmallVec<[i32; 10]> = archived.deserialize(&mut Infallible).unwrap();
        assert_eq!(value, deserialized);
    }
}
