#[cfg(feature = "copy")]
use crate::copy::ArchiveCopyOptimize;
use crate::{
    ser::{ScratchSpace, Serializer},
    Archive, ArchivePointee, ArchiveUnsized, Archived, ArchivedMetadata, Deserialize,
    DeserializeUnsized, Fallible, FixedUsize, Serialize, SerializeUnsized,
};
use core::{alloc::Layout, mem::ManuallyDrop, ptr, str};
use ptr_meta::Pointee;

pub mod ops;
pub mod option;
pub mod primitive;
pub mod result;
pub mod time;

impl<T> ArchivePointee for T {
    type ArchivedMetadata = ();

    #[inline]
    fn pointer_metadata(_: &Self::ArchivedMetadata) -> <Self as Pointee>::Metadata {}
}

impl<T: Archive> ArchiveUnsized for T {
    type Archived = T::Archived;

    type MetadataResolver = ();

    #[inline]
    unsafe fn resolve_metadata(
        &self,
        _: usize,
        _: Self::MetadataResolver,
        _: *mut ArchivedMetadata<Self>,
    ) {
    }
}

impl<T: Serialize<S>, S: Serializer + ?Sized> SerializeUnsized<S> for T {
    #[inline]
    fn serialize_unsized(&self, serializer: &mut S) -> Result<usize, S::Error> {
        serializer.serialize_value(self)
    }

    #[inline]
    fn serialize_metadata(&self, _: &mut S) -> Result<(), S::Error> {
        Ok(())
    }
}

impl<T: Archive, D: Fallible + ?Sized> DeserializeUnsized<T, D> for T::Archived
where
    T::Archived: Deserialize<T, D>,
{
    #[inline]
    unsafe fn deserialize_unsized(
        &self,
        deserializer: &mut D,
        mut alloc: impl FnMut(Layout) -> *mut u8,
    ) -> Result<*mut (), D::Error> {
        let deserialized = self.deserialize(deserializer)?;

        let layout = Layout::new::<T>();
        if layout.size() == 0 {
            Ok(ptr::NonNull::<T>::dangling().as_ptr().cast())
        } else {
            let ptr = alloc(layout).cast::<T>();
            assert!(!ptr.is_null());
            ptr.write(deserialized);
            Ok(ptr.cast())
        }
    }

    #[inline]
    fn deserialize_metadata(&self, _: &mut D) -> Result<<T as Pointee>::Metadata, D::Error> {
        Ok(())
    }
}

#[cfg(not(feature = "strict"))]
macro_rules! peel_tuple {
    ($type:ident $index:tt, $($type_rest:ident $index_rest:tt,)*) => { impl_tuple! { $($type_rest $index_rest,)* } };
}

#[cfg(not(feature = "strict"))]
macro_rules! impl_tuple {
    () => ();
    ($($type:ident $index:tt,)+) => {
        impl<$($type: Archive),+> Archive for ($($type,)+) {
            type Archived = ($($type::Archived,)+);
            type Resolver = ($($type::Resolver,)+);

            #[inline]
            unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
                $(
                    let (fp, fo) = out_field!(out.$index);
                    self.$index.resolve(pos + fp, resolver.$index, fo);
                )+
            }
        }

        impl<$($type: Serialize<S>),+, S: Fallible + ?Sized> Serialize<S> for ($($type,)+) {
            #[inline]
            fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
                let rev = ($(self.$index.serialize(serializer)?,)+);
                Ok(($(rev.$index,)+))
            }
        }

        impl<D: Fallible + ?Sized, $($type: Archive),+> Deserialize<($($type,)+), D> for ($($type::Archived,)+)
        where
            $($type::Archived: Deserialize<$type, D>,)+
        {
            #[inline]
            fn deserialize(&self, deserializer: &mut D) -> Result<($($type,)+), D::Error> {
                let rev = ($(&self.$index,)+);
                Ok(($(rev.$index.deserialize(deserializer)?,)+))
            }
        }

        peel_tuple! { $($type $index,)+ }
    };
}

#[cfg(not(feature = "strict"))]
impl_tuple! { T11 11, T10 10, T9 9, T8 8, T7 7, T6 6, T5 5, T4 4, T3 3, T2 2, T1 1, T0 0, }

impl<T: Archive, const N: usize> Archive for [T; N] {
    type Archived = [T::Archived; N];
    type Resolver = [T::Resolver; N];

