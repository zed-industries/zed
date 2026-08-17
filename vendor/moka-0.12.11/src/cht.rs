//! Lock-free hash tables.
//!
//! The hash tables in this crate are, at their core, open addressing hash
//! tables implemented using open addressing and boxed buckets. The core of
//! these hash tables are bucket arrays, which consist of a vector of atomic
//! pointers to buckets, an atomic pointer to the next bucket array, and an
//! epoch number. In the context of this crate, an atomic pointer is a nullable
//! pointer that is accessed and manipulated using atomic memory operations.
//! Each bucket consists of a key and a possibly-uninitialized value.
//!
//! The key insight into making the hash table resizable is to incrementally
//! copy buckets from the old bucket array to the new bucket array. As buckets
//! are copied between bucket arrays, their pointers in the old bucket array are
//! CAS'd with a null pointer that has a sentinel bit set. If the CAS fails,
//! that thread must read the bucket pointer again and retry copying it into the
//! new bucket array. If at any time a thread reads a bucket pointer with the
//! sentinel bit set, that thread knows that a new (larger) bucket array has
//! been allocated. That thread will then immediately attempt to copy all
//! buckets to the new bucket array. It is possible to implement an algorithm in
//! which a subset of buckets are relocated per-thread; such an algorithm has
//! not been implemented for the sake of simplicity.
//!
//! Bucket pointers that have been copied from an old bucket array into a new
//! bucket array are marked with a borrowed bit. If a thread copies a bucket
//! from an old bucket array into a new bucket array, fails to CAS the bucket
//! pointer in the old bucket array, it attempts to CAS the bucket pointer in
//! the new bucket array that it previously inserted to. If the bucket pointer
//! in the new bucket array does *not* have the borrowed tag bit set, that
//! thread knows that the value in the new bucket array was modified more
//! recently than the value in the old bucket array. To avoid discarding updates
//! to the new bucket array, a thread will never replace a bucket pointer that
//! has the borrowed tag bit set with one that does not. To see why this is
//! necessary, consider the case where a bucket pointer is copied into the new
//! array, removed from the new array by a second thread, then copied into the
//! new array again by a third thread.
//!
//! Mutating operations are, at their core, an atomic compare-and-swap (CAS) on
//! a bucket pointer. Insertions CAS null pointers and bucket pointers with
//! matching keys, modifications CAS bucket pointers with matching keys, and
//! removals CAS non-tombstone bucket pointers. Tombstone bucket pointers are
//! bucket pointers with a tombstone bit set as part of a removal; this
//! indicates that the bucket's value has been moved from and will be destroyed
//! if it has not been already.
//!
//! As previously mentioned, removing an entry from the hash table results in
//! that bucket pointer having a tombstone bit set. Insertions cannot
//! displace a tombstone bucket unless their key compares equal, so once an
//! entry is inserted into the hash table, the specific index it is assigned to
//! will only ever hold entries whose keys compare equal. Without this
//! restriction, resizing operations could result in the old and new bucket
//! arrays being temporarily inconsistent. Consider the case where one thread,
//! as part of a resizing operation, copies a bucket into a new bucket array
//! while another thread removes and replaces that bucket from the old bucket
//! array. If the new bucket has a non-matching key, what happens to the bucket
//! that was just copied into the new bucket array?
//!
//! Tombstone bucket pointers are typically not copied into new bucket arrays.
//! The exception is the case where a bucket pointer was copied to the new
//! bucket array, then CAS on the old bucket array fails because that bucket has
//! been replaced with a tombstone. In this case, the tombstone bucket pointer
//! will be copied over to reflect the update without displacing a key from its
//! bucket.
//!
//! This hash table algorithm was inspired by [a blog post by Jeff Phreshing]
//! that describes the implementation of the Linear hash table in [Junction], a
//! C++ library of concurrent data structures. Additional inspiration was drawn
//! from the lock-free hash table described by Cliff Click in [a tech talk] given
//! at Google in 2007.
//!
//! [a blog post by Jeff Phreshing]: https://preshing.com/20160222/a-resizable-concurrent-map/
//! [Junction]: https://github.com/preshing/junction
//! [a tech talk]: https://youtu.be/HJ-719EGIts

pub(crate) mod iter;
pub(crate) mod map;
pub(crate) mod segment;

#[cfg(test)]
#[macro_use]
pub(crate) mod test_util;

pub(crate) use segment::HashMap as SegmentedHashMap;
