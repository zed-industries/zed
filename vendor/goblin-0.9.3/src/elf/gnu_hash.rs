//! A Gnu Hash table as 4 sections:
//!
//!  1. Header
//!  2. Bloom Filter
//!  3. Hash Buckets
//!  4. Chains
//!
//! The header has is an array of four `u32`s:
//!
//!  1. nbuckets
//!  2. symndx
//!  3. maskwords
//!  4. shift2
//!
//! See more:
//!  * <http://www.linker-aliens.org/blogs/ali/entry/gnu_hash_elf_sections>
//!    or <https://blogs.oracle.com/solaris/gnu-hash-elf-sections-v2>
//!  * <https://flapenguin.me/2017/05/10/elf-lookup-dt-gnu-hash/>

/// GNU hash function: accepts a symbol name and returns a value that may be
/// used to compute a bucket index.
///
/// Consequently, if the hashing function returns the value `x` for some name,
/// `buckets[x % nbuckets]` gives an index, `y`, into both the symbol table
/// and the chain table.
pub fn hash(symbol: &str) -> u32 {
    const HASH_SEED: u32 = 5381;
    symbol.bytes().fold(HASH_SEED, |hash, b| {
        hash.wrapping_mul(33).wrapping_add(u32::from(b))
    })
}

#[cfg(test)]
mod tests {
    use super::hash;
    #[test]
    fn test_hash() {
        assert_eq!(hash(""), 0x0000_1505);
        assert_eq!(hash("printf"), 0x156b_2bb8);
        assert_eq!(hash("exit"), 0x7c96_7e3f);
        assert_eq!(hash("syscall"), 0xbac2_12a0);
        assert_eq!(hash("flapenguin.me"), 0x8ae9_f18e);
    }
}

