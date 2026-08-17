use crate::{
    boxed::{ArchivedBox, BoxResolver},
    collections::util::Entry,
    niche::option_box::{ArchivedOptionBox, OptionBoxResolver},
    ser::{ScratchSpace, Serializer},
    string::{ArchivedString, StringResolver},
    vec::{ArchivedVec, RawArchivedVec, VecResolver},
    with::{
        ArchiveWith, AsOwned, AsVec, CopyOptimize, DeserializeWith, Map, Niche, Raw, RefAsBox,
        SerializeWith, With,
    },
    Archive, ArchiveUnsized, ArchivedMetadata, Deserialize, DeserializeUnsized, Fallible,
    MetadataResolver, Serialize, SerializeUnsized,
};
use ::core::marker::PhantomData;
#[cfg(not(feature = "std"))]
use alloc::{
    borrow::Cow,
    boxed::Box,
    collections::{BTreeMap, BTreeSet},
    vec::Vec,
};
#[cfg(feature = "std")]
use std::{
    borrow::Cow,
    boxed::Box,
    collections::{BTreeMap, BTreeSet},
};

// Map for Vecs

impl<A, O> ArchiveWith<Vec<O>> for Map<A>
where
    A: ArchiveWith<O>,
{
    type Archived = ArchivedVec<<A as ArchiveWith<O>>::Archived>;
    type Resolver = VecResolver;

    unsafe fn resolve_with(
        field: &Vec<O>,
        pos: usize,
        resolver: Self::Resolver,
        out: *mut Self::Archived,
    ) {
        ArchivedVec::resolve_from_len(field.len(), pos, resolver, out)
    }
}

impl<A, O, S> SerializeWith<Vec<O>, S> for Map<A>
where
    S: Fallible + ScratchSpace + Serializer + ?Sized,
    A: ArchiveWith<O> + SerializeWith<O, S>,
{
    fn serialize_with(field: &Vec<O>, s: &mut S) -> Result<Self::Resolver, S::Error> {
        // Wrapper for O so that we have an Archive and Serialize implementation
        // and ArchivedVec::serialize_from_* is happy about the bound constraints
        struct RefWrapper<'o, A, O>(&'o O, PhantomData<A>);

        impl<A: ArchiveWith<O>, O> Archive for RefWrapper<'_, A, O> {
            type Archived = <A as ArchiveWith<O>>::Archived;
            type Resolver = <A as ArchiveWith<O>>::Resolver;

            unsafe fn resolve(
                &self,
                pos: usize,
                resolver: Self::Resolver,
                out: *mut Self::Archived,
            ) {
                A::resolve_with(self.0, pos, resolver, out)
            }
        }

        impl<A, O, S> Serialize<S> for RefWrapper<'_, A, O>
        where
            A: ArchiveWith<O> + SerializeWith<O, S>,
            S: Fallible + Serializer + ?Sized,
        {
            fn serialize(&self, s: &mut S) -> Result<Self::Resolver, S::Error> {
                A::serialize_with(self.0, s)
            }
        }

        let iter = field
            .iter()
            .map(|value| RefWrapper::<'_, A, O>(value, PhantomData));

        ArchivedVec::serialize_from_iter(iter, s)
    }
}

impl<A, O, D> DeserializeWith<ArchivedVec<<A as ArchiveWith<O>>::Archived>, Vec<O>, D> for Map<A>
where
    A: ArchiveWith<O> + DeserializeWith<<A as ArchiveWith<O>>::Archived, O, D>,
    D: Fallible + ?Sized,
{
    fn deserialize_with(
        field: &ArchivedVec<<A as ArchiveWith<O>>::Archived>,
        d: &mut D,
    ) -> Result<Vec<O>, D::Error> {
        field
            .iter()
            .map(|value| <A as DeserializeWith<_, _, D>>::deserialize_with(value, d))
            .collect()
    }
}

// AsOwned

impl<'a, F: Archive + Clone> ArchiveWith<Cow<'a, F>> for AsOwned {
    type Archived = F::Archived;
    type Resolver = F::Resolver;

