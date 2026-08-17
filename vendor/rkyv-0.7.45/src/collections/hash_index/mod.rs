//! A helper type that archives index data for hashed collections using
//! [compress, hash and displace](http://cmph.sourceforge.net/papers/esa09.pdf).

use crate::{Archive, Archived, RelPtr};
use core::{
    fmt,
    hash::{Hash, Hasher},
    slice,
};

/// The hash builder for archived hash indexes.
pub use seahash::SeaHasher as HashBuilder;

#[cfg(feature = "validation")]
pub mod validation;

/// An archived hash index.
#[cfg_attr(feature = "strict", repr(C))]
pub struct ArchivedHashIndex {
    len: Archived<usize>,
    displace: RelPtr<Archived<u32>>,
}

impl ArchivedHashIndex {
    /// Gets the number of items in the hash index.
    #[inline]
    pub const fn len(&self) -> usize {
        from_archived!(self.len) as usize
    }

    #[inline]
    fn make_hasher() -> HashBuilder {
        HashBuilder::with_seeds(
            0x08576fb6170b5f5f,
            0x587775eeb84a7e46,
            0xac701115428ee569,
            0x910feb91b92bb1cd,
        )
    }

    /// Gets the hasher for this hash index. The hasher for all archived hash indexes is the same
    /// for reproducibility.
    #[inline]
    pub fn hasher(&self) -> HashBuilder {
        Self::make_hasher()
    }

    #[inline]
    fn displace_slice(&self) -> &[Archived<u32>] {
        unsafe { slice::from_raw_parts(self.displace.as_ptr(), self.len()) }
    }

    #[inline]
    fn displace(&self, index: usize) -> u32 {
        from_archived!(self.displace_slice()[index])
    }

    /// Returns the index where a key may be located in the hash index.
    ///
    /// The hash index does not have access to the keys used to build it, so the key at the returned
    /// index must be checked for equality.
    #[inline]
    pub fn index<K: Hash + ?Sized>(&self, k: &K) -> Option<usize> {
        if self.is_empty() {
            return None;
        }
        let mut hasher = self.hasher();
        k.hash(&mut hasher);
        let displace_index = hasher.finish() % self.len() as u64;
        let displace = self.displace(displace_index as usize);

        if displace == u32::MAX {
            None
        } else if displace & 0x80_00_00_00 == 0 {
            Some(displace as usize)
        } else {
            let mut hasher = self.hasher();
            displace.hash(&mut hasher);
            k.hash(&mut hasher);
            let index = hasher.finish() % self.len() as u64;
            Some(index as usize)
        }
    }

    /// Returns whether there are no items in the hash index.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Resolves an archived hash index from a given length and parameters.
    ///
    /// # Safety
    ///
    /// - `len` must be the number of elements in the hash index
    /// - `pos` must be the position of `out` within the archive
    /// - `resolver` must be the result of building and serializing a hash index
    #[inline]
    pub unsafe fn resolve_from_len(
        len: usize,
        pos: usize,
        resolver: HashIndexResolver,
        out: *mut Self,
    ) {
        let (fp, fo) = out_field!(out.len);
        len.resolve(pos + fp, (), fo);

        let (fp, fo) = out_field!(out.displace);
        RelPtr::emplace(pos + fp, resolver.displace_pos, fo);
    }
}

#[cfg(feature = "alloc")]
const _: () = {
    use crate::{
        ser::{ScratchSpace, Serializer},
        ScratchVec,
    };
    #[cfg(not(feature = "std"))]
    use alloc::vec::Vec;
    use core::{
        cmp::Reverse,
        mem::{size_of, MaybeUninit},
    };

    impl ArchivedHashIndex {
        /// Builds and serializes a hash index from an iterator of key-value pairs.
        ///
        /// # Safety
        ///
        /// - The keys returned by the iterator must be unique.
        /// - `entries` must have a capacity of `iter.len()` entries.
        #[allow(clippy::type_complexity)]
        pub unsafe fn build_and_serialize<'a, K, V, S, I>(
            iter: I,
            serializer: &mut S,
            entries: &mut ScratchVec<MaybeUninit<(&'a K, &'a V)>>,
        ) -> Result<HashIndexResolver, S::Error>
        where
            K: 'a + Hash,
            V: 'a,
            S: Serializer + ScratchSpace + ?Sized,
            I: ExactSizeIterator<Item = (&'a K, &'a V)>,
        {
            let len = iter.len();

            let mut bucket_size = ScratchVec::new(serializer, len)?;
            for _ in 0..len {
                bucket_size.push(0u32);
            }

            let mut displaces = ScratchVec::new(serializer, len)?;

            for (key, value) in iter {
                let mut hasher = Self::make_hasher();
                key.hash(&mut hasher);
                let displace = (hasher.finish() % len as u64) as u32;
                displaces.push((displace, (key, value)));
                bucket_size[displace as usize] += 1;
            }

            displaces
                .sort_by_key(|&(displace, _)| (Reverse(bucket_size[displace as usize]), displace));

            let mut occupied = ScratchVec::new(serializer, len)?;
            for _ in 0..len {
                occupied.push(false);
            }

            let mut displacements = ScratchVec::new(serializer, len)?;
            for _ in 0..len {
                displacements.push(to_archived!(u32::MAX));
            }

            let mut first_empty = 0;
            let mut assignments = Vec::with_capacity(8);

            let mut start = 0;
            while start < displaces.len() {
                let displace = displaces[start].0;
                let bucket_size = bucket_size[displace as usize] as usize;
                let end = start + bucket_size;
                let bucket = &displaces[start..end];
                start = end;

                if bucket_size > 1 {
                    'find_seed: for seed in 0x80_00_00_00u32..=0xFF_FF_FF_FFu32 {
                        let mut base_hasher = Self::make_hasher();
                        seed.hash(&mut base_hasher);

                        assignments.clear();

                        for &(_, (key, _)) in bucket.iter() {
                            let mut hasher = base_hasher;
                            key.hash(&mut hasher);
                            let index = (hasher.finish() % len as u64) as u32;
                            if occupied[index as usize] || assignments.contains(&index) {
                                continue 'find_seed;
                            } else {
                                assignments.push(index);
                            }
                        }

                        for i in 0..bucket_size {
                            occupied[assignments[i] as usize] = true;
                            entries[assignments[i] as usize]
                                .as_mut_ptr()
                                .write(bucket[i].1);
                        }
                        displacements[displace as usize] = to_archived!(seed);
                        break;
                    }
                } else {
                    let offset = occupied[first_empty..]
                        .iter()
                        .position(|value| !value)
                        .unwrap();
                    first_empty += offset;
                    occupied[first_empty] = true;
                    entries[first_empty].as_mut_ptr().write(bucket[0].1);
                    displacements[displace as usize] = to_archived!(first_empty as u32);
                    first_empty += 1;
                }
            }

            // Write displacements
            let displace_pos = serializer.align_for::<Archived<u32>>()?;
            let displacements_slice = slice::from_raw_parts(
                displacements.as_ptr().cast::<u8>(),
                len * size_of::<Archived<u32>>(),
            );
            serializer.write(displacements_slice)?;

            // Free scratch vecs
            displacements.free(serializer)?;
            occupied.free(serializer)?;
            displaces.free(serializer)?;
            bucket_size.free(serializer)?;

            Ok(HashIndexResolver { displace_pos })
        }
    }
};

impl fmt::Debug for ArchivedHashIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.displace_slice()).finish()
    }
}

/// The resolver for an archived hash index.
pub struct HashIndexResolver {
    displace_pos: usize,
}