    #[inline]
    unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
        let mut resolvers = core::mem::MaybeUninit::new(resolver);
        let resolvers_ptr = resolvers.as_mut_ptr().cast::<T::Resolver>();
        let out_ptr = out.cast::<T::Archived>();
        for (i, value) in self.iter().enumerate() {
            value.resolve(
                pos + i * core::mem::size_of::<T::Archived>(),
                resolvers_ptr.add(i).read(),
                out_ptr.add(i),
            );
        }
    }
}

impl<T: Serialize<S>, S: Fallible + ?Sized, const N: usize> Serialize<S> for [T; N] {
    #[inline]
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        let mut result = core::mem::MaybeUninit::<Self::Resolver>::uninit();
        let result_ptr = result.as_mut_ptr().cast::<T::Resolver>();
        for (i, value) in self.iter().enumerate() {
            unsafe {
                result_ptr.add(i).write(value.serialize(serializer)?);
            }
        }
        unsafe { Ok(result.assume_init()) }
    }
}

impl<T: Archive, D: Fallible + ?Sized, const N: usize> Deserialize<[T; N], D> for [T::Archived; N]
where
    T::Archived: Deserialize<T, D>,
{
    #[inline]
    fn deserialize(&self, deserializer: &mut D) -> Result<[T; N], D::Error> {
        let mut result = core::mem::MaybeUninit::<[T; N]>::uninit();
        let result_ptr = result.as_mut_ptr().cast::<T>();
        for (i, value) in self.iter().enumerate() {
            unsafe {
                result_ptr.add(i).write(value.deserialize(deserializer)?);
            }
        }
        unsafe { Ok(result.assume_init()) }
    }
}

impl<T: Archive> ArchiveUnsized for [T] {
    type Archived = [T::Archived];

    type MetadataResolver = ();

    #[inline]
    unsafe fn resolve_metadata(
        &self,
        _: usize,
        _: Self::MetadataResolver,
        out: *mut ArchivedMetadata<Self>,
    ) {
        out.write(to_archived!(ptr_meta::metadata(self) as FixedUsize));
    }
}

impl<T> ArchivePointee for [T] {
    type ArchivedMetadata = Archived<usize>;

    #[inline]
    fn pointer_metadata(archived: &Self::ArchivedMetadata) -> <Self as Pointee>::Metadata {
        from_archived!(*archived) as usize
    }
}

impl<T: Serialize<S>, S: ScratchSpace + Serializer + ?Sized> SerializeUnsized<S> for [T] {
    default! {
        fn serialize_unsized(&self, serializer: &mut S) -> Result<usize, S::Error> {
            use crate::ScratchVec;

            unsafe {
                let mut resolvers = ScratchVec::new(serializer, self.len())?;

                for value in self.iter() {
                    resolvers.push(value.serialize(serializer)?);
                }
                let result = serializer.align_for::<T::Archived>()?;
                for (value, resolver) in self.iter().zip(resolvers.drain(..)) {
                    serializer.resolve_aligned(value, resolver)?;
                }

                resolvers.free(serializer)?;

                Ok(result)
            }
        }
    }

    default! {
        #[inline]
        fn serialize_metadata(&self, _: &mut S) -> Result<Self::MetadataResolver, S::Error> {
            Ok(())
        }
    }
}

#[cfg(feature = "copy")]
impl<T, S> SerializeUnsized<S> for [T]
where
    T: Serialize<S> + crate::copy::ArchiveCopyOptimize,
    S: ScratchSpace + Serializer + ?Sized,
{
    fn serialize_unsized(&self, serializer: &mut S) -> Result<usize, S::Error> {
        unsafe {
            let result = serializer.align_for::<T>()?;
            if !self.is_empty() {
                let bytes = core::slice::from_raw_parts(
                    (self.as_ptr() as *const T).cast::<u8>(),
                    self.len() * core::mem::size_of::<T>(),
                );
                serializer.write(bytes)?;
            }
            Ok(result)
        }
    }

    #[inline]
    fn serialize_metadata(&self, _: &mut S) -> Result<Self::MetadataResolver, S::Error> {
        Ok(())
    }
}

impl<T: Deserialize<U, D>, U, D: Fallible + ?Sized> DeserializeUnsized<[U], D> for [T] {
    default! {
        unsafe fn deserialize_unsized(&self, deserializer: &mut D, mut alloc: impl FnMut(Layout) -> *mut u8) -> Result<*mut (), D::Error> {
            if self.is_empty() || core::mem::size_of::<U>() == 0 {
                Ok(ptr::NonNull::<U>::dangling().as_ptr().cast())
            } else {
                let result = alloc(Layout::array::<U>(self.len()).unwrap()).cast::<U>();
                assert!(!result.is_null());
                for (i, item) in self.iter().enumerate() {
                    result.add(i).write(item.deserialize(deserializer)?);
                }
                Ok(result.cast())
            }
        }
    }

