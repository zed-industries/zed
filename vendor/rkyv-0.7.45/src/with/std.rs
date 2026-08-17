use crate::{
    collections::util::Entry,
    ser::{ScratchSpace, Serializer},
    string::{ArchivedString, StringResolver},
    time::ArchivedDuration,
    vec::{ArchivedVec, VecResolver},
    with::{
        ArchiveWith, AsString, AsStringError, AsVec, DeserializeWith, Immutable, Lock, LockError,
        SerializeWith, UnixTimestamp, UnixTimestampError,
    },
    Archive, Deserialize, Fallible, Serialize, SerializeUnsized,
};
use core::{hash::Hash, str::FromStr};
use std::{
    collections::{HashMap, HashSet},
    ffi::OsString,
    path::PathBuf,
    sync::{Mutex, RwLock},
    time::{SystemTime, UNIX_EPOCH},
};

// AsString

impl ArchiveWith<OsString> for AsString {
    type Archived = ArchivedString;
    type Resolver = StringResolver;

    #[inline]
    unsafe fn resolve_with(
        field: &OsString,
        pos: usize,
        resolver: Self::Resolver,
        out: *mut Self::Archived,
    ) {
        // It's safe to unwrap here because if the OsString wasn't valid UTF-8 it would have failed
        // to serialize
        ArchivedString::resolve_from_str(field.to_str().unwrap(), pos, resolver, out);
    }
}

impl<S: Fallible + ?Sized> SerializeWith<OsString, S> for AsString
where
    S::Error: From<AsStringError>,
    str: SerializeUnsized<S>,
{
    #[inline]
    fn serialize_with(field: &OsString, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        ArchivedString::serialize_from_str(
            field.to_str().ok_or(AsStringError::InvalidUTF8)?,
            serializer,
        )
    }
}

impl<D: Fallible + ?Sized> DeserializeWith<ArchivedString, OsString, D> for AsString {
    #[inline]
    fn deserialize_with(field: &ArchivedString, _: &mut D) -> Result<OsString, D::Error> {
        Ok(OsString::from_str(field.as_str()).unwrap())
    }
}

impl ArchiveWith<PathBuf> for AsString {
    type Archived = ArchivedString;
    type Resolver = StringResolver;

    #[inline]
    unsafe fn resolve_with(
        field: &PathBuf,
        pos: usize,
        resolver: Self::Resolver,
        out: *mut Self::Archived,
    ) {
        // It's safe to unwrap here because if the OsString wasn't valid UTF-8 it would have failed
        // to serialize
        ArchivedString::resolve_from_str(field.to_str().unwrap(), pos, resolver, out);
    }
}

impl<S: Fallible + ?Sized> SerializeWith<PathBuf, S> for AsString
where
    S::Error: From<AsStringError>,
    str: SerializeUnsized<S>,
{
    #[inline]
    fn serialize_with(field: &PathBuf, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        ArchivedString::serialize_from_str(
            field.to_str().ok_or(AsStringError::InvalidUTF8)?,
            serializer,
        )
    }
}

impl<D: Fallible + ?Sized> DeserializeWith<ArchivedString, PathBuf, D> for AsString {
    #[inline]
    fn deserialize_with(field: &ArchivedString, _: &mut D) -> Result<PathBuf, D::Error> {
        Ok(PathBuf::from_str(field.as_str()).unwrap())
    }
}

// Lock

impl<F: Archive> ArchiveWith<Mutex<F>> for Lock {
    type Archived = Immutable<F::Archived>;
    type Resolver = F::Resolver;

    #[inline]
    unsafe fn resolve_with(
        field: &Mutex<F>,
        pos: usize,
        resolver: Self::Resolver,
        out: *mut Self::Archived,
    ) {
        // Unfortunately, we have to unwrap here because resolve must be infallible
        //
        // An alternative would be to only implement ArchiveWith for Arc<Mutex<F>>, copy an Arc into
        // the resolver, and hold the guard in there as well (as a reference to the internal mutex).
        // This unfortunately will cause a deadlock if two Arcs to the same Mutex are serialized
        // before the first is resolved. The compromise is, unfortunately, to just unwrap poison
        // errors here and document it.
        field.lock().unwrap().resolve(pos, resolver, out.cast());
    }
}

impl<F: Serialize<S>, S: Fallible + ?Sized> SerializeWith<Mutex<F>, S> for Lock
where
    S::Error: From<LockError>,
{
    #[inline]
    fn serialize_with(field: &Mutex<F>, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        field
            .lock()
            .map_err(|_| LockError::Poisoned)?
            .serialize(serializer)
    }
}

impl<F, T, D> DeserializeWith<Immutable<F>, Mutex<T>, D> for Lock
where
    F: Deserialize<T, D>,
    D: Fallible + ?Sized,
{
    #[inline]
    fn deserialize_with(field: &Immutable<F>, deserializer: &mut D) -> Result<Mutex<T>, D::Error> {
        Ok(Mutex::new(field.value().deserialize(deserializer)?))
    }
}

impl<F: Archive> ArchiveWith<RwLock<F>> for Lock {
    type Archived = Immutable<F::Archived>;
    type Resolver = F::Resolver;

    #[inline]
    unsafe fn resolve_with(
        field: &RwLock<F>,
        pos: usize,
        resolver: Self::Resolver,
        out: *mut Self::Archived,
    ) {
        // Unfortunately, we have to unwrap here because resolve must be infallible
        //
        // An alternative would be to only implement ArchiveWith for Arc<Mutex<F>>, copy a an Arc
        // into the resolver, and hold the guard in there as well (as a reference to the internal
        // mutex). This unfortunately will cause a deadlock if two Arcs to the same Mutex are
        // serialized before the first is resolved. The compromise is, unfortunately, to just
        // unwrap poison errors here and document it.
        field.read().unwrap().resolve(pos, resolver, out.cast());
    }
}

