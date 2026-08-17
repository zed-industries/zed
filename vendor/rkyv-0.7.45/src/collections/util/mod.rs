//! Utilities for archived collections.

#[cfg(feature = "validation")]
pub mod validation;

use crate::{Archive, Fallible, Serialize};

/// A simple key-value pair.
///
/// This is typically used by associative containers that store keys and values together.
#[derive(Debug, Eq)]
#[cfg_attr(feature = "strict", repr(C))]
pub struct Entry<K, V> {
    /// The key of the pair.
    pub key: K,
    /// The value of the pair.
    pub value: V,
}

impl<K: Archive, V: Archive> Archive for Entry<&'_ K, &'_ V> {
    type Archived = Entry<K::Archived, V::Archived>;
    type Resolver = (K::Resolver, V::Resolver);

    #[inline]
    unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
        let (fp, fo) = out_field!(out.key);
        self.key.resolve(pos + fp, resolver.0, fo);

        let (fp, fo) = out_field!(out.value);
        self.value.resolve(pos + fp, resolver.1, fo);
    }
}

impl<K: Serialize<S>, V: Serialize<S>, S: Fallible + ?Sized> Serialize<S> for Entry<&'_ K, &'_ V> {
    #[inline]
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        Ok((
            self.key.serialize(serializer)?,
            self.value.serialize(serializer)?,
        ))
    }
}

impl<K, V, UK, UV> PartialEq<Entry<UK, UV>> for Entry<K, V>
where
    K: PartialEq<UK>,
    V: PartialEq<UV>,
{
    #[inline]
    fn eq(&self, other: &Entry<UK, UV>) -> bool {
        self.key.eq(&other.key) && self.value.eq(&other.value)
    }
}
