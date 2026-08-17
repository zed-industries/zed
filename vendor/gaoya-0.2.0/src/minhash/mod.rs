mod hashers;
mod min_hasher;
mod min_hasher64;

mod minhash_index;
mod string_index;
mod super_min_hash;
mod id_container;


pub use self::hashers::SipHasher24BuildHasher;
pub use self::hashers::Sha1Hasher;
pub use self::min_hasher64::MinHasher64V1;
pub use self::min_hasher::MinHasher8;
pub use self::min_hasher::MinHasher16;
pub use self::min_hasher::MinHasher32;
pub use self::minhash_index::MinHashIndex;
pub use self::string_index::MinHashStringIndex;
pub use self::id_container::IdContainer;
pub use self::id_container::HashSetContainer;
pub use self::id_container::SmallVecContainer;
pub use self::id_container::VecContainer;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::iter::FromIterator;
use num_traits::{AsPrimitive, PrimInt};
use rayon::prelude::*;

/// MinHashType can be any integer.
pub trait MinHashType: Hash + Send + Sync + PrimInt {}
impl MinHashType for u64 {}
impl MinHashType for u32 {}
impl MinHashType for u16 {}
impl MinHashType for u8 {}
impl MinHashType for i64 {}
impl MinHashType for i32 {}
impl MinHashType for i16 {}
impl MinHashType for i8 {}





pub trait MinHasher {
    /// The data type of individual hash.
    /// This should be one of u-numeric types such as u64, u32, u16, u8
    type V: MinHashType;

    fn create_signature<T, U>(&self, iter: T) -> Vec<Self::V>
        where
            T: Iterator<Item=U>,
            U: Hash;

    fn bulk_create_signature<U>(&self, batch: &Vec<Vec<U>>) -> Vec<Vec<Self::V>>
        where
            U: Hash + Sync,
            Self: Sync + Send {
        batch
            .par_iter()
            .map(|tokens| self.create_signature(tokens.iter()))
            .collect()
    }

    fn bulk_create_signature_refs<U>(&self, batch: &Vec<&Vec<U>>) -> Vec<Vec<Self::V>>
        where
            U: Hash + Sync,
            Self: Sync + Send {
        batch
            .par_iter()
            .map(|tokens| self.create_signature(tokens.iter()))
            .collect()
    }


    fn compute_similarity<T, U>(&self, iter_1: T, iter_2: T) -> f64
        where
            T: Iterator<Item=U>,
            U: Hash {
        compute_minhash_similarity(
            &self.create_signature(iter_1),
            &self.create_signature(iter_2),
        )
    }
}

pub fn compute_jaccard_similarity<T, U>(iter_1: T, iter_2: T) -> f32
    where
        T: Iterator<Item=U>,
        U: Hash + Eq,
{
    let h1 = HashSet::<U>::from_iter(iter_1);
    let h2 = HashSet::<U>::from_iter(iter_2);
    let intersection_len = h1.intersection(&h2).count();
    intersection_len as f32 / (h1.len() + h2.len() - intersection_len) as f32
}

pub fn compute_jaccard_distance<T, U>(iter_1: T, iter_2: T) -> f32
    where
        T: Iterator<Item=U>,
        U: Hash + Eq,
{
    1.0 - compute_jaccard_similarity(iter_1, iter_2)
}


/// Calculates jaccard similarity between two minhashes
/// # Examples
///
/// ```
/// use gaoya::minhash::compute_minhash_similarity;
///
/// let m1 = [1, 2, 3, 4, 5, 6];
/// let m2 = [1, 2, 3, 7, 5, 8];
/// assert!((compute_minhash_similarity(&m1, &m2) - 0.666) < 0.01);
///
/// ```
#[inline]
pub fn compute_minhash_similarity<T>(min_hashes_1: &[T], min_hashes_2: &[T]) -> f64
    where
        T: Eq,
{
    assert_eq!(min_hashes_1.len(), min_hashes_2.len());
    let num_hashes = min_hashes_1.len();
    let matches: u64 = min_hashes_1
        .iter()
        .zip(min_hashes_2.iter())
        .map(|(min_hash_1, min_hash_2)| (min_hash_1 == min_hash_2) as u64)
        .sum();
    (matches as f64) / (num_hashes as f64)
}

#[inline]
pub fn compute_minhash_distance<T>(min_hashes_1: &[T], min_hashes_2: &[T]) -> f64
    where
        T: Eq,
{
    1.0 - compute_minhash_similarity(min_hashes_1, min_hashes_2)
}

pub fn similarity_greater_than_threshold<T>(
    min_hashes_1: &[T],
    min_hashes_2: &[T],
    threshold: f64,
) -> bool
    where
        T: Eq,
{
    assert_eq!(min_hashes_1.len(), min_hashes_2.len());
    let num_hashes = min_hashes_1.len();
    let expected_matches = (num_hashes as f64 * threshold) as u32;
    let mut num_matches: u32 = 0;
    for pair in min_hashes_1.iter().zip(min_hashes_2.iter()) {
        if pair.0 == pair.1 {
            num_matches += 1;
        }
        if num_matches >= expected_matches {
            return true;
        }
    }
    false
}

