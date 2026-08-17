//! The Rust standard collections
//!
//! This library provides efficient implementations of the most common general purpose programming
//! data structures.

pub mod binary_heap;
pub mod btree_map;
pub mod btree_set;
pub mod hash_map;
pub mod hash_set;
pub mod linked_list;
pub mod vec_deque;

#[allow(unused)]
pub use binary_heap::BinaryHeap;
#[allow(unused)]
pub use btree_map::BTreeMap;
#[allow(unused)]
pub use btree_set::BTreeSet;
#[allow(unused)]
pub use hash_map::HashMap;
#[allow(unused)]
pub use hash_set::HashSet;
#[allow(unused)]
pub use linked_list::LinkedList;
#[allow(unused)]
pub use vec_deque::VecDeque;