macro_rules! elf_gnu_hash_impl {
    ($IntTy:ty) => {
        use crate::elf::sym::Sym;
        use crate::strtab::Strtab;
        use core::fmt;
        use core::mem;
        use core::slice;

        const INT_SIZE: usize = mem::size_of::<$IntTy>();
        const U32_SIZE: usize = mem::size_of::<u32>();
        /// Size of a bits mask in bloom filter
        const ELFCLASS_BITS: u32 = INT_SIZE as u32 * 8;

        /// A better hash table for the ELF used by GNU systems in GNU-compatible software.
        pub struct GnuHash<'a> {
            /// Index of the first symbol in the `.dynsym` table which is accessible with
            /// the hash table
            symindex: u32,
            /// Shift count used in the bloom filter
            shift2: u32,
            /// 2 bit bloom filter on `chains`
            // Either 32 or 64-bit depending on the class of object
            bloom_filter: &'a [$IntTy],
            /// GNU hash table bucket array; indexes start at 0. This array holds symbol
            /// table indexes and contains the index of hashes in `chains`
            buckets: &'a [u32],
            /// Hash values; indexes start at 0. This array holds symbol table indexes.
            chains: &'a [u32], // => chains[dynsyms.len() - symindex]
            dynsyms: &'a [Sym],
        }

        impl fmt::Debug for GnuHash<'_> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.debug_struct("GnuHash")
                    .field("nbuckets", &self.buckets.len())
                    .field("symindex", &self.symindex)
                    .field("maskwords", &(self.bloom_filter.len() - 1))
                    .field("shift2", &self.shift2)
                    .field("bloom_filter", &self.bloom_filter.as_ptr())
                    .field("bucket", &self.buckets.as_ptr())
                    .field("chains", &self.chains.as_ptr())
                    .finish()
            }
        }

        impl<'a> GnuHash<'a> {
            /// Initialize a GnuHash from a pointer to `.hash` (or `.gnu.hash`) section
            /// and total number of dynamic symbols.
            /// # Safety
            ///
            /// This function creates a `GnuHash` directly from a raw pointer
            pub unsafe fn from_raw_table(
                hashtab: &'a [u8],
                dynsyms: &'a [Sym],
            ) -> Result<Self, &'static str> {
                if hashtab.as_ptr() as usize % INT_SIZE != 0 {
                    return Err("hashtab is not aligned with 64-bit");
                }

                if hashtab.len() <= 16 {
                    return Err("failed to read in number of buckets");
                }

                let [nbuckets, symindex, maskwords, shift2] =
                    (hashtab.as_ptr() as *const u32 as *const [u32; 4]).read();

                if !maskwords.is_power_of_two() {
                    return Err("maskwords must be a power of two");
                }

                let hashtab = &hashtab[16..];
                {
                    // SAFETY: Condition to check for an overflow
                    //   size_of(chains) + size_of(buckets) + size_of(bloom_filter) == size_of(hashtab)

                    if dynsyms.len() <= symindex as usize {
                        return Err("symindex must be smaller than dynsyms.len()");
                    }
                    let chains_size = (dynsyms.len() - symindex as usize).checked_mul(U32_SIZE);
                    let buckets_size = (nbuckets as usize).checked_mul(U32_SIZE);
                    let bloom_size = (maskwords as usize).checked_mul(INT_SIZE);

                    let total_size = match (chains_size, buckets_size, bloom_size) {
                        (Some(a), Some(b), Some(c)) => {
                            a.checked_add(b).and_then(|t| t.checked_add(c))
                        }
                        _ => None,
                    };
                    match total_size {
                        Some(size) if size == hashtab.len() => {}
                        _ => return Err("index out of bound or non-complete hash section"),
                    }
                }

                let bloom_filter_ptr = hashtab.as_ptr() as *const $IntTy;
                let buckets_ptr = bloom_filter_ptr.add(maskwords as usize) as *const u32;
                let chains_ptr = buckets_ptr.add(nbuckets as usize);
                let bloom_filter = slice::from_raw_parts(bloom_filter_ptr, maskwords as usize);
                let buckets = slice::from_raw_parts(buckets_ptr, nbuckets as usize);
                let chains = slice::from_raw_parts(chains_ptr, dynsyms.len() - symindex as usize);
                Ok(Self {
                    symindex,
                    shift2,
                    bloom_filter,
                    buckets,
                    chains,
                    dynsyms,
                })
            }

            /// Locate the hash chain, and corresponding hash value element.
            #[cold]
            fn lookup(&self, symbol: &str, hash: u32, dynstrtab: &Strtab) -> Option<&'a Sym> {
                const MASK_LOWEST_BIT: u32 = 0xffff_fffe;
                let bucket = self.buckets[hash as usize % self.buckets.len()];

                // Empty hash chain, symbol not present
                if bucket < self.symindex {
                    return None;
                }
                // Walk the chain until the symbol is found or the chain is exhausted.
                let chain_idx = bucket - self.symindex;
                let hash = hash & MASK_LOWEST_BIT;
                let chains = &self.chains.get((chain_idx as usize)..)?;
                let dynsyms = &self.dynsyms.get((bucket as usize)..)?;
                for (hash2, symb) in chains.iter().zip(dynsyms.iter()) {
                    if (hash == (hash2 & MASK_LOWEST_BIT))
                        && (symbol == &dynstrtab[symb.st_name as usize])
                    {
                        return Some(symb);
                    }
                    // Chain ends with an element with the lowest bit set to 1.
                    if hash2 & 1 == 1 {
                        break;
                    }
                }
                None
            }

            /// Check if symbol maybe is in the hash table, or definitely not in it.
            #[inline]
            fn check_maybe_match(&self, hash: u32) -> bool {
                const MASK: u32 = ELFCLASS_BITS - 1;
                let hash2 = hash >> self.shift2;
                // `x & (N - 1)` is equivalent to `x % N` iff `N = 2^y`.
                let bitmask: $IntTy = 1 << (hash & (MASK)) | 1 << (hash2 & MASK);
                let bloom_idx = (hash / ELFCLASS_BITS) & (self.bloom_filter.len() as u32 - 1);
                let bitmask_word = self.bloom_filter[bloom_idx as usize];
                (bitmask_word & bitmask) == bitmask
            }

            /// Given a symbol, a hash of that symbol, a dynamic string table and
            /// a `dynstrtab` to cross-reference names, maybe returns a Sym.
            pub fn find(&self, symbol: &str, dynstrtab: &Strtab) -> Option<&'a Sym> {
                let hash = self::hash(symbol);
                self.find_with_hash(symbol, hash, dynstrtab)
            }

            /// This function will not check if the passed `hash` is really
            /// the hash of `symbol`
            pub fn find_with_hash(
                &self,
                symbol: &str,
                hash: u32,
                dynstrtab: &Strtab,
            ) -> Option<&'a Sym> {
                if self.check_maybe_match(hash) {
                    self.lookup(symbol, hash, dynstrtab)
                } else {
                    None
                }
            }
        }
    };
}
