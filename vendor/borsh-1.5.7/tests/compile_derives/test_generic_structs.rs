#[cfg(feature = "hashbrown")]
use hashbrown::HashMap;

#[cfg(hash_collections)]
use core::{cmp::Eq, hash::Hash};

#[cfg(feature = "std")]
use std::collections::HashMap;

use alloc::{
    collections::BTreeMap,
    string::String,
};

use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct TupleA<W>(W, u32);

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct NamedA<W> {
    a: W,
    b: u32,
}

/// `T: PartialOrd` is injected here via field bound to avoid having this restriction on
/// the struct itself
#[cfg(hash_collections)]
#[derive(BorshSerialize)]
struct C1<T, U> {
    a: String,
    #[borsh(bound(serialize = "T: borsh::ser::BorshSerialize + Ord,
         U: borsh::ser::BorshSerialize"))]
    b: HashMap<T, U>,
}

/// `T: PartialOrd + Hash + Eq` is injected here via field bound to avoid having this restriction on
/// the struct itself
#[allow(unused)]
#[cfg(hash_collections)]
#[derive(BorshDeserialize)]
struct C2<T, U> {
    a: String,
    #[borsh(bound(deserialize = "T: Ord + Hash + Eq + borsh::de::BorshDeserialize,
         U: borsh::de::BorshDeserialize"))]
    b: HashMap<T, U>,
}

/// `T: Ord` bound is required for `BorshDeserialize` derive to be successful
#[derive(BorshSerialize, BorshDeserialize)]
struct D<T: Ord, R> {
    a: String,
    b: BTreeMap<T, R>,
}

#[cfg(hash_collections)]
#[derive(BorshSerialize)]
struct G<K, V, U>(#[borsh(skip)] HashMap<K, V>, U);

#[cfg(hash_collections)]
#[derive(BorshDeserialize)]
struct G1<K, V, U>(#[borsh(skip)] HashMap<K, V>, U);

#[cfg(hash_collections)]
#[derive(BorshDeserialize)]
struct G2<K: Ord + Hash + Eq, R, U>(HashMap<K, R>, #[borsh(skip)] U);

/// implicit derived `core::default::Default` bounds on `K` and `V` are dropped by empty bound
/// specified, as `HashMap` hash its own `Default` implementation
#[cfg(hash_collections)]
#[derive(BorshDeserialize)]
struct G3<K, V, U>(#[borsh(skip, bound(deserialize = ""))] HashMap<K, V>, U);

#[cfg(hash_collections)]
#[derive(BorshSerialize, BorshDeserialize)]
struct H<K: Ord, V, U> {
    x: BTreeMap<K, V>,
    #[allow(unused)]
    #[borsh(skip)]
    y: U,
}

trait TraitName {
    type Associated;
    fn method(&self);
}


#[allow(unused)]
#[derive(BorshSerialize)]
struct ParametrizedWrongDerive<T, V>
where
    T: TraitName,
{
    #[borsh(bound(serialize = "<T as TraitName>::Associated: borsh::ser::BorshSerialize"))]
    field: <T as TraitName>::Associated,
    another: V,
}
