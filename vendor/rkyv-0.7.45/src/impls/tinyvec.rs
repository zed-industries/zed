use crate::{
    ser::{ScratchSpace, Serializer},
    vec::{ArchivedVec, VecResolver},
    Archive, Archived, Deserialize, Fallible, Serialize,
};
#[cfg(all(feature = "tinyvec", feature = "alloc"))]
use tinyvec::TinyVec;
use tinyvec::{Array, ArrayVec, SliceVec};

// ArrayVec

impl<A: Array> Archive for ArrayVec<A>
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

impl<A: Array, S: ScratchSpace + Serializer + ?Sized> Serialize<S> for ArrayVec<A>
where
    A::Item: Serialize<S>,
{
    #[inline]
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        ArchivedVec::serialize_from_slice(self.as_slice(), serializer)
    }
}

impl<A: Array, D: Fallible + ?Sized> Deserialize<ArrayVec<A>, D> for ArchivedVec<Archived<A::Item>>
where
    A::Item: Archive,
    Archived<A::Item>: Deserialize<A::Item, D>,
{
    #[inline]
    fn deserialize(&self, deserializer: &mut D) -> Result<ArrayVec<A>, D::Error> {
        let mut result = ArrayVec::new();
        for item in self.as_slice() {
            result.push(item.deserialize(deserializer)?);
        }
        Ok(result)
    }
}

// SliceVec

impl<'s, T: Archive> Archive for SliceVec<'s, T> {
    type Archived = ArchivedVec<T::Archived>;
    type Resolver = VecResolver;

    #[inline]
    unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
        ArchivedVec::resolve_from_slice(self.as_slice(), pos, resolver, out);
    }
}

impl<'s, T: Serialize<S>, S: ScratchSpace + Serializer + ?Sized> Serialize<S> for SliceVec<'s, T> {
    #[inline]
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        ArchivedVec::serialize_from_slice(self.as_slice(), serializer)
    }
}

// SliceVec cannot be deserialized because it borrows backing memory

// TinyVec

#[cfg(all(feature = "tinyvec", feature = "alloc"))]
impl<A: Array> Archive for TinyVec<A>
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

#[cfg(all(feature = "tinyvec", feature = "alloc"))]
impl<A: Array, S: ScratchSpace + Serializer + ?Sized> Serialize<S> for TinyVec<A>
where
    A::Item: Serialize<S>,
{
    #[inline]
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        ArchivedVec::serialize_from_slice(self.as_slice(), serializer)
    }
}

#[cfg(all(feature = "tinyvec", feature = "alloc"))]
impl<A: Array, D: Fallible + ?Sized> Deserialize<TinyVec<A>, D> for ArchivedVec<Archived<A::Item>>
where
    A::Item: Archive,
    Archived<A::Item>: Deserialize<A::Item, D>,
{
    #[inline]
    fn deserialize(&self, deserializer: &mut D) -> Result<TinyVec<A>, D::Error> {
        let mut result = TinyVec::new();
        for item in self.as_slice() {
            result.push(item.deserialize(deserializer)?);
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::{archived_root, ser::Serializer, Deserialize, Infallible};
    use tinyvec::{array_vec, Array, ArrayVec, SliceVec};

    #[test]
    fn array_vec() {
        use crate::ser::serializers::CoreSerializer;

        let value = array_vec!([i32; 10] => 10, 20, 40, 80);

        let mut serializer = CoreSerializer::<256, 256>::default();
        serializer.serialize_value(&value).unwrap();
        let end = serializer.pos();
        let result = serializer.into_serializer().into_inner();
        let archived = unsafe { archived_root::<ArrayVec<[i32; 10]>>(&result[0..end]) };
        assert_eq!(archived.as_slice(), &[10, 20, 40, 80]);

        let deserialized: ArrayVec<[i32; 10]> = archived.deserialize(&mut Infallible).unwrap();
        assert_eq!(value, deserialized);
    }

    #[test]
    fn slice_vec() {
        use crate::ser::serializers::CoreSerializer;

        let mut backing = [0i32; 10];
        let mut value = SliceVec::from_slice_len(backing.as_slice_mut(), 0);
        value.push(10);
        value.push(20);
        value.push(40);
        value.push(80);

        let mut serializer = CoreSerializer::<256, 256>::default();
        serializer.serialize_value(&value).unwrap();
        let end = serializer.pos();
        let result = serializer.into_serializer().into_inner();
        let archived = unsafe { archived_root::<SliceVec<'_, i32>>(&result[0..end]) };
        assert_eq!(archived.as_slice(), &[10, 20, 40, 80]);
    }

    #[cfg(all(feature = "tinyvec", feature = "alloc"))]
    #[test]
    fn tiny_vec() {
        use crate::ser::serializers::AllocSerializer;
        #[cfg(not(feature = "std"))]
        use alloc::vec;
        use tinyvec::{tiny_vec, TinyVec};

        let value = tiny_vec!([i32; 10] => 10, 20, 40, 80);

        let mut serializer = AllocSerializer::<256>::default();
        serializer.serialize_value(&value).unwrap();
        let result = serializer.into_serializer().into_inner();
        let archived = unsafe { archived_root::<TinyVec<[i32; 10]>>(result.as_ref()) };
        assert_eq!(archived.as_slice(), &[10, 20, 40, 80]);

        let deserialized: TinyVec<[i32; 10]> = archived.deserialize(&mut Infallible).unwrap();
        assert_eq!(value, deserialized);
    }
}