    default! {
        #[inline]
        fn deserialize_metadata(&self, _: &mut D) -> Result<<[U] as Pointee>::Metadata, D::Error> {
            Ok(ptr_meta::metadata(self))
        }
    }
}

#[cfg(feature = "copy")]
impl<T, U, D> DeserializeUnsized<[U], D> for [T]
where
    T: Deserialize<U, D>,
    U: ArchiveCopyOptimize,
    D: Fallible + ?Sized,
{
    unsafe fn deserialize_unsized(
        &self,
        _: &mut D,
        mut alloc: impl FnMut(Layout) -> *mut u8,
    ) -> Result<*mut (), D::Error> {
        if self.is_empty() || core::mem::size_of::<T>() == 0 {
            Ok(ptr::NonNull::<U>::dangling().as_ptr().cast())
        } else {
            let result = alloc(Layout::array::<T>(self.len()).unwrap()).cast::<T>();
            assert!(!result.is_null());
            ptr::copy_nonoverlapping(self.as_ptr(), result, self.len());
            Ok(result.cast())
        }
    }

    #[inline]
    fn deserialize_metadata(&self, _: &mut D) -> Result<<[T] as Pointee>::Metadata, D::Error> {
        Ok(ptr_meta::metadata(self))
    }
}

/// `str`

impl ArchiveUnsized for str {
    type Archived = str;

    type MetadataResolver = ();

    #[inline]
    unsafe fn resolve_metadata(
        &self,
        _: usize,
        _: Self::MetadataResolver,
        out: *mut ArchivedMetadata<Self>,
    ) {
        out.write(to_archived!(ptr_meta::metadata(self) as FixedUsize))
    }
}

impl ArchivePointee for str {
    type ArchivedMetadata = Archived<usize>;

    #[inline]
    fn pointer_metadata(archived: &Self::ArchivedMetadata) -> <Self as Pointee>::Metadata {
        <[u8]>::pointer_metadata(archived)
    }
}

impl<S: Serializer + ?Sized> SerializeUnsized<S> for str {
    #[inline]
    fn serialize_unsized(&self, serializer: &mut S) -> Result<usize, S::Error> {
        let result = serializer.pos();
        serializer.write(self.as_bytes())?;
        Ok(result)
    }

    #[inline]
    fn serialize_metadata(&self, _: &mut S) -> Result<Self::MetadataResolver, S::Error> {
        Ok(())
    }
}

impl<D: Fallible + ?Sized> DeserializeUnsized<str, D> for <str as ArchiveUnsized>::Archived {
    #[inline]
    unsafe fn deserialize_unsized(
        &self,
        _: &mut D,
        mut alloc: impl FnMut(Layout) -> *mut u8,
    ) -> Result<*mut (), D::Error> {
        if self.is_empty() {
            Ok(ptr::NonNull::<u8>::dangling().as_ptr().cast())
        } else {
            let bytes = alloc(Layout::array::<u8>(self.len()).unwrap());
            assert!(!bytes.is_null());
            ptr::copy_nonoverlapping(self.as_ptr(), bytes, self.len());
            Ok(bytes.cast())
        }
    }

    #[inline]
    fn deserialize_metadata(&self, _: &mut D) -> Result<<str as Pointee>::Metadata, D::Error> {
        Ok(ptr_meta::metadata(self))
    }
}

// `ManuallyDrop`

impl<T: Archive> Archive for ManuallyDrop<T> {
    type Archived = ManuallyDrop<T::Archived>;
    type Resolver = T::Resolver;

    #[inline]
    unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
        T::resolve(self, pos, resolver, out.cast::<T::Archived>())
    }
}

impl<T: Serialize<S>, S: Fallible + ?Sized> Serialize<S> for ManuallyDrop<T> {
    #[inline]
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        T::serialize(self, serializer)
    }
}

impl<T: Archive, D: Fallible + ?Sized> Deserialize<ManuallyDrop<T>, D> for ManuallyDrop<T::Archived>
where
    T::Archived: Deserialize<T, D>,
{
    #[inline]
    fn deserialize(&self, deserializer: &mut D) -> Result<ManuallyDrop<T>, <D as Fallible>::Error> {
        T::Archived::deserialize(self, deserializer).map(ManuallyDrop::new)
    }
}
