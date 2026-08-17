use crate::{
    boxed::{ArchivedBox, BoxResolver},
    Archive, ArchivePointee, ArchiveUnsized, Deserialize, DeserializeUnsized, Fallible, Serialize,
    SerializeUnsized,
};
#[cfg(not(feature = "std"))]
use ::alloc::{alloc, boxed::Box};
use ::core::cmp;
#[cfg(feature = "std")]
use ::std::alloc;

impl<T: ArchiveUnsized + ?Sized> Archive for Box<T> {
    type Archived = ArchivedBox<T::Archived>;
    type Resolver = BoxResolver<T::MetadataResolver>;

    #[inline]
    unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
        ArchivedBox::resolve_from_ref(self.as_ref(), pos, resolver, out);
    }
}

impl<T: SerializeUnsized<S> + ?Sized, S: Fallible + ?Sized> Serialize<S> for Box<T> {
    #[inline]
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        ArchivedBox::serialize_from_ref(self.as_ref(), serializer)
    }
}

impl<T, D> Deserialize<Box<T>, D> for ArchivedBox<T::Archived>
where
    T: ArchiveUnsized + ?Sized,
    T::Archived: DeserializeUnsized<T, D>,
    D: Fallible + ?Sized,
{
    #[inline]
    fn deserialize(&self, deserializer: &mut D) -> Result<Box<T>, D::Error> {
        unsafe {
            let data_address = self
                .get()
                .deserialize_unsized(deserializer, |layout| alloc::alloc(layout))?;
            let metadata = self.get().deserialize_metadata(deserializer)?;
            let ptr = ptr_meta::from_raw_parts_mut(data_address, metadata);
            Ok(Box::from_raw(ptr))
        }
    }
}

impl<T: ArchivePointee + PartialEq<U> + ?Sized, U: ?Sized> PartialEq<Box<U>> for ArchivedBox<T> {
    #[inline]
    fn eq(&self, other: &Box<U>) -> bool {
        self.get().eq(other.as_ref())
    }
}

impl<T: ArchivePointee + PartialOrd<U> + ?Sized, U: ?Sized> PartialOrd<Box<U>> for ArchivedBox<T> {
    #[inline]
    fn partial_cmp(&self, other: &Box<U>) -> Option<cmp::Ordering> {
        self.get().partial_cmp(other.as_ref())
    }
}
