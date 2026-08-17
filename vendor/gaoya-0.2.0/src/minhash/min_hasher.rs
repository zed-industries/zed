use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::hash::{BuildHasher, BuildHasherDefault, Hash, Hasher};

use crate::minhash::{compute_minhash_similarity, MinHasher, MinHashType};
use crate::minhash::hashers::{SipHasher24BuildHasher};
use rand::distributions::{Distribution, Uniform};
use rand::seq::SliceRandom;
use rayon::prelude::*;
use std::cmp::min;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::marker::PhantomData;
use fnv::FnvBuildHasher;
use num_traits::{AsPrimitive, PrimInt};
use siphasher::sip::{SipHasher, SipHasher24};
use num_traits as num;



macro_rules! make_min_hasher {
    ($name: ident, $type: ident) => {
        pub struct $name<B: BuildHasher> {
            build_hasher: B,
            a: Vec<u32>,
            b: Vec<u32>,
            num_hashes: usize
        }

        impl $name<FnvBuildHasher> {
            pub fn new(num_hashes: usize) -> Self {
                return $name::new_with_hasher(num_hashes, FnvBuildHasher::default());
            }
        }

        impl<B: BuildHasher> $name<B> {
            pub fn new_with_hasher(num_hashes: usize, build_hasher: B) -> Self {
                Self::new_with_hasher_and_seed(num_hashes, build_hasher, 3)
            }

            pub fn new_with_hasher_and_seed(num_hashes: usize, build_hasher: B, seed: u64) -> Self {
                let mut rng = StdRng::seed_from_u64(seed);
                let rand_range1 = Uniform::from(1..MERSENNE_PRIME_31);
                let rand_range2 = Uniform::from(0..MERSENNE_PRIME_31);
                $name {
                    build_hasher,
                    a: (0..num_hashes)
                        .map(|_| rand_range1.sample(&mut rng))
                        .collect(),
                    b: (0..num_hashes)
                        .map(|_| rand_range2.sample(&mut rng))
                        .collect(),
                    num_hashes
                }
            }



            pub fn num_hashes(&self) -> usize {
                self.num_hashes
            }
        }


        impl<B: BuildHasher> MinHasher for $name<B> {
            type V = $type;
            fn create_signature<T, U>(&self, iter: T) -> Vec<Self::V>
                where
                    T: Iterator<Item = U>,
                    U: Hash
            {
                let hashes: Vec<u32> = iter
                    .map(|item| {
                        let mut hasher = self.build_hasher.build_hasher();
                        item.hash(&mut hasher);
                        hasher.finish() as u32
                    })
                    .collect::<Vec<_>>();

                match hashes.len() {
                    len if len > 0 => self
                        .a
                        .iter()
                        .zip(self.b.iter())
                        .map(|ab| {
                            let h = hashes
                                .iter()
                                .map(|hash| {
                                    hash.wrapping_mul(*ab.0).wrapping_add(*ab.1) % MERSENNE_PRIME_31
                                })
                                .min()
                                .unwrap();
                            h as $type
                        })
                        .collect(),
                    _ => vec![0; self.num_hashes],
                }
            }
        }
    }
}


const MERSENNE_PRIME_31: u32 = (1 << 31) - 1;



make_min_hasher!(MinHasher32, u32);
make_min_hasher!(MinHasher16, u16);
make_min_hasher!(MinHasher8, u8);



#[cfg(test)]
mod tests {
    use crate::minhash::{compute_jaccard_similarity, MinHasher};
    use crate::minhash::compute_minhash_similarity;
    use crate::text::whitespace_split;
    use std::cmp::min;
    use std::f64;
    use crate::minhash::min_hasher::MinHasher16;
    use crate::minhash::min_hasher::MinHasher8;
    use crate::minhash::min_hasher::MinHasher32;

    static S1: &'static str = "local sensitive hashing is cool";
    static S2: &'static str = "local sensitive hashing is great";
    static S3: &'static str = "local sensitive hashing is awesome";

    static S10: &'static str = "If you're still searching, we can visit a few open houses together in the next few weeks. It might help give clarity on what you're looking for. What do you think? - Gail's assistant w/eXp Realty";
    static S11: &'static str = "If you're still searching, we can visit a few open houses together in the next few weeks. It might help give clarity on what you're looking for. What do you think? - Elle's assistant w/Bright Birch Real Estate";

    #[test]
    fn test_min_hash_32() {
        let min_hash = MinHasher32::new(128);
        test_min_hash(&min_hash);
    }

    #[test]
    fn test_min_hash_16() {
        let min_hash = MinHasher16::new(128);
        test_min_hash(&min_hash);
    }

    #[test]
    fn test_min_hash_8() {
        let min_hash = MinHasher8::new(128);
        test_min_hash(&min_hash);
    }


    fn test_min_hash<M: MinHasher>(min_hash: &M) {
        let similarity = min_hash.compute_similarity(whitespace_split(S10), whitespace_split(S11)) as f32;
        let actual_similarity = compute_jaccard_similarity(whitespace_split(S10), whitespace_split(S11));
        println!("actual {} estimated {} ", actual_similarity, similarity);
        assert!(f32::abs(similarity - 0.75) < 0.15);

        let estimated_similarity =
            min_hash.compute_similarity(whitespace_split(S1), whitespace_split(S3)) as f32;
        let actual_similarity = compute_jaccard_similarity(whitespace_split(S1), whitespace_split(S3));
        println!(
            "actual {} estimated {}",
            actual_similarity, estimated_similarity
        );
        assert!(f32::abs(estimated_similarity - actual_similarity) < 0.15);
    }
}
