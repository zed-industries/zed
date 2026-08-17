pub mod clusterer_parallel;
pub mod clustering_serial;

use std::collections::HashSet;
use ahash::{AHasher, AHashSet};


pub trait QueryIndex {
    type Id: Sized;

    fn query(&self, id: &Self::Id) -> HashSet<&Self::Id>;
}