    #[inline]
    unsafe fn resolve_with(
        field: &Cow<'a, F>,
        pos: usize,
        resolver: Self::Resolver,
        out: *mut Self::Archived,
    ) {
        field.resolve(pos, resolver, out);
    }
}

impl<'a, F: Serialize<S> + Clone, S: Fallible + ?Sized> SerializeWith<Cow<'a, F>, S> for AsOwned {
    #[inline]
    fn serialize_with(field: &Cow<'a, F>, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        field.serialize(serializer)
    }
}

impl<T: Archive + Clone, D: Fallible + ?Sized> DeserializeWith<T::Archived, T, D> for AsOwned
where
    T::Archived: Deserialize<T, D>,
{
    #[inline]
    fn deserialize_with(field: &T::Archived, deserializer: &mut D) -> Result<T, D::Error> {
        field.deserialize(deserializer)
    }
}

impl<'a, T: Archive + Clone> ArchiveWith<Cow<'a, [T]>> for AsOwned {
    type Archived = ArchivedVec<T::Archived>;
    type Resolver = VecResolver;

    #[inline]
    unsafe fn resolve_with(
        field: &Cow<'a, [T]>,
        pos: usize,
        resolver: Self::Resolver,
        out: *mut Self::Archived,
    ) {
        ArchivedVec::resolve_from_slice(field, pos, resolver, out);
    }
}

impl<'a, T: Serialize<S> + Clone, S: ScratchSpace + Serializer + ?Sized>
    SerializeWith<Cow<'a, [T]>, S> for AsOwned
{
    #[inline]
    fn serialize_with(
        field: &Cow<'a, [T]>,
        serializer: &mut S,
    ) -> Result<Self::Resolver, S::Error> {
        ArchivedVec::serialize_from_slice(field, serializer)
    }
}

impl<'a, T, D> DeserializeWith<ArchivedVec<T::Archived>, Cow<'a, [T]>, D> for AsOwned
where
    T: Archive + Clone,
    T::Archived: Deserialize<T, D>,
    D: Fallible + ?Sized,
{
    #[inline]
    fn deserialize_with(
        field: &ArchivedVec<T::Archived>,
        deserializer: &mut D,
    ) -> Result<Cow<'a, [T]>, D::Error> {
        Ok(Cow::Owned(field.deserialize(deserializer)?))
    }
}

impl<'a> ArchiveWith<Cow<'a, str>> for AsOwned {
    type Archived = ArchivedString;
    type Resolver = StringResolver;

    #[inline]
    unsafe fn resolve_with(
        field: &Cow<'a, str>,
        pos: usize,
        resolver: Self::Resolver,
        out: *mut Self::Archived,
    ) {
        ArchivedString::resolve_from_str(field, pos, resolver, out);
    }
}

impl<'a, S: Serializer + ?Sized> SerializeWith<Cow<'a, str>, S> for AsOwned {
    #[inline]
    fn serialize_with(
        field: &Cow<'a, str>,
        serializer: &mut S,
    ) -> Result<Self::Resolver, S::Error> {
        ArchivedString::serialize_from_str(field, serializer)
    }
}

impl<'a, D: Fallible + ?Sized> DeserializeWith<ArchivedString, Cow<'a, str>, D> for AsOwned {
    #[inline]
    fn deserialize_with(
        field: &ArchivedString,
        deserializer: &mut D,
    ) -> Result<Cow<'a, str>, D::Error> {
        Ok(Cow::Owned(field.deserialize(deserializer)?))
    }
}

