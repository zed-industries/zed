use crate::minhash::{compute_minhash_similarity, MinHasher};
use rand::distributions::{Distribution, Uniform};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use rayon::prelude::*;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::{BuildHasher, BuildHasherDefault, Hash, Hasher};
use fnv::FnvBuildHasher;


#[derive(Clone)]
pub struct MinHasher64V1<B: BuildHasher> {
    build_hasher: B,
    a: Vec<u64>,
    b: Vec<u64>,
    num_hashes: usize,
}

static MERSENNE_PRIME: u64 = (1 << 61) - 1;

impl MinHasher64V1<FnvBuildHasher> {
    /// Constructs a new `MinHash64V1` with a specified number of hash functions to use.
    /// ```
    /// use gaoya::minhash::MinHasher64V1;
    ///
    /// let min_hash = MinHasher64V1::new(100);
    /// ```
    pub fn new(num_hashes: usize) -> Self {
        MinHasher64V1::new_with_hasher(num_hashes, FnvBuildHasher::default())
    }
}

impl<B: BuildHasher> MinHasher64V1<B> {

    pub fn new_with_hasher(num_hashes: usize, build_hasher: B) -> Self {
       Self::new_with_hasher_and_seed(num_hashes, build_hasher, 3)
    }

    pub fn new_with_hasher_and_seed(num_hashes: usize, build_hasher: B, seed: u64) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);
        let rand_range1 = Uniform::from(1..MERSENNE_PRIME);
        let rand_range2 = Uniform::from(0..MERSENNE_PRIME);
        MinHasher64V1 {
            build_hasher,
            a: (0..num_hashes)
                .map(|_| rand_range1.sample(&mut rng))
                .collect(),
            b: (0..num_hashes)
                .map(|_| rand_range2.sample(&mut rng))
                .collect(),
            num_hashes,
        }
    }

    /// Returns the number of hash functions being used in `MinHash`.
    pub fn num_hashes(&self) -> usize {
        self.num_hashes
    }

    fn get_min_hashes_into_vec<T, U>(&self, iter: T, ret: &mut[u64])
    where
        T: Iterator<Item = U>,
        U: Hash,
    {
        assert_eq!(self.num_hashes, ret.len());
        let hashes: Vec<u64> = iter
            .map(|item| {
                let mut hasher = self.build_hasher.build_hasher();
                item.hash(&mut hasher);
                hasher.finish()
            })
            .collect::<Vec<_>>();

        if !hashes.is_empty() {
            for (index, hash) in ret.iter_mut().enumerate() {
                *hash = hashes
                    .iter()
                    .map(|hash| {
                        hash.wrapping_mul(self.a[index]).wrapping_add(self.b[index])
                            % MERSENNE_PRIME
                    })
                    .min()
                    .unwrap();
            }
        }
    }
}

impl<B: BuildHasher> MinHasher for MinHasher64V1<B> {
    type V = u64;
    fn create_signature<T, U>(&self, iter: T) -> Vec<u64>
    where
        T: Iterator<Item = U>,
        U: Hash,
    {
        let hashes: Vec<u64> = iter
            .map(|item| {
                let mut hasher = self.build_hasher.build_hasher();
                item.hash(&mut hasher);
                hasher.finish()
            })
            .collect::<Vec<_>>();

        match hashes.len() {
            len if len > 0 => self
                .a.iter()
                .zip(self.b.iter())
                .map(|ab| {
                    hashes
                        .iter()
                        .map(|hash| hash.wrapping_mul(*ab.0).wrapping_add(*ab.1) % MERSENNE_PRIME)
                        .min()
                        .unwrap()
                })
                .collect(),
            _ => vec![0; self.num_hashes],
        }
    }

}

#[cfg(test)]
mod tests {
    use super::MinHasher64V1;

    use crate::minhash::{minhash_centroid, compute_jaccard_similarity, MinHasher};
    use crate::text::whitespace_split;
    use std::f64;

    static S1: &'static str = "local sensitive hashing is cool";
    static S2: &'static str = "local sensitive hashing is great";
    static S3: &'static str = "local sensitive hashing is awesome";

    static S10: &'static str = "If you're still searching, we can visit a few open houses together in the next few weeks. It might help give clarity on what you're looking for. What do you think? - Gail's assistant w/eXp Realty";
    static S11: &'static str = "If you're still searching, we can visit a few open houses together in the next few weeks. It might help give clarity on what you're looking for. What do you think? - Elle's assistant w/Bright Birch Real Estate";

    #[test]
    fn test_min_hash_similarity() {
        let min_hash = MinHasher64V1::new(200);
        let similarity = min_hash.compute_similarity(whitespace_split(S10), whitespace_split(S11)) as f32;
        let actual_similarity = compute_jaccard_similarity(whitespace_split(S10), whitespace_split(S11));
        println!("actual {} estimated {} ", actual_similarity, similarity);
        assert!(f32::abs(similarity - 0.75) < 0.1);

        let estimated_similarity =
            min_hash.compute_similarity(whitespace_split(S1), whitespace_split(S3)) as f32;
        let actual_similarity = compute_jaccard_similarity(whitespace_split(S1), whitespace_split(S3));
        println!(
            "actual {} estimated {}",
            actual_similarity, estimated_similarity
        );
        assert!(f32::abs(estimated_similarity - actual_similarity) < 0.1);
    }

    #[test]
    fn test_min_hash_centroid() {
        let min_hashes = vec![
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 40],
            vec![1, 20, 3, 40],
            vec![1, 2, 3, 50],
        ];

        let centroid = minhash_centroid(&min_hashes);
        assert_eq!(vec![1, 2, 3, 40], centroid);
    }
}
