use crate::{
    binary_heap::Kind as BinaryHeapKind, len_type::LenType, BinaryHeap, Deque, HistoryBuf,
    IndexMap, IndexSet, LinearMap, String, Vec,
};
use core::{
    fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
};
use hash32::BuildHasherDefault;
use serde_core::de::{self, Deserialize, Deserializer, Error, MapAccess, SeqAccess};

// Sequential containers

impl<'de, T, KIND, const N: usize> Deserialize<'de> for BinaryHeap<T, KIND, N>
where
    T: Ord + Deserialize<'de>,

    KIND: BinaryHeapKind,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ValueVisitor<'de, T, KIND, const N: usize>(PhantomData<(&'de (), T, KIND)>);

        impl<'de, T, KIND, const N: usize> de::Visitor<'de> for ValueVisitor<'de, T, KIND, N>
        where
            T: Ord + Deserialize<'de>,
            KIND: BinaryHeapKind,
        {
            type Value = BinaryHeap<T, KIND, N>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a sequence")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut values = BinaryHeap::new();

                while let Some(value) = seq.next_element()? {
                    if values.push(value).is_err() {
                        return Err(A::Error::invalid_length(values.capacity() + 1, &self))?;
                    }
                }

                Ok(values)
            }
        }
        deserializer.deserialize_seq(ValueVisitor(PhantomData))
    }
}

impl<'de, T, S, const N: usize> Deserialize<'de> for IndexSet<T, BuildHasherDefault<S>, N>
where
    T: Eq + Hash + Deserialize<'de>,
    S: Hasher + Default,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ValueVisitor<'de, T, S, const N: usize>(PhantomData<(&'de (), T, S)>);

        impl<'de, T, S, const N: usize> de::Visitor<'de> for ValueVisitor<'de, T, S, N>
        where
            T: Eq + Hash + Deserialize<'de>,
            S: Hasher + Default,
        {
            type Value = IndexSet<T, BuildHasherDefault<S>, N>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a sequence")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut values = IndexSet::new();

                while let Some(value) = seq.next_element()? {
                    if values.insert(value).is_err() {
                        return Err(A::Error::invalid_length(values.capacity() + 1, &self))?;
                    }
                }

                Ok(values)
            }
        }
        deserializer.deserialize_seq(ValueVisitor(PhantomData))
    }
}

impl<'de, T, LenT: LenType, const N: usize> Deserialize<'de> for Vec<T, N, LenT>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ValueVisitor<'de, T, LenT: LenType, const N: usize>(PhantomData<(&'de (), T, LenT)>);

        impl<'de, T, LenT, const N: usize> serde_core::de::Visitor<'de> for ValueVisitor<'de, T, LenT, N>
        where
            T: Deserialize<'de>,
            LenT: LenType,
        {
            type Value = Vec<T, N, LenT>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a sequence")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut values = Vec::new();

                while let Some(value) = seq.next_element()? {
                    if values.push(value).is_err() {
                        return Err(A::Error::invalid_length(values.capacity() + 1, &self))?;
                    }
                }

                Ok(values)
            }
        }
        deserializer.deserialize_seq(ValueVisitor(PhantomData))
    }
}

impl<'de, T, const N: usize> Deserialize<'de> for Deque<T, N>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ValueVisitor<'de, T, const N: usize>(PhantomData<(&'de (), T)>);

        impl<'de, T, const N: usize> serde_core::de::Visitor<'de> for ValueVisitor<'de, T, N>
        where
            T: Deserialize<'de>,
        {
            type Value = Deque<T, N>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a sequence")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut values = Deque::new();

                while let Some(value) = seq.next_element()? {
                    if values.push_back(value).is_err() {
                        return Err(A::Error::invalid_length(values.capacity() + 1, &self))?;
                    }
                }

                Ok(values)
            }
        }
        deserializer.deserialize_seq(ValueVisitor(PhantomData))
    }
}

impl<'de, T, const N: usize> Deserialize<'de> for HistoryBuf<T, N>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ValueVisitor<'de, T, const N: usize>(PhantomData<(&'de (), T)>);

        impl<'de, T, const N: usize> serde_core::de::Visitor<'de> for ValueVisitor<'de, T, N>
        where
            T: Deserialize<'de>,
        {
            type Value = HistoryBuf<T, N>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a sequence")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut values = HistoryBuf::new();

                while let Some(value) = seq.next_element()? {
                    values.write(value);
                }

                Ok(values)
            }
        }
        deserializer.deserialize_seq(ValueVisitor(PhantomData))
    }
}

// Dictionaries

impl<'de, K, V, S, const N: usize> Deserialize<'de> for IndexMap<K, V, BuildHasherDefault<S>, N>
where
    K: Eq + Hash + Deserialize<'de>,
    V: Deserialize<'de>,
    S: Default + Hasher,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ValueVisitor<'de, K, V, S, const N: usize>(PhantomData<(&'de (), K, V, S)>);

        impl<'de, K, V, S, const N: usize> de::Visitor<'de> for ValueVisitor<'de, K, V, S, N>
        where
            K: Eq + Hash + Deserialize<'de>,
            V: Deserialize<'de>,
            S: Default + Hasher,
        {
            type Value = IndexMap<K, V, BuildHasherDefault<S>, N>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a map")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut values = IndexMap::new();

                while let Some((key, value)) = map.next_entry()? {
                    if values.insert(key, value).is_err() {
                        return Err(A::Error::invalid_length(values.capacity() + 1, &self))?;
                    }
                }

                Ok(values)
            }
        }
        deserializer.deserialize_map(ValueVisitor(PhantomData))
    }
}

impl<'de, K, V, const N: usize> Deserialize<'de> for LinearMap<K, V, N>
where
    K: Eq + Deserialize<'de>,
    V: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ValueVisitor<'de, K, V, const N: usize>(PhantomData<(&'de (), K, V)>);

        impl<'de, K, V, const N: usize> de::Visitor<'de> for ValueVisitor<'de, K, V, N>
        where
            K: Eq + Deserialize<'de>,
            V: Deserialize<'de>,
        {
            type Value = LinearMap<K, V, N>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a map")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut values = LinearMap::new();

                while let Some((key, value)) = map.next_entry()? {
                    if values.insert(key, value).is_err() {
                        return Err(A::Error::invalid_length(values.capacity() + 1, &self))?;
                    }
                }

                Ok(values)
            }
        }
        deserializer.deserialize_map(ValueVisitor(PhantomData))
    }
}

// String containers

impl<'de, LenT: LenType, const N: usize> Deserialize<'de> for String<N, LenT> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ValueVisitor<'de, LenT: LenType, const N: usize>(PhantomData<(&'de (), LenT)>);

        impl<'de, LenT: LenType, const N: usize> de::Visitor<'de> for ValueVisitor<'de, LenT, N> {
            type Value = String<N, LenT>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(formatter, "a string no more than {} bytes long", N as u64)
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let mut s = String::new();
                s.push_str(v)
                    .map_err(|_| E::invalid_length(v.len(), &self))?;
                Ok(s)
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let mut s = String::new();

                s.push_str(
                    core::str::from_utf8(v)
                        .map_err(|_| E::invalid_value(de::Unexpected::Bytes(v), &self))?,
                )
                .map_err(|_| E::invalid_length(v.len(), &self))?;

                Ok(s)
            }
        }

        deserializer.deserialize_str(ValueVisitor(PhantomData))
    }
}