#[cfg(feature = "std")]
const _: () = {
    use crate::ffi::{ArchivedCString, CStringResolver};
    use std::ffi::CStr;

    impl<'a> ArchiveWith<Cow<'a, CStr>> for AsOwned {
        type Archived = ArchivedCString;
        type Resolver = CStringResolver;

        #[inline]
        unsafe fn resolve_with(
            field: &Cow<'a, CStr>,
            pos: usize,
            resolver: Self::Resolver,
            out: *mut Self::Archived,
        ) {
            ArchivedCString::resolve_from_c_str(field, pos, resolver, out);
        }
    }

    impl<'a, S: Serializer + ?Sized> SerializeWith<Cow<'a, CStr>, S> for AsOwned {
        #[inline]
        fn serialize_with(
            field: &Cow<'a, CStr>,
            serializer: &mut S,
        ) -> Result<Self::Resolver, S::Error> {
            ArchivedCString::serialize_from_c_str(field, serializer)
        }
    }

    impl<'a, D: Fallible + ?Sized> DeserializeWith<ArchivedCString, Cow<'a, CStr>, D> for AsOwned {
        #[inline]
        fn deserialize_with(
            field: &ArchivedCString,
            deserializer: &mut D,
        ) -> Result<Cow<'a, CStr>, D::Error> {
            Ok(Cow::Owned(field.deserialize(deserializer)?))
        }
    }
};

// AsVec

impl<K: Archive, V: Archive> ArchiveWith<BTreeMap<K, V>> for AsVec {
    type Archived = ArchivedVec<Entry<K::Archived, V::Archived>>;
    type Resolver = VecResolver;

    unsafe fn resolve_with(
        field: &BTreeMap<K, V>,
        pos: usize,
        resolver: Self::Resolver,
        out: *mut Self::Archived,
    ) {
        ArchivedVec::resolve_from_len(field.len(), pos, resolver, out);
    }
}

impl<K, V, S> SerializeWith<BTreeMap<K, V>, S> for AsVec
where
    K: Serialize<S>,
    V: Serialize<S>,
    S: ScratchSpace + Serializer + ?Sized,
{
    fn serialize_with(
        field: &BTreeMap<K, V>,
        serializer: &mut S,
    ) -> Result<Self::Resolver, S::Error> {
        ArchivedVec::serialize_from_iter(
            field.iter().map(|(key, value)| Entry { key, value }),
            serializer,
        )
    }
}

impl<K, V, D> DeserializeWith<ArchivedVec<Entry<K::Archived, V::Archived>>, BTreeMap<K, V>, D>
    for AsVec
where
    K: Archive + Ord,
    V: Archive,
    K::Archived: Deserialize<K, D>,
    V::Archived: Deserialize<V, D>,
    D: Fallible + ?Sized,
{
    fn deserialize_with(
        field: &ArchivedVec<Entry<K::Archived, V::Archived>>,
        deserializer: &mut D,
    ) -> Result<BTreeMap<K, V>, D::Error> {
        let mut result = BTreeMap::new();
        for entry in field.iter() {
            result.insert(
                entry.key.deserialize(deserializer)?,
                entry.value.deserialize(deserializer)?,
            );
        }
        Ok(result)
    }
}

impl<T: Archive> ArchiveWith<BTreeSet<T>> for AsVec {
    type Archived = ArchivedVec<T::Archived>;
    type Resolver = VecResolver;

    unsafe fn resolve_with(
        field: &BTreeSet<T>,
        pos: usize,
        resolver: Self::Resolver,
        out: *mut Self::Archived,
    ) {
        ArchivedVec::resolve_from_len(field.len(), pos, resolver, out);
    }
}

impl<T, S> SerializeWith<BTreeSet<T>, S> for AsVec
where
    T: Serialize<S>,
    S: ScratchSpace + Serializer + ?Sized,
{
    fn serialize_with(field: &BTreeSet<T>, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        ArchivedVec::<T::Archived>::serialize_from_iter::<T, _, _, _>(field.iter(), serializer)
    }
}

impl<T, D> DeserializeWith<ArchivedVec<T::Archived>, BTreeSet<T>, D> for AsVec
where
    T: Archive + Ord,
    T::Archived: Deserialize<T, D>,
    D: Fallible + ?Sized,
{
    fn deserialize_with(
        field: &ArchivedVec<T::Archived>,
        deserializer: &mut D,
    ) -> Result<BTreeSet<T>, D::Error> {
        let mut result = BTreeSet::new();
        for key in field.iter() {
            result.insert(key.deserialize(deserializer)?);
        }
        Ok(result)
    }
}

// Niche

