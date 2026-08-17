use super::impls::macros::foreach_map;
use crate::prelude::*;
#[cfg(feature = "hashbrown_0_14")]
use hashbrown_0_14::HashMap as HashbrownMap014;
#[cfg(feature = "hashbrown_0_15")]
use hashbrown_0_15::HashMap as HashbrownMap015;
#[cfg(feature = "hashbrown_0_16")]
use hashbrown_0_16::HashMap as HashbrownMap016;
#[cfg(feature = "hashbrown_0_17")]
use hashbrown_0_17::HashMap as HashbrownMap017;
#[cfg(feature = "indexmap_1")]
use indexmap_1::IndexMap;
#[cfg(feature = "indexmap_2")]
use indexmap_2::IndexMap as IndexMap2;

enum GoodOrError<T, TAs, I> {
    Good(T),
    // Only here to consume the TAs,I generics
    Error(PhantomData<(TAs, I)>),
}

impl<'de, T, TAs, I> Deserialize<'de> for GoodOrError<T, TAs, I>
where
    TAs: DeserializeAs<'de, T>,
    I: InspectError,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let is_hr = deserializer.is_human_readable();
        let content: content::de::Content<'de> = Deserialize::deserialize(deserializer)?;

        Ok(
            match <DeserializeAsWrap<T, TAs>>::deserialize(content::de::ContentDeserializer::<
                D::Error,
            >::new(content, is_hr))
            {
                Ok(elem) => GoodOrError::Good(elem.into_inner()),
                Err(e) => {
                    I::inspect_error(e);
                    GoodOrError::Error(PhantomData)
                }
            },
        )
    }
}

impl<'de, T, U, I> DeserializeAs<'de, Vec<T>> for VecSkipError<U, I>
where
    U: DeserializeAs<'de, T>,
    I: InspectError,
{
    fn deserialize_as<D>(deserializer: D) -> Result<Vec<T>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SeqVisitor<T, U, I>(PhantomData<(T, U, I)>);

        impl<'de, T, TAs, I> Visitor<'de> for SeqVisitor<T, TAs, I>
        where
            I: InspectError,
            TAs: DeserializeAs<'de, T>,
        {
            type Value = Vec<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a sequence")
            }

            fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                utils::SeqIter::new(seq)
                    .filter_map(|res: Result<GoodOrError<T, TAs, I>, A::Error>| match res {
                        Ok(GoodOrError::Good(value)) => Some(Ok(value)),
                        Ok(GoodOrError::Error(_)) => None,
                        Err(err) => Some(Err(err)),
                    })
                    .collect()
            }
        }

        let visitor = SeqVisitor::<T, U, I>(PhantomData);
        deserializer.deserialize_seq(visitor)
    }
}

struct MapSkipErrorVisitor<MAP, K, KAs, V, VAs, I>(PhantomData<(MAP, K, KAs, V, VAs, I)>);

impl<'de, MAP, K, KAs, V, VAs, I> Visitor<'de> for MapSkipErrorVisitor<MAP, K, KAs, V, VAs, I>
where
    MAP: FromIterator<(K, V)>,
    KAs: DeserializeAs<'de, K>,
    VAs: DeserializeAs<'de, V>,
    I: InspectError,
{
    type Value = MAP;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a map")
    }

    #[inline]
    fn visit_map<A>(self, access: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        type KVPair<K, KAs, V, VAs, I> = (GoodOrError<K, KAs, I>, GoodOrError<V, VAs, I>);
        utils::MapIter::new(access)
            .filter_map(
                |res: Result<KVPair<K, KAs, V, VAs, I>, A::Error>| match res {
                    Ok((GoodOrError::Good(key), GoodOrError::Good(value))) => {
                        Some(Ok((key, value)))
                    }
                    Ok(_) => None,
                    Err(err) => Some(Err(err)),
                },
            )
            .collect()
    }
}

#[cfg(feature = "alloc")]
macro_rules! map_impl {
    (
        $ty:ident < K $(: $kbound1:ident $(+ $kbound2:ident)*)?, V $(, $typaram:ident : $bound1:ident $(+ $bound2:ident)*)* >,
        $with_capacity:expr
    ) => {
        impl<'de, K, V, KAs, VAs, I $(, $typaram)*> DeserializeAs<'de, $ty<K, V $(, $typaram)*>>
            for MapSkipError<KAs, VAs, I>
        where
            KAs: DeserializeAs<'de, K>,
            VAs: DeserializeAs<'de, V>,
            I: InspectError,
            $(K: $kbound1 $(+ $kbound2)*,)?
            $($typaram: $bound1 $(+ $bound2)*),*
        {
            fn deserialize_as<D>(deserializer: D) -> Result<$ty<K, V $(, $typaram)*>, D::Error>
            where
                D: Deserializer<'de>,
            {
                deserializer.deserialize_map(MapSkipErrorVisitor::<
                    $ty<K, V $(, $typaram)*>,
                    K,
                    KAs,
                    V,
                    VAs,
                    I,
                >(PhantomData))
            }
        }
    };
}
foreach_map!(map_impl);