impl<F: Serialize<S>, S: Fallible + ?Sized> SerializeWith<RwLock<F>, S> for Lock
where
    S::Error: From<LockError>,
{
    #[inline]
    fn serialize_with(field: &RwLock<F>, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        field
            .read()
            .map_err(|_| LockError::Poisoned)?
            .serialize(serializer)
    }
}

impl<F, T, D> DeserializeWith<Immutable<F>, RwLock<T>, D> for Lock
where
    F: Deserialize<T, D>,
    D: Fallible + ?Sized,
{
    #[inline]
    fn deserialize_with(field: &Immutable<F>, deserializer: &mut D) -> Result<RwLock<T>, D::Error> {
        Ok(RwLock::new(field.value().deserialize(deserializer)?))
    }
}

// AsVec

impl<K: Archive, V: Archive> ArchiveWith<HashMap<K, V>> for AsVec {
    type Archived = ArchivedVec<Entry<K::Archived, V::Archived>>;
    type Resolver = VecResolver;

    unsafe fn resolve_with(
        field: &HashMap<K, V>,
        pos: usize,
        resolver: Self::Resolver,
        out: *mut Self::Archived,
    ) {
        ArchivedVec::resolve_from_len(field.len(), pos, resolver, out);
    }
}

impl<K, V, S> SerializeWith<HashMap<K, V>, S> for AsVec
where
    K: Serialize<S>,
    V: Serialize<S>,
    S: ScratchSpace + Serializer + ?Sized,
{
    fn serialize_with(
        field: &HashMap<K, V>,
        serializer: &mut S,
    ) -> Result<Self::Resolver, S::Error> {
        ArchivedVec::serialize_from_iter(
            field.iter().map(|(key, value)| Entry { key, value }),
            serializer,
        )
    }
}

impl<K, V, D> DeserializeWith<ArchivedVec<Entry<K::Archived, V::Archived>>, HashMap<K, V>, D>
    for AsVec
where
    K: Archive + Hash + Eq,
    V: Archive,
    K::Archived: Deserialize<K, D>,
    V::Archived: Deserialize<V, D>,
    D: Fallible + ?Sized,
{
    fn deserialize_with(
        field: &ArchivedVec<Entry<K::Archived, V::Archived>>,
        deserializer: &mut D,
    ) -> Result<HashMap<K, V>, D::Error> {
        let mut result = HashMap::with_capacity(field.len());
        for entry in field.iter() {
            result.insert(
                entry.key.deserialize(deserializer)?,
                entry.value.deserialize(deserializer)?,
            );
        }
        Ok(result)
    }
}

impl<T: Archive> ArchiveWith<HashSet<T>> for AsVec {
    type Archived = ArchivedVec<T::Archived>;
    type Resolver = VecResolver;

    unsafe fn resolve_with(
        field: &HashSet<T>,
        pos: usize,
        resolver: Self::Resolver,
        out: *mut Self::Archived,
    ) {
        ArchivedVec::resolve_from_len(field.len(), pos, resolver, out);
    }
}

impl<T, S> SerializeWith<HashSet<T>, S> for AsVec
where
    T: Serialize<S>,
    S: ScratchSpace + Serializer + ?Sized,
{
    fn serialize_with(field: &HashSet<T>, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        ArchivedVec::<T::Archived>::serialize_from_iter::<T, _, _, _>(field.iter(), serializer)
    }
}

impl<T, D> DeserializeWith<ArchivedVec<T::Archived>, HashSet<T>, D> for AsVec
where
    T: Archive + Hash + Eq,
    T::Archived: Deserialize<T, D>,
    D: Fallible + ?Sized,
{
    fn deserialize_with(
        field: &ArchivedVec<T::Archived>,
        deserializer: &mut D,
    ) -> Result<HashSet<T>, D::Error> {
        let mut result = HashSet::with_capacity(field.len());
        for key in field.iter() {
            result.insert(key.deserialize(deserializer)?);
        }
        Ok(result)
    }
}

// UnixTimestamp

impl ArchiveWith<SystemTime> for UnixTimestamp {
    type Archived = ArchivedDuration;
    type Resolver = ();

    #[inline]
    unsafe fn resolve_with(
        field: &SystemTime,
        pos: usize,
        resolver: Self::Resolver,
        out: *mut Self::Archived,
    ) {
        // We already checked the duration during serialize_with
        let duration = field.duration_since(UNIX_EPOCH).unwrap();
        Archive::resolve(&duration, pos, resolver, out);
    }
}

impl<S: Fallible + ?Sized> SerializeWith<SystemTime, S> for UnixTimestamp
where
    S::Error: From<UnixTimestampError>,
{
    fn serialize_with(field: &SystemTime, _: &mut S) -> Result<Self::Resolver, S::Error> {
        field
            .duration_since(UNIX_EPOCH)
            .map_err(|_| UnixTimestampError::TimeBeforeUnixEpoch)?;
        Ok(())
    }
}

impl<D: Fallible + ?Sized> DeserializeWith<ArchivedDuration, SystemTime, D> for UnixTimestamp {
    fn deserialize_with(field: &ArchivedDuration, _: &mut D) -> Result<SystemTime, D::Error> {
        Ok(UNIX_EPOCH + (*field).into())
    }
}