fn minhash_centroid<T>(signatures: &[Vec<T>]) -> Vec<T>
where
    T: Hash + Copy + Eq,
{
    let signature_len = signatures[0].len();
    let mut centroid = Vec::with_capacity(signature_len);
    let mut hash_counters: Vec<HashMap<T, usize>> = vec![HashMap::new(); signature_len];
    for signature in signatures.iter() {
        for (i, hash) in signature.iter().enumerate() {
            let count = hash_counters[i].entry(*hash).or_insert(1);
            *count += 1;
        }

    }
    for counter in hash_counters {
        let most_frequent_hash = counter.into_iter()
            .max_by(|x, y| x.1.cmp(&y.1))
            .unwrap().0;
        centroid.push(most_frequent_hash);
    }

    centroid

}

fn minhash_band_centroid_from_refs<T>(signatures: &[&Vec<T>], num_bands: usize, band_size: usize) -> Vec<T>
    where
        T: Hash + Copy + Eq,
{
    let mut band_counters: Vec<HashMap<&[T], usize>> = Vec::new();
    for i in 0..num_bands {
        band_counters.push(HashMap::new());
    }

    for signature in signatures.iter() {
        for i in 0..num_bands {
            let band: &[T] = &signature[i * band_size..(i + 1) * band_size];
            let count = band_counters[i].entry(band).or_insert(1);
            *count += 1;
        }
    }

    let mut centroid = Vec::new();
    for counter in band_counters {
        let most_frequent_band = counter.into_iter()
            .max_by(|x, y| x.1.cmp(&y.1))
            .unwrap().0;
        centroid.extend_from_slice(most_frequent_band);
    }
    centroid

}

fn minhash_centroid_from_refs<T>(signatures: &[&Vec<T>]) -> Vec<T>
    where
        T: Hash + Copy + Eq,
{
    let signature_len = signatures[0].len();
    let mut centroid = Vec::with_capacity(signature_len);
    let mut hash_counters: Vec<HashMap<T, usize>> = vec![HashMap::new(); signature_len];
    for signature in signatures.iter() {
        for (i, hash) in signature.iter().enumerate() {
            let count = hash_counters[i].entry(*hash).or_insert(1);
            *count += 1;
        }

    }
    for counter in hash_counters {
        let most_frequent_hash = counter.into_iter()
            .max_by(|x, y| x.1.cmp(&y.1))
            .unwrap().0;
        centroid.push(most_frequent_hash);
    }

    centroid
}


/// Calculates number of bands `b` and band width `r` (number of rows) given
/// the minimum `jaccard similarity`, number of hashes `num_hashes`, and desired
/// probability `desired_proba` of two sets with similarity > `jaccard_similarity` to
/// share a bucket
/// For more info see 3.4.2 in http://infolab.stanford.edu/~ullman/mmds/ch3n.pdf
///
/// # Examples
///
/// ```
/// use gaoya::minhash::calculate_minhash_params;
/// let (b, r) = calculate_minhash_params(0.5, 128);
/// assert_eq!(b, 42);
/// assert_eq!(r, 3);
///
/// let (b, r) = calculate_minhash_params(0.7, 196);
///  assert_eq!(b, 39);
///  assert_eq!(r, 5);
/// ```
pub fn calculate_minhash_params(jaccard_similarity: f64, num_hashes: usize)
    -> (usize, usize) {
    calculate_b_and_r(jaccard_similarity, num_hashes, 0.99)
}

pub fn calculate_minhash_params_with_proba(jaccard_similarity: f64, num_hashes: usize, desired_proba: f64)
                                -> (usize, usize) {
    calculate_b_and_r(jaccard_similarity, num_hashes, desired_proba)
}


fn calculate_b_and_r(s: f64, n: usize, p: f64) -> (usize, usize) {
    let proba = |b, r| {
        1.0 - (1.0 - s.powf(r)).powf(b)
    };
    let mut b = n;
    let mut r = 1;
    while b > 1 {
        let r1 = r + 1;
        let b1 = n / r1;
        if proba(b1 as f64, r1 as f64) > p {
            b = b1;
            r = r1;
        } else {
            break;
        }
    }
    (b, r)
}


#[cfg(test)]
mod tests {
    use std::cmp::min;
    use crate::minhash::{minhash_centroid, compute_minhash_similarity};


    #[test]
    fn test_min_hash_centroid() {
        let min_hashes = vec![
            vec![1, 2,  3, 4,  5],
            vec![1, 2,  3, 40, 51],
            vec![1, 20, 3, 40, 52],
            vec![1, 2,  3, 50, 55],
            vec![1, 2,  3, 60, 55],
        ];

        let centroid = minhash_centroid(&min_hashes);
        assert_eq!(vec![1, 2, 3, 40, 55], centroid);

        // the minhash jaccard similarity from centroid to any point should be
        // greater than pairwise similarity of every point
        let pairwise_similarities: Vec<Vec<f64>> = min_hashes.iter()
            .map(|m1| {
                min_hashes.iter()
                    .map(|m2| compute_minhash_similarity(m1, m2))
                    .collect()

            }).collect();

        let sums_similarity_from_points: Vec<f64> = pairwise_similarities.iter()
            .map(|similarities| similarities.iter().sum())
            .collect();

        let sum_similarity_from_centroid: f64 = min_hashes.iter()
            .map(|minhash| compute_minhash_similarity(minhash, &centroid))
            .sum();

        for s in sums_similarity_from_points {
            assert!(sum_similarity_from_centroid > s);
        }
    }
}