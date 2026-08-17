use borsh::{BorshDeserialize, BorshSerialize};

#[cfg(feature = "hashbrown")]
use hashbrown::HashMap;

#[cfg(feature = "std")]
use std::collections::HashMap;

#[cfg(hash_collections)]
use core::{cmp::Eq, hash::Hash};
use alloc::{boxed::Box, string::String};


#[cfg(hash_collections)]
#[derive(BorshSerialize, BorshDeserialize)]
struct CRec<U: Ord + Hash + Eq> {
    a: String,
    b: HashMap<U, CRec<U>>,
}

//  `impl<T, U> BorshDeserialize for Box<T>` pulls in => `ToOwned`
// => pulls in at least `Clone`
#[derive(Clone, BorshSerialize, BorshDeserialize)]
struct CRecA {
    a: String,
    b: Box<CRecA>,
}


#[cfg(hash_collections)]
#[derive(BorshSerialize, BorshDeserialize)]
struct CRecC {
    a: String,
    b: HashMap<String, CRecC>,
}
