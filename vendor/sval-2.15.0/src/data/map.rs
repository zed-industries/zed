use crate::{std::fmt, Result, Stream, Value};

/**
An adapter that streams a slice of key-value pairs as a map.
 */
#[repr(transparent)]
pub struct MapSlice<K, V>([(K, V)]);

impl<K, V> MapSlice<K, V> {
    /**
    Treat a slice of key-value pairs as a map.
     */
    pub const fn new<'a>(map: &'a [(K, V)]) -> &'a Self {
        // SAFETY: `MapSlice` and `[(K, V)]` have the same ABI
        unsafe { &*(map as *const _ as *const MapSlice<K, V>) }
    }

    /**
    Get a reference to the underlying slice.
     */
    pub const fn as_slice(&self) -> &[(K, V)] {
        &self.0
    }
}

impl<K: fmt::Debug, V: fmt::Debug> fmt::Debug for MapSlice<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl<K, V> AsRef<[(K, V)]> for MapSlice<K, V> {
    fn as_ref(&self) -> &[(K, V)] {
        &self.0
    }
}

impl<K: Value, V: Value> Value for MapSlice<K, V> {
    fn stream<'sval, S: Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> Result {
        stream.map_begin(Some(self.0.len()))?;

        for (k, v) in self.0.iter() {
            stream.map_key_begin()?;
            stream.value(k)?;
            stream.map_key_end()?;

            stream.map_value_begin()?;
            stream.value(v)?;
            stream.map_value_end()?;
        }

        stream.map_end()
    }
}

#[cfg(feature = "alloc")]
mod alloc_support {
    use super::*;
    use crate::std::collections::BTreeMap;

    impl<K: Value, V: Value> Value for BTreeMap<K, V> {
        fn stream<'sval, S: Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> Result {
            stream.map_begin(Some(self.len()))?;

            for (k, v) in self {
                stream.map_key_begin()?;
                stream.value(k)?;
                stream.map_key_end()?;

                stream.map_value_begin()?;
                stream.value(v)?;
                stream.map_value_end()?;
            }

            stream.map_end()
        }
    }
}

#[cfg(feature = "std")]
mod std_support {
    use super::*;
    use crate::std::{collections::HashMap, hash::BuildHasher};

    impl<K: Value, V: Value, H: BuildHasher> Value for HashMap<K, V, H> {
        fn stream<'sval, S: Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> Result {
            stream.map_begin(Some(self.len()))?;

            for (k, v) in self {
                stream.map_key_begin()?;
                stream.value(k)?;
                stream.map_key_end()?;

                stream.map_value_begin()?;
                stream.value(v)?;
                stream.map_value_end()?;
            }

            stream.map_end()
        }
    }
}
