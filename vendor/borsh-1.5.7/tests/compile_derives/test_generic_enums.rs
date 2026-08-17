use borsh::{BorshDeserialize, BorshSerialize};

#[allow(unused)]
use alloc::{string::String, vec::Vec};

#[cfg(feature = "hashbrown")]
use hashbrown::HashMap;

#[cfg(hash_collections)]
use core::{cmp::Eq, hash::Hash};

#[cfg(feature = "std")]
use std::collections::HashMap;

use alloc::collections::BTreeMap;

/// `T: Ord` bound is required for `BorshDeserialize` derive to be successful
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
enum E<T: Ord, U, W> {
    X { f: BTreeMap<T, U> },
    Y(W),
}

#[cfg(hash_collections)]
#[derive(BorshSerialize, BorshDeserialize, Debug)]
enum I1<K, V, R> {
    B {
        #[allow(unused)]
        #[borsh(skip)]
        x: HashMap<K, V>,
        y: String,
    },
    C(K, Vec<R>),
}

#[cfg(hash_collections)]
#[derive(BorshSerialize, BorshDeserialize, Debug)]
enum I2<K: Ord + Eq + Hash, R, U> {
    B { x: HashMap<K, R>, y: String },
    C(K, #[borsh(skip)] U),
}
