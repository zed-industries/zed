use std::{alloc, cmp, collections::VecDeque};

use crate::{
    ser::{ScratchSpace, Serializer},
    vec::{ArchivedVec, VecResolver},
    Archive, Deserialize, DeserializeUnsized, Fallible, Serialize,
};

impl<T: PartialEq<U>, U> PartialEq<VecDeque<U>> for ArchivedVec<T> {
    #[inline]
    fn eq(&self, other: &VecDeque<U>) -> bool {
        self.len() == other.len() && self.iter().eq(other.iter())
    }
}
impl<T: PartialEq<U>, U> PartialEq<ArchivedVec<U>> for VecDeque<T> {
    #[inline]
    fn eq(&self, other: &ArchivedVec<U>) -> bool {
        self.len() == other.len() && self.iter().eq(other.iter())
    }
}
impl<T: PartialOrd> PartialOrd<VecDeque<T>> for ArchivedVec<T> {
    #[inline]
    fn partial_cmp(&self, other: &VecDeque<T>) -> Option<cmp::Ordering> {
        self.iter().partial_cmp(other.iter())
    }
}
impl<T: PartialOrd> PartialOrd<ArchivedVec<T>> for VecDeque<T> {
    #[inline]
    fn partial_cmp(&self, other: &ArchivedVec<T>) -> Option<cmp::Ordering> {
        self.iter().partial_cmp(other.iter())
    }
}

impl<T: Archive> Archive for VecDeque<T> {
    type Archived = ArchivedVec<T::Archived>;
    type Resolver = VecResolver;

    #[inline]
    unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
        ArchivedVec::resolve_from_len(self.len(), pos, resolver, out);
    }
}

impl<T: Serialize<S>, S: ScratchSpace + Serializer + ?Sized> Serialize<S> for VecDeque<T> {
    #[inline]
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        let (a, b) = self.as_slices();
        if b.is_empty() {
            ArchivedVec::<T::Archived>::serialize_from_slice(a, serializer)
        } else if a.is_empty() {
            ArchivedVec::<T::Archived>::serialize_from_slice(b, serializer)
        } else {
            ArchivedVec::<T::Archived>::serialize_from_iter::<T, _, _, _>(self.iter(), serializer)
        }
    }
}

impl<T: Archive, D: Fallible + ?Sized> Deserialize<VecDeque<T>, D> for ArchivedVec<T::Archived>
where
    [T::Archived]: DeserializeUnsized<[T], D>,
{
    #[inline]
    fn deserialize(&self, deserializer: &mut D) -> Result<VecDeque<T>, D::Error> {
        unsafe {
            let data_address = self
                .as_slice()
                .deserialize_unsized(deserializer, |layout| alloc::alloc(layout))?;
            let metadata = self.as_slice().deserialize_metadata(deserializer)?;
            let ptr = ptr_meta::from_raw_parts_mut(data_address, metadata);
            let vec: Vec<T> = Box::<[T]>::from_raw(ptr).into();
            Ok(vec.into())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use crate::{archived_root, ser::Serializer, Deserialize, Infallible};

    #[test]
    fn vecdeque() {
        use crate::ser::serializers::CoreSerializer;

        for n in 2..10 {
            for k in 1..n {
                // Construct `deque` as containing `0..n` split across two
                // slices `0..k` and `k..n`.
                //
                // This might not work with some possible implementations of
                // `VecDeque`.  Imagine one, for example, where it fills the
                // deque starting from the middle.  However, the [documentation
                // example for `VecDeque::as_slice`][1] implies that we can do
                // this, so we might as well.
                //
                // [1]: https://doc.rust-lang.org/stable/std/collections/struct.VecDeque.html#method.as_slices
                let mut deque = VecDeque::with_capacity(n as usize + 1);
                for x in k..n {
                    deque.push_back(x);
                }
                for x in (0..k).rev() {
                    deque.push_front(x);
                }
                assert!(deque.iter().copied().eq(0..n));

                let (a, b) = deque.as_slices();
                assert!(a.iter().copied().eq(0..k));
                assert!(b.iter().copied().eq(k..n));

                // Now serialize and deserialize and verify that the
                // deserialized version contains `0..n`.
                let mut serializer = CoreSerializer::<256, 256>::default();
                serializer.serialize_value(&deque).unwrap();
                let end = serializer.pos();
                let result = serializer.into_serializer().into_inner();
                let archived = unsafe { archived_root::<VecDeque<i32>>(&result[0..end]) };
                assert!(archived.iter().copied().eq(0..n));

                let deserialized: VecDeque<i32> = archived.deserialize(&mut Infallible).unwrap();
                assert_eq!(deque, deserialized);
            }
        }
    }
}