impl<T: ArchiveUnsized + ?Sized> ArchiveWith<Option<Box<T>>> for Niche
where
    ArchivedMetadata<T>: Default,
{
    type Archived = ArchivedOptionBox<T::Archived>;
    type Resolver = OptionBoxResolver<T::MetadataResolver>;

    unsafe fn resolve_with(
        field: &Option<Box<T>>,
        pos: usize,
        resolver: Self::Resolver,
        out: *mut Self::Archived,
    ) {
        ArchivedOptionBox::resolve_from_option(field.as_deref(), pos, resolver, out);
    }
}

impl<T, S> SerializeWith<Option<Box<T>>, S> for Niche
where
    T: SerializeUnsized<S> + ?Sized,
    S: Serializer + ?Sized,
    ArchivedMetadata<T>: Default,
{
    fn serialize_with(
        field: &Option<Box<T>>,
        serializer: &mut S,
    ) -> Result<Self::Resolver, S::Error> {
        ArchivedOptionBox::serialize_from_option(field.as_deref(), serializer)
    }
}

impl<T, D> DeserializeWith<ArchivedOptionBox<T::Archived>, Option<Box<T>>, D> for Niche
where
    T: ArchiveUnsized + ?Sized,
    T::Archived: DeserializeUnsized<T, D>,
    D: Fallible + ?Sized,
{
    fn deserialize_with(
        field: &ArchivedOptionBox<T::Archived>,
        deserializer: &mut D,
    ) -> Result<Option<Box<T>>, D::Error> {
        if let Some(value) = field.as_ref() {
            Ok(Some(value.deserialize(deserializer)?))
        } else {
            Ok(None)
        }
    }
}

// CopyOptimize

impl<T: Archive> ArchiveWith<Vec<T>> for CopyOptimize {
    type Archived = ArchivedVec<T::Archived>;
    type Resolver = VecResolver;

    unsafe fn resolve_with(
        field: &Vec<T>,
        pos: usize,
        resolver: Self::Resolver,
        out: *mut Self::Archived,
    ) {
        ArchivedVec::resolve_from_len(field.len(), pos, resolver, out);
    }
}

impl<T, S> SerializeWith<Vec<T>, S> for CopyOptimize
where
    T: Serialize<S>,
    S: Serializer + ?Sized,
{
    fn serialize_with(field: &Vec<T>, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        use ::core::mem::size_of;

        // Basic debug assert that T and T::Archived are at least the same size
        debug_assert_eq!(size_of::<T>(), size_of::<T::Archived>());

        unsafe { ArchivedVec::serialize_copy_from_slice(field.as_slice(), serializer) }
    }
}

impl<T, D> DeserializeWith<ArchivedVec<T::Archived>, Vec<T>, D> for CopyOptimize
where
    T: Archive,
    T::Archived: Deserialize<T, D>,
    D: Fallible + ?Sized,
{
    fn deserialize_with(field: &ArchivedVec<T::Archived>, _: &mut D) -> Result<Vec<T>, D::Error> {
        use ::core::{mem::size_of, ptr::copy_nonoverlapping};

        // Basic debug assert that T and T::Archived are at least the same size
        debug_assert_eq!(size_of::<T>(), size_of::<T::Archived>());

        let mut result = Vec::with_capacity(field.len());
        unsafe {
            copy_nonoverlapping(field.as_ptr().cast(), result.as_mut_ptr(), field.len());
            result.set_len(field.len());
        }

        Ok(result)
    }
}

impl<T: Archive> ArchiveWith<Box<[T]>> for CopyOptimize {
    type Archived = ArchivedBox<[T::Archived]>;
    type Resolver = BoxResolver<MetadataResolver<[T]>>;

    unsafe fn resolve_with(
        field: &Box<[T]>,
        pos: usize,
        resolver: Self::Resolver,
        out: *mut Self::Archived,
    ) {
        ArchivedBox::resolve_from_ref(&**field, pos, resolver, out);
    }
}

impl<T, S> SerializeWith<Box<[T]>, S> for CopyOptimize
where
    T: Serialize<S>,
    S: Serializer + ?Sized,
{
    fn serialize_with(field: &Box<[T]>, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        use ::core::mem::size_of;

        // Basic debug assert that T and T::Archived are at least the same size
        debug_assert_eq!(size_of::<T>(), size_of::<T::Archived>());

        unsafe { ArchivedBox::<[T::Archived]>::serialize_copy_from_slice(field, serializer) }
    }
}

impl<T, D> DeserializeWith<ArchivedBox<[T::Archived]>, Box<[T]>, D> for CopyOptimize
where
    T: Archive,
    T::Archived: Deserialize<T, D>,
    D: Fallible + ?Sized,
{
    fn deserialize_with(
        field: &ArchivedBox<[T::Archived]>,
        _: &mut D,
    ) -> Result<Box<[T]>, D::Error> {
        use ::core::{mem::size_of, ptr::copy_nonoverlapping};

        // Basic debug assert that T and T::Archived are at least the same size
        debug_assert_eq!(size_of::<T>(), size_of::<T::Archived>());

        let mut result = Vec::with_capacity(field.len());
        unsafe {
            copy_nonoverlapping(field.as_ptr().cast(), result.as_mut_ptr(), field.len());
            result.set_len(field.len());
        }

        Ok(result.into_boxed_slice())
    }
}

impl<'a, T: Archive> ArchiveWith<With<&'a [T], RefAsBox>> for CopyOptimize {
    type Archived = ArchivedBox<[T::Archived]>;
    type Resolver = BoxResolver<MetadataResolver<[T]>>;

    unsafe fn resolve_with(
        field: &With<&'a [T], RefAsBox>,
        pos: usize,
        resolver: Self::Resolver,
        out: *mut Self::Archived,
    ) {
        ArchivedBox::resolve_from_ref(*field.as_ref(), pos, resolver, out);
    }
}

impl<'a, T, S> SerializeWith<With<&'a [T], RefAsBox>, S> for CopyOptimize
where
    T: Serialize<S>,
    S: Serializer + ?Sized,
{
    fn serialize_with(
        field: &With<&'a [T], RefAsBox>,
        serializer: &mut S,
    ) -> Result<Self::Resolver, S::Error> {
        use ::core::mem::size_of;

        // Basic debug assert that T and T::Archived are at least the same size
        debug_assert_eq!(size_of::<T>(), size_of::<T::Archived>());

        unsafe {
            ArchivedBox::<[T::Archived]>::serialize_copy_from_slice(field.as_ref(), serializer)
        }
    }
}

// Raw

impl<T: Archive> ArchiveWith<Vec<T>> for Raw {
    type Archived = RawArchivedVec<T::Archived>;
    type Resolver = VecResolver;

    unsafe fn resolve_with(
        field: &Vec<T>,
        pos: usize,
        resolver: Self::Resolver,
        out: *mut Self::Archived,
    ) {
        RawArchivedVec::resolve_from_slice(field.as_slice(), pos, resolver, out);
    }
}

impl<T, S> SerializeWith<Vec<T>, S> for Raw
where
    T: Serialize<S>,
    S: Serializer + ?Sized,
{
    fn serialize_with(field: &Vec<T>, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        use ::core::mem::size_of;

        // Basic debug assert that T and T::Archived are at least the same size
        debug_assert_eq!(size_of::<T>(), size_of::<T::Archived>());

        unsafe { ArchivedVec::serialize_copy_from_slice(field.as_slice(), serializer) }
    }
}

impl<T, D> DeserializeWith<RawArchivedVec<T::Archived>, Vec<T>, D> for Raw
where
    T: Archive,
    T::Archived: Deserialize<T, D>,
    D: Fallible + ?Sized,
{
    fn deserialize_with(
        field: &RawArchivedVec<T::Archived>,
        _: &mut D,
    ) -> Result<Vec<T>, D::Error> {
        use ::core::{mem::size_of, ptr::copy_nonoverlapping};

        // Basic debug assert that T and T::Archived are at least the same size
        debug_assert_eq!(size_of::<T>(), size_of::<T::Archived>());

        let mut result = Vec::with_capacity(field.len());
        unsafe {
            copy_nonoverlapping(field.as_ptr().cast(), result.as_mut_ptr(), field.len());
            result.set_len(field.len());
        }

        Ok(result)
    }
}
