use crate::minhash::{calculate_b_and_r, compute_minhash_distance, compute_minhash_similarity,
                     minhash_band_centroid_from_refs, minhash_centroid, MinHashType
};
use rayon::prelude::*;
use std::any::type_name;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::{BuildHasher, Hash, Hasher};
use std::{fmt, slice};
#[cfg(all(feature = "unstable"))]
use std::collections::hash_map::RawEntryMut;

use std::fmt::{Display, Formatter};
use std::marker::PhantomData;
use std::ops::{Index, Range};
use ahash::{AHasher, AHashMap, AHashSet, RandomState};
use itertools::Itertools;
use crate::clustering::QueryIndex;
use crate::minhash::id_container::{HashSetContainer, IdContainer};


/// BandKey contains the hash of the band.
/// Using the hash of the band instead of the whole band slice will not decrease
/// recall. Banding provides candidates, which then are compared with the search query using full
/// minhash-jaccard similarity
struct BandKey {
    pub hash: u64
}

impl BandKey {

    #[inline]
    pub fn new<T: MinHashType>(band: &[T], mut hasher: AHasher) -> Self {
        band.hash(&mut hasher);
        Self {
            hash:  hasher.finish()
        }
    }
}

impl Hash for BandKey {

    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.hash)
    }
}

impl Eq for BandKey {}

impl PartialEq for BandKey {

    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}

/// Because we hash the band slice in the BandKey using AHash we don't need
/// to hash the hash in the hashmap.
struct NoOpHasher {
    pub hash: u64
}

impl Hasher for NoOpHasher {

    #[inline]
    fn finish(&self) -> u64 {
        debug_assert!(self.hash > 0);
        self.hash
    }

    fn write(&mut self, bytes: &[u8]) {
        panic!("Should not have been called");
    }

    #[inline]
    fn write_u64(&mut self, h: u64) {
        // h is the BandKey.hash
        self.hash = h;
    }
}

struct NoOpHashBuilder {}

impl BuildHasher for NoOpHashBuilder {
    type Hasher = NoOpHasher;

    #[inline]
    fn build_hasher(&self) -> Self::Hasher {
        NoOpHasher {hash: 0}
    }

}


struct MinHashBand<T, Id, C>
where
    T: MinHashType,
    Id: Hash + Eq + Clone,
    C: IdContainer<Id>
{
    hash_table: HashMap<BandKey, C, NoOpHashBuilder>,
    band_start: usize,
    band_end: usize,
    len: usize,
    build_ahash: RandomState,
    phantom: PhantomData<(T, Id)>,
}

impl<T, Id, C> MinHashBand<T, Id, C>
where
    T: MinHashType,
    Id: Hash + Eq + Clone,
    C: IdContainer<Id>
{
    pub fn new(band_start: usize, band_end: usize, build_ahash: RandomState) -> Self {
        let mut hash_table = HashMap::with_hasher(NoOpHashBuilder {});
        hash_table.reserve(1000);
        MinHashBand {
            hash_table,
            band_start,
            band_end,
            len: (band_end - band_start) as usize,
            build_ahash,
            phantom: PhantomData,
        }
    }

    pub fn new_with_capacity(band_start: usize,
                             band_end: usize,
                             capacity: usize,
                             build_ahash: RandomState) -> Self {
        let mut hash_table = HashMap::with_hasher(NoOpHashBuilder {});
        hash_table.reserve(capacity);
        MinHashBand {
            hash_table,
            band_start,
            band_end,
            len: (band_end - band_start) as usize,
            build_ahash,
            phantom: PhantomData,
        }
    }


    #[inline]
    fn insert(&mut self, id: Id, signature: &[T]) {
        let band_data = &signature[self.band_start..self.band_end];
        let band_key = BandKey::new(band_data, self.build_ahash.build_hasher());
        self.hash_table
            .entry(band_key)
            .or_insert_with(|| C::new())
            .push(id);
    }

    #[inline]
    fn query<'a, S: BuildHasher>(&'a self, signature: &[T], match_ids: &mut HashSet<&'a Id, S>) {
        let band_data = &signature[self.band_start..self.band_end];
        let band_key = BandKey::new(band_data, self.build_ahash.build_hasher());
        match self.hash_table.get(&band_key) {
            Some(ids) =>  ids.copy_refs_to(match_ids),
            None => (),
        }
    }

    #[inline]
    fn query_to_owned(&self, signature: &[T], match_ids: &mut HashSet<Id>) {
        let band_data = &signature[self.band_start..self.band_end];
        let band_key = BandKey::new(band_data, self.build_ahash.build_hasher());
        match self.hash_table.get(&band_key) {
            Some(ids) => {
                ids.copy_to(match_ids);
            }
            None => (),
        }
    }

    /// Returns the index of signature that gives highest recall
    /// of this band on points that are not in all_ids.
    /// Used by centroid calculation to choose the most optimal
    /// band portion of the hash
    // fn find_signature_with_highest_recall<'a>(
    //     &'a self,
    //     signatures: &[&[T]],
    //     all_ids: &mut HashSet<&'a Id>,
    // ) -> Option<usize> {
    //     let mut max_count: usize = 0;
    //     let mut best_index: isize = -1;
    //     for minhash in signatures.iter().enumerate() {
    //         let band_data = &minhash.1[self.band_start..self.band_end];
    //         let band_key = BandKey::new(band_data, self.build_ahash.build_hasher());
    //         match self.hash_table.get(&band_key) {
    //             Some(ids) => {
    //                 let new_count = ids.iter()
    //                     .map(|id| !all_ids.contains(&id) as usize)
    //                     .count();
    //                 if new_count > max_count {
    //                     max_count = new_count;
    //                     best_index = minhash.0 as isize;
    //                 }
    //             }
    //             None => (),
    //         }
    //     }
    //     let band_data = &signatures[best_index as usize][self.band_start..self.band_end];
    //     let band_key = BandKey::new(band_data, self.build_ahash.build_hasher());
    //     match self.hash_table.get(&band_key) {
    //         Some(ids) => {
    //             all_ids.extend(ids.iter())
    //         }
    //         None => (),
    //     }
    //     if best_index >= 0 {
    //         Some(best_index as usize)
    //     } else {
    //         None
    //     }
    // }

    /// Removes id from the band
    /// Returns true if the band portion of the signature is not in the hashtable
    fn remove(&mut self, id: &Id, signature: &[T]) {
        let band_data = &signature[self.band_start..self.band_end];
        let band_key = BandKey::new(band_data, self.build_ahash.build_hasher());
        match self.hash_table.get_mut(&band_key) {
            Some(ids) => {
                ids.remove(id);
                if ids.is_empty() {
                    self.hash_table.remove(&band_key);
                }
            }
            None => (),
        }
    }

    fn clear(&mut self) {
        self.hash_table.clear();
    }

    #[inline]
    fn has_ids(&self, signature: &[T]) -> bool {
        let band_data = &signature[self.band_start as usize..self.band_end as usize];
        let band_key = BandKey::new(band_data, self.build_ahash.build_hasher());

        match self.hash_table.get(&band_key) {
            Some(ids) => ids.len() > 0,
            None => false,
        }
    }

    pub fn shrink_to_fit(&mut self) {
        for item in self.hash_table.iter_mut() {
            //item.1.shrink_to_fit();
        }
        self.hash_table.shrink_to_fit();
    }

    pub fn shrink_to(&mut self, min_capacity: usize) {
        for item in self.hash_table.iter_mut() {
            //item.1.shrink_to_fit();
        }
        self.hash_table.shrink_to(min_capacity);
    }
}

/// Data Structure to index minhashes into bands.
///
/// Reference: [Chapter 3, Mining of Massive Datasets](http://www.mmds.org)
///
/// MinHashIndex implements classic banding technique described in MMDS book.
/// Full MinHash signatures are stored in the index. [`MinHashIndex::query`](struct.MinHashIndex.html#method.query)
/// selects candidates from each band, then filters candidates by calculating full
/// jaccard similarity between the query and the candidate.
/// Configuration parameters to [`MinHashIndex::new`](struct.MinHashIndex.html#method.new)
/// `num_bands` and `band_width` correspond to `b` and `r` in MMDS book.
///
/// # Type Parameters
/// `T`: The type of the individual hash in the signature
/// `Id`: The type of id associated with each signature. The type can be any `Hash` + `Eq` + `Clone`
///  and `Send` + `Sync` for bulk operations. The id will be cloned and stored in every band,
///  so it is best to use small ids. Integer makes a great id for `MinHashIndex`. For [`String`] ids
/// use `Rc<String>` or `Arc<String>`
/// `C`: The type of the [`IdContainer`] used to store ids in a band.
/// # Examples
///
/// ```
/// use std::collections::HashSet;
/// use gaoya::minhash::{MinHashIndex, MinHasher32, MinHasher} ;
/// use gaoya::text::whitespace_split;
/// let corpus = [
///     "This is the first document.",
///     "This document is the second document.",
///     "And this is the third document.",
///     "Is this the first document?",
///     "This not the first nor the second nor the third, but the fourth document"];
/// let (num_bands, band_width) = (42, 3);
/// let minhasher = MinHasher32::new(num_bands * band_width);
/// let mut index = MinHashIndex::new(num_bands, band_width, 0.5);
/// for (i, doc) in corpus.iter().enumerate() {
///     index.insert(i, minhasher.create_signature(whitespace_split(&doc.to_lowercase())));
/// }
/// for (i, doc) in corpus.iter().enumerate() {
///     if i < 4 {
///         let mut expected = HashSet::default();
///         expected.extend(vec![0, 1, 2, 3].into_iter());
///         assert_eq!(index.query_owned(&minhasher.create_signature(whitespace_split(&doc.to_lowercase()))), expected);
///     } else {
///         let mut expected = HashSet::default();
///         expected.insert(4);
///         assert_eq!(index.query_owned(&minhasher.create_signature(whitespace_split(&doc.to_lowercase()))), expected);
///     }
/// }
///
/// ```
#[derive()]
pub struct MinHashIndex<T, Id, C = HashSetContainer<Id>>
where
    T: MinHashType,
    Id: Hash + Eq + Clone,
    C: IdContainer<Id>
{
    bands: Vec<MinHashBand<T, Id, C>>,
    id_signatures: HashMap<Id, Vec<T>, ahash::RandomState>,
    threshold: f64,
    r: usize,
    b: usize,
    num_hashes: usize
}

impl<T, Id, C> fmt::Display for MinHashIndex<T, Id, C>
where
    T: MinHashType,
    Id: Hash + Eq + Clone,
    C: IdContainer<Id>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MinHashIndex<{}> {{ threshold = {}, num_hashes = {}, bands = {}, rows_per_band = {}, size = {} }}",
               type_name::<T>(),
               self.threshold, self.b * self.r, self.b, self.r, self.size())
    }
}

impl<T, Id> MinHashIndex<T, Id>
    where
        T: MinHashType,
        Id: Hash + Eq + Clone + Send + Sync
{
    /// Create a new MinHashIndex
    pub fn new(num_bands: usize, band_width: usize, jaccard_threshold: f64) -> Self {
        MinHashIndex::<T, Id, HashSetContainer<Id>>::new_index(num_bands, band_width, jaccard_threshold)
    }
}


impl<T, Id, C> MinHashIndex<T, Id, C>
where
    T: MinHashType,
    Id: Hash + Eq + Clone,
    C: IdContainer<Id>
{
    /// Create a new MinHashIndex
    pub fn new_index(num_bands: usize, band_width: usize, jaccard_threshold: f64) -> Self {
        let build_hasher = RandomState::new();
        let mut bands = Vec::new();
        for i in 0..num_bands {
            let (start, end) = (i * band_width, (i + 1) * band_width);
            bands.push(MinHashBand::<T, Id, C>::new(start, end, build_hasher.clone()));
        }
        let mut hash_table = HashMap::with_hasher(ahash::RandomState::new());
        hash_table.reserve(1000);
        MinHashIndex {
            bands,
            threshold: jaccard_threshold,
            id_signatures: hash_table,
            b: num_bands,
            r: band_width,
            num_hashes: num_bands * band_width,
        }
    }

    /// Creates new MinHashIndex with specified `initial_capacity` and `expected_similarity_ratio`,
    /// which is the expected ratio of similar documents to the total.
    /// `MinHashIndex` stores signatures in bands, such that similar signatures locate in
    /// the same location within the band. The size of the band in inversely proportional to
    /// the similarity ratio.
    /// For example with similarity ratio 0.9 the band size on average will be 0.1 of the total
    /// number of signatures.
    pub fn new_with_capacity(num_bands: usize, band_width: usize,
                             jaccard_threshold: f64,
                             initial_capacity: usize,
                             expected_similarity_ratio: f64) -> Self {
        let mut bands = Vec::new();
        let build_hasher = RandomState::new();

        let band_capacity = (initial_capacity as f64 * (1.0 - expected_similarity_ratio)) as usize;
        for i in 0..num_bands {
            let (start, end) = (i * band_width, (i + 1) * band_width);
            bands.push(MinHashBand::<T, Id, C>::new_with_capacity(start, end, band_capacity, build_hasher.clone()));
        }
        let mut hash_table = HashMap::with_hasher(ahash::RandomState::new());
        hash_table.reserve(initial_capacity);
        MinHashIndex {
            bands,
            threshold: jaccard_threshold,
            id_signatures: hash_table,
            b: num_bands,
            r: band_width,
            num_hashes: num_bands * band_width,
        }
    }


    /// Returns a reference to the map containing all inserted points
    pub fn get_id_signature_map(&self) -> &HashMap<Id, Vec<T>, ahash::RandomState> {
        &self.id_signatures
    }


    #[inline]
    pub fn insert(&mut self, id: Id, signature: Vec<T>) {
        assert_eq!(self.num_hashes(), signature.len());
        for band in &mut self.bands {
            band.insert(id.clone(), &signature);
        }
        self.id_signatures.insert(id, signature);
    }

    pub fn par_bulk_insert(&mut self, ids: Vec<Id>, signatures: Vec<Vec<T>>)
    where
        Id: Hash + Eq + Clone + Send + Sync,
        T: Send + Sync,
    {
        if !signatures.is_empty() {
            assert_eq!(self.num_hashes(), signatures[0].len());
        }

        self.bands.par_iter_mut().for_each(|band| {
            for item in signatures.iter().zip(ids.iter()) {
                let hashes = item.0;
                let id = item.1.clone();
                band.insert(id, hashes);
            }
        });

        for id_hash in ids.into_iter().zip(signatures.into_iter()) {
            self.id_signatures.insert(id_hash.0, id_hash.1);
        }
    }

    pub fn par_bulk_insert_pairs(&mut self, id_signature_pairs: Vec<(Id, Vec<T>)>)
    where
        Id: Hash + Eq + Clone + Send + Sync,
        T: Send + Sync,
    {
        self.bands.par_iter_mut().for_each(|band| {
            for item in id_signature_pairs.iter() {
                let i: &(Id, Vec<T>) = item;
                let (a, b) = i;
                let k: Id = a.clone();
                band.insert(k, b);
            }
        });

        for id_hash in id_signature_pairs {
            self.id_signatures.insert(id_hash.0, id_hash.1);
        }
    }

    pub fn shrink_to_fit(&mut self)
    where Id: Send + Sync,
          T: Send + Sync
    {
        self.bands.par_iter_mut()
            .for_each(|band| band.shrink_to_fit());
        self.id_signatures.shrink_to_fit();
    }

    pub fn shrink_to(&mut self, min_capacity: usize)
        where Id: Send + Sync,
              T: Send + Sync
    {
        if min_capacity > self.id_signatures.capacity() {
            return;
        }
        self.bands.par_iter_mut()
            .for_each(|band| {
                // On average the size of every band will be lower than the size of
                // id_signatures. We adjust the desired capacity.
                let ratio = band.hash_table.len() as f64 / self.id_signatures.len() as f64;
                assert!(ratio < 1.0);
                let adjusted_capacity = (min_capacity as f64 * ratio) as usize;
                band.shrink_to(adjusted_capacity)
            });
        self.id_signatures.shrink_to(min_capacity);
    }


    pub fn clear(&mut self) {
        self.bands.iter_mut().for_each(|band| band.clear());
        self.id_signatures.clear();
    }

    pub fn get_signature(&self, id: &Id) -> Option<&Vec<T>> {
        self.id_signatures.get(id)
    }

    /// Queries the index for the closest to query_signature point and returns a tuple
    /// where the first element is the reference to id and the second is the jaccard similarity
    /// between input signature and the signature of the returned point
    /// Returns None if no point in the index is within the threshold.
    ///
    /// # Examples
    ///
    /// ```
    /// use gaoya::minhash::{MinHasher,MinHasher16, MinHashIndex};
    /// use gaoya::text::whitespace_split;
    ///
    /// let mut index = MinHashIndex::new(33, 3, 0.6);
    /// let minhasher = MinHasher16::new(33 * 3);
    /// let signature1 = minhasher.create_signature(["a", "b", "c", "d", "e", "f"].iter());
    /// let signature2 = minhasher.create_signature(["a", "b", "c", "d", "e", "g"].iter());
    /// let signature3 = minhasher.create_signature(["a", "b", "c", "d"].iter());
    /// let query = signature1.clone();
    /// index.insert(1u32, signature1.clone());
    /// index.insert(3u32, signature3);
    /// let result = index.query_one(&signature2).unwrap();
    /// assert_eq!(*result.0, 1);
    /// assert!(f64::abs(result.1 - 0.71) < 0.05);
    /// ```

    pub fn query_one(&self, query_signature: &Vec<T>) -> Option<(&Id, f64)> {
        assert_eq!(self.num_hashes(), query_signature.len());
        let mut match_ids = HashSet::with_capacity(10);
        for band in &self.bands {
            band.query(query_signature, &mut match_ids);
        }

        match_ids.into_iter()
            .map(|id| (id, self.id_signatures.get(&id)))
            .filter(|(id, sig)| sig.is_some())
            .map(|(id, sig)| (id, compute_minhash_similarity(sig.unwrap(), query_signature)))
            .filter(|pair| pair.1 > self.threshold)
            .max_by(|x, y| x.1.partial_cmp(&y.1).unwrap())
    }

    pub fn query(&self, query_signature: &Vec<T>) -> HashSet<&Id> {
        assert_eq!(self.num_hashes(), query_signature.len());
        let mut match_ids = HashSet::with_capacity(10);
        for band in &self.bands {
            band.query(query_signature, &mut match_ids);
        }

        match_ids.retain(|id| {
            match self.id_signatures.get(id) {
                Some(signature) => {
                    compute_minhash_similarity(signature, query_signature) >= self.threshold
                },
                None => false
            }
        });
        match_ids
    }

    #[inline]
    pub fn query_by_id(&self, id: &Id) -> HashSet<&Id> {
        match self.id_signatures.get(id) {
            Some(signature) => self.query(signature),
            None => HashSet::new()
        }
    }

    #[inline]
    pub fn query_by_id_owned(&self, id: &Id) -> HashSet<Id> {
        match self.id_signatures.get(id) {
            Some(signature) => self.query_owned(signature),
            None => HashSet::new()
        }
    }

    pub fn query_owned(&self, query_signature: &Vec<T>) -> HashSet<Id>
    where
        Id: Hash + Eq + Clone,
    {
        assert_eq!(self.num_hashes(), query_signature.len());
        let mut match_ids = HashSet::with_capacity(10);
        for band in &self.bands {
            band.query_to_owned(query_signature, &mut match_ids);
        }
        match_ids.retain(|id| {
            match self.id_signatures.get(id) {
                Some(signature) => {
                    compute_minhash_similarity(signature, query_signature) >= self.threshold
                },
                None => false
            }
        });
        match_ids
    }

    pub fn par_bulk_query(&self, signatures: &Vec<Vec<T>>) -> Vec<HashSet<Id>>
        where
            Id: Hash + Eq + Clone + Send + Sync,
            T: Send + Sync
    {
        signatures.par_iter()
            .map(|signature| self.query_owned(signature))
            .collect()
    }

    pub fn par_bulk_query_return_similarity(&self, signatures: &Vec<Vec<T>>) -> Vec<Vec<(Id, f64)>>
        where
            Id: Hash + Eq + Clone + Send + Sync,
            T: Send + Sync
    {
        signatures.par_iter()
            .map(|signature| self.query_owned_return_similarity(signature))
            .collect()
    }


    pub fn query_owned_return_similarity(&self, query_signature: &[T]) -> Vec<(Id, f64)>
        where
            Id: Hash + Eq + Clone,
    {
        let mut match_ids = HashSet::with_capacity(10);
        for band in &self.bands {
            band.query_to_owned(query_signature, &mut match_ids);
        }
        let mut result = Vec::new();
        for id in match_ids.into_iter() {
            let signature = &self.id_signatures[&id];
            let similarity = compute_minhash_similarity(signature, query_signature);
            if similarity >= self.threshold {
                result.push((id, similarity))
            }
        }
        result.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        result
    }


    pub fn query_top_k(&self, query_signature: &[T], k: usize) -> Vec<(Id, f64)> {
        let mut match_ids = HashSet::with_capacity(10);
        for band in &self.bands {
            band.query_to_owned(query_signature, &mut match_ids);
        }
        let mut ids_distances: Vec<(Id, f64)> = match_ids
            .into_iter()
            .map(|id| {
                let signature = &self.id_signatures[&id];
                let distance = compute_minhash_distance(query_signature, signature);
                (id, distance)
            })
            .collect();
        ids_distances.sort_unstable_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        ids_distances[0..std::cmp::min(ids_distances.len(), k)].to_vec()
    }


    /// Removes an id from the index, returning the signature at the id if the id
    /// was previously in the index.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use ahash::AHasher;
    /// use gaoya::minhash::{MinHasher,MinHasher16, MinHashIndex};
    /// use gaoya::text::whitespace_split;
    ///
    /// let mut index = MinHashIndex::new(33, 3, 0.6);
    /// let minhasher = MinHasher16::new(33 * 3);
    /// let signature1 = minhasher.create_signature(whitespace_split("This is the first minhashed document"));
    /// let signature2 = minhasher.create_signature(whitespace_split("This is the second minhashed document"));
    /// let query = signature1.clone();
    /// index.insert(1u32, signature1.clone());
    /// index.insert(2u32, signature2.clone());
    /// let mut result =  index.query_owned(&query).into_iter().collect::<Vec<_>>();
    /// result.sort();
    /// assert_eq!(result, vec![1, 2]);
    /// assert_eq!(index.remove(&1), Some(signature1));
    /// assert_eq!(index.remove(&1), None);
    /// ```
    pub fn remove(&mut self, id: &Id) -> Option<Vec<T>> {
        match self.id_signatures.remove(id) {
            Some(signature) => {
                self
                    .bands
                    .iter_mut()
                    .for_each(|band| band.remove(id, &signature));
                Some(signature)
            }
            None => None
        }
    }

    // Removes signatures for the specified ids. Returns removed signatures
    pub fn bulk_remove(&mut self, ids: &Vec<Id>) -> Vec<Vec<T>>
        where
            Id: Hash + Eq + Clone + Send + Sync,
            T: Send + Sync {
        let sigs: Vec<Vec<T>> = ids.iter()
            .filter_map(|id| self.id_signatures.remove(id))
            .collect();

        self.bands.par_iter_mut()
            .for_each(|band| {
                sigs.iter().zip(ids)
                    .for_each(|(sig, id)| band.remove(id, sig))
            });
        sigs
    }

    pub fn size(&self) -> usize {
        self.id_signatures.len()
    }

    pub fn capacity(&self) -> usize {
        self.id_signatures.capacity()
    }

    pub fn num_hashes(&self) -> usize {
        self.num_hashes
    }

    /// This method filters candidates by measuring similarity between query_minhash and
    /// each candidate using full minhash similarity measure.
    fn filter_by_minhash_similarity<'a>(
        &self,
        query_signature: &[T],
        candidates: HashSet<&'a Id>,
    ) -> HashSet<&'a Id> {
        let mut result = HashSet::new();
        for candidate in candidates {
            let candidate_signature = &self.id_signatures[candidate];
            let similarity = compute_minhash_distance(query_signature, candidate_signature);
            if similarity >= self.threshold {
                result.insert(candidate);
            }
        }
        result
    }

    /// Calculates minhash centroid that optimizes recall for this `MinHashIndex` configuration
    pub fn calculate_centroid(&self, ids: &[Id]) -> Vec<T> where
         {

        let signatures: Vec<_> = ids.iter()
            .filter_map(|id| self.id_signatures.get(id))
            .collect();
        minhash_band_centroid_from_refs(&signatures, self.b, self.r)
    }

    // pub fn calculate_centroid_experimental<I>(&self,  ids: I) -> Vec<T>
    // where
    //     I: Iterator<Item = Id> {
    //     let mut bands: Vec<HashSet<&[T]>> = Vec::new();
    //     for i in 0..self.b {
    //         bands.push(HashSet::new());
    //     }
    //     let mut first_signature = None;
    //     for id in ids {
    //         let mut signature = self.id_signatures.get(&id).unwrap();
    //         for i in 0..self.b {
    //             let band: &[T] = &signature[self.band_range(i)];
    //             bands[i].insert(band);
    //         }
    //
    //         match first_signature {
    //             None => {
    //                 first_signature.insert(signature);
    //             }
    //             Some(_) => {}
    //         };
    //     }
    //     let first_signature = first_signature.unwrap();
    //     let mut all_ids = HashSet::new();
    //     let mut centroid_signature = Vec::new();
    //     for i in 0..self.b {
    //         let band: &MinHashBand<T, Id, C> = &self.bands[i];
    //         let band_signatures: Vec<&[T]> = bands[i].iter().map(|k| *k).collect();
    //         let index = band.find_signature_with_highest_recall(&band_signatures, &mut all_ids);
    //         match index {
    //             Some(index) => {
    //                 centroid_signature.extend_from_slice(band_signatures[index]);
    //             }
    //             None => {
    //                 centroid_signature.extend_from_slice(&first_signature[self.band_range(i)]);
    //             }
    //         }
    //     }
    //     centroid_signature
    // }

    fn band_range(&self, band_index: usize) -> Range<usize> {
        band_index * self.r..(band_index + 1) * self.r
    }

    pub fn band_sizes(&self) -> BandStats {
        let band_sizes: Vec<_> = self.bands.iter()
            .map(|band| band.hash_table.len())
            .collect();
        let max_size = band_sizes.iter().max().unwrap();
        let min_size = band_sizes.iter().min().unwrap();
        BandStats {
            min_size: *min_size,
            max_size: *max_size,
            sizes: band_sizes
        }
    }

}


#[derive(Debug)]
pub struct BandStats {
    pub min_size: usize,
    pub max_size: usize,
    pub sizes: Vec<usize>
}

impl<T, Id, C> QueryIndex for MinHashIndex<T, Id, C>
    where
        T: MinHashType ,
        Id: Hash + Eq + Clone,
        C: IdContainer<Id>{
    type Id = Id;

    fn query(&self, id: &Self::Id) -> HashSet<&Self::Id> {
        match self.id_signatures.get(id) {
            Some(signature) => {
                self::MinHashIndex::query(self, signature)
            }
            None => HashSet::new()
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::minhash::min_hasher64::MinHasher64V1;
    use crate::minhash::{calculate_b_and_r, calculate_minhash_params, HashSetContainer, IdContainer, MinHasher, MinHashIndex, SmallVecContainer};
    use rand::distributions::{Distribution, Uniform};
    use rand::prelude::ThreadRng;
    use rand::{thread_rng, Rng};
    use std::borrow::Borrow;
    use std::collections::HashSet;
    use fnv::FnvBuildHasher;
    use crate::minhash::id_container::VecContainer;
    use crate::minhash::min_hasher::MinHasher32;
    use crate::text::whitespace_split;

    static S1: &'static str = "local sensitive hashing is cool";
    static S2: &'static str = "local sensitive hashing is great";
    static S3: &'static str = "local sensitive hashing is awesome";
    static S4: &'static str = "we all scream for ice cream";
    static S5: &'static str = "we all scream for ice cream sandwich";
    static S6: &'static str = "i like ice cream sandwich";

    #[test]
    pub fn test_lsh_index() {
        let (b, r) = calculate_minhash_params(0.5, 200);
        let min_hash = MinHasher64V1::new(b * r);
        let mut lsh_index = MinHashIndex::new(b, r, 0.5);
        lsh_index.insert(1, min_hash.create_signature(S1.split_whitespace()));
        lsh_index.insert(2, min_hash.create_signature(S2.split_whitespace()));
        lsh_index.insert(3, min_hash.create_signature(S3.split_whitespace()));
        lsh_index.insert(4, min_hash.create_signature(S4.split_whitespace()));
        lsh_index.insert(5, min_hash.create_signature(S5.split_whitespace()));
        lsh_index.insert(6, min_hash.create_signature(S6.split_whitespace()));

        println!("{}", lsh_index);
        assert_eq!(lsh_index.size(), 6);
        let ret = lsh_index.query(&min_hash.create_signature(S2.split_whitespace()));

        let ret_str: String = ret.iter().map(|x| x.to_string()).collect();
        assert_eq!(ret.len(), 3, "{}", ret_str);
        assert!(ret.contains(&1));
        assert!(ret.contains(&2));
        assert!(ret.contains(&3));

        lsh_index.remove(&2);
        assert_eq!(lsh_index.size(), 5);
        let ret = lsh_index.query(&min_hash.create_signature(S2.split_whitespace()));
        assert_eq!(ret.len(), 2);
        assert!(ret.contains(&1));
        assert!(ret.contains(&3));
    }

    #[test]
    pub fn test_example() {
        let corpus = [
            "This is the first document.",
            "This document is the second document.",
            "And this is the third document.",
            "Is this the first document?",
            "This not the first nor the second nor the third, but the fourth document"];
        let minhasher = MinHasher32::new(42 * 3);
        let mut index = MinHashIndex::new(42, 3, 0.5);
        for (i, doc) in corpus.iter().enumerate() {
            index.insert(i, minhasher.create_signature(whitespace_split(&doc.to_lowercase())));
        }
        for (i, doc) in corpus.iter().enumerate() {
            if i < 4 {
                let mut expected = HashSet::new();
                expected.extend(vec![0, 1, 2, 3].into_iter());
                assert_eq!(index.query_owned(&minhasher.create_signature(whitespace_split(&doc.to_lowercase()))), expected);
            } else {
                let mut expected = HashSet::new();
                expected.insert(4);
                assert_eq!(index.query_owned(&minhasher.create_signature(whitespace_split(&doc.to_lowercase()))), expected);
            }
        }

    }


    #[test]
    pub fn test_remove() {
        _test_remove(&mut MinHashIndex::new(4, 2, 0.5));
        _test_remove(&mut MinHashIndex::<_, _, VecContainer<_>>::new_index(4, 2, 0.5));
        _test_remove(&mut MinHashIndex::<_, _, SmallVecContainer<_, 4>>::new_index(4, 2, 0.5));
    }
    pub fn _test_remove<C: IdContainer<u32>>(lsh_index: &mut MinHashIndex<u32, u32, C>) {
        lsh_index.insert(1, vec![1, 1,  1, 1,  1, 1,  1, 1]);
        lsh_index.insert(2, vec![1, 1,  1, 1,  1, 1,  1, 1]);

        lsh_index.insert(3, vec![1, 1,  1, 1,  1, 1,  2, 2]);
        lsh_index.insert(4, vec![1, 1,  1, 1,  1, 1,  2, 3]);

        lsh_index.insert(5, vec![2, 2,  2, 3,  3, 3,  4, 4]);

        lsh_index.insert(6, vec![3, 3,  3, 4,  4, 4,  5, 5]);
        lsh_index.insert(7, vec![3, 3,  3, 4,  4, 4,  5, 6]);

        let res = lsh_index.query(&vec![1, 1,  1, 1,  1, 1,  1, 1]);
        assert_eq!(res, vec![1, 2, 3, 4].iter().collect());

        lsh_index.remove(&1);
        let res = lsh_index.query(&vec![1, 1,  1, 1,  1, 1,  1, 1]);
        assert_eq!(res, vec![2, 3, 4].iter().collect());

        lsh_index.remove(&2);
        let res = lsh_index.query(&vec![1, 1,  1, 1,  1, 1,  1, 1]);
        assert_eq!(res, vec![3, 4].iter().collect());
        let res = lsh_index.query(&vec![1, 1,  1, 1,  1, 1,  2, 2]);
        assert_eq!(res, vec![3, 4].iter().collect());

        lsh_index.remove(&5);

        let res = lsh_index.query(&vec![2, 2,  2, 3,  3, 3,  4, 4]);
        assert_eq!(res, vec![].iter().collect());

        lsh_index.remove(&7);

        let res = lsh_index.query(&vec![3, 3,  3, 4,  4, 4,  5, 5]);
        assert_eq!(res, vec![6].iter().collect());

        lsh_index.remove(&6);

        let res = lsh_index.query(&vec![3, 3,  3, 4,  4, 4,  5, 6]);
        assert_eq!(res.len(), 0);

        lsh_index.remove(&3);
        lsh_index.remove(&4);
        assert_eq!(lsh_index.size(), 0);
    }

    #[test]
    pub fn test_lsh_index_batch_construction() {
        let (b, r) = calculate_minhash_params(0.5, 200);
        let min_hash = MinHasher32::new(b * r);

        fn new_index<C: IdContainer<u32>>(b: usize, r: usize) -> MinHashIndex<u32, u32, C> {
            MinHashIndex::<_, _, C>::new_index(b, r, 0.5)
        }

        _test_lsh_index_batch_construction(&min_hash, &mut new_index::<HashSetContainer<u32>>(b, r));
        _test_lsh_index_batch_construction(&min_hash, &mut new_index::<VecContainer<u32>>(b, r));
        _test_lsh_index_batch_construction(&min_hash, &mut new_index::<SmallVecContainer<u32, 4>>(b, r));
    }


    pub fn _test_lsh_index_batch_construction<C: IdContainer<u32>>(
        min_hash: &MinHasher32<FnvBuildHasher>,
        lsh_index: &mut MinHashIndex<u32, u32, C>) {
        let mut signatures: Vec<(u32, Vec<u32>)> = Vec::new();
        signatures.push((1, min_hash.create_signature(S1.split_whitespace())));
        signatures.push((2, min_hash.create_signature(S2.split_whitespace())));
        signatures.push((3, min_hash.create_signature(S3.split_whitespace())));
        signatures.push((4, min_hash.create_signature(S4.split_whitespace())));
        signatures.push((5, min_hash.create_signature(S5.split_whitespace())));
        signatures.push((6, min_hash.create_signature(S6.split_whitespace())));

        lsh_index.par_bulk_insert_pairs(signatures);
        assert_eq!(lsh_index.size(), 6);

        let ret = lsh_index.query(&min_hash.create_signature(S2.split_whitespace()));

        let ret_str: String = ret.iter().map(|x| x.to_string()).collect();
        assert_eq!(ret.len(), 3, "{}", ret_str);
        assert!(ret.contains(&1));
        assert!(ret.contains(&2));
        assert!(ret.contains(&3));
    }

    fn random_change_values(
        v: &Vec<u64>,
        num_changes: usize,
        num_vecs: usize,
        rng: &mut ThreadRng,
    ) -> Vec<Vec<u64>> {
        let rand_range = Uniform::from(1..100000);
        let index_rand_range = Uniform::from(0..1000);
        (0..num_vecs)
            .map(|_| {
                let indices: Vec<usize> = (0..num_changes)
                    .map(|_| index_rand_range.sample(rng))
                    .collect();
                let changes: Vec<u64> = (0..num_changes).map(|_| rand_range.sample(rng)).collect();
                let mut c = v.clone();
                assert_eq!(c.len(), v.len());
                for i in indices.iter().zip(changes.iter()) {
                    c[*i.0] = i.1.clone();
                }
                c
            })
            .collect()
    }

    #[test]
    pub fn test_lsh_index_batch_construction2() {
        let (b, r) = calculate_minhash_params(0.5, 128);
        let min_hash = MinHasher64V1::new(b * r);
        let mut lsh_index: MinHashIndex<u64, u64> = MinHashIndex::new(b, r, 0.5);

        let mut vecs = Vec::new();
        let rand_range = Uniform::from(1..100000);
        let mut rng = thread_rng();
        let v1: Vec<u64> = (0..1000).map(|_| rand_range.sample(&mut rng)).collect();
        let v2: Vec<u64> = (0..1000).map(|_| rand_range.sample(&mut rng)).collect();
        let v3: Vec<u64> = (0..1000).map(|_| rand_range.sample(&mut rng)).collect();
        assert_eq!(v1.len(), 1000);
        vecs.push(v1.clone());
        vecs.extend_from_slice(random_change_values(&v1, 100, 99, &mut rng).as_slice());
        vecs.push(v2.clone());
        vecs.extend_from_slice(random_change_values(&v2, 50, 99, &mut rng).as_slice());
        vecs.push(v3.clone());
        vecs.extend_from_slice(random_change_values(&v3, 10, 99, &mut rng).as_slice());

        let mut ids: Vec<u64> = (0..300).collect();
        let signatures = vecs
            .iter()
            .map(|v| min_hash.create_signature(v.iter()))
            .collect();

        assert_eq!(vecs.len(), ids.len());
        lsh_index.par_bulk_insert(ids, signatures);
        assert_eq!(lsh_index.size(), 300);

        let ret = lsh_index.query(&min_hash.create_signature(v1.iter()));
        assert_eq!(ret.len(), 100);
        assert_eq!((0..100).filter(|i| ret.contains(&i)).count(), 100);

        let ret = lsh_index.query_top_k(&min_hash.create_signature(v1.iter()), 10);
        assert_eq!(ret.len(), 10);

        let ret = lsh_index.query(&min_hash.create_signature(v2.iter()));
        assert_eq!(ret.len(), 100);
        assert_eq!((100..200).filter(|i| ret.contains(&i)).count(), 100);

        let ret = lsh_index.query(&min_hash.create_signature(v3.iter()));
        assert_eq!(ret.len(), 100);
        assert_eq!((200..300).filter(|i| ret.contains(&i)).count(), 100);

        let removed_ids: Vec<u64> = (0..100).step_by(2).collect();
        lsh_index.bulk_remove(&removed_ids);
        let ret = lsh_index.query(&min_hash.create_signature(v1.iter()));
        assert_eq!(ret.len(), 50);

    }

    // This test addresses issue #19 - Duplicate ids causes panic
    #[test]
    pub fn test_duplidate_ids() {
        let (b, r) = calculate_minhash_params(0.5, 200);
        let min_hash = MinHasher64V1::new(b * r);
        let mut lsh_index = MinHashIndex::new(b, r, 0.5);
        lsh_index.insert(1, min_hash.create_signature(S1.split_whitespace()));
        let (id, _) = lsh_index.query_one(&min_hash.create_signature(S1.split_whitespace())).unwrap();
        assert_eq!(*id, 1);

        // Inserting a record with the same ID. The index with contain
        // signatures S4 and S6 associated with the same ID 1
        lsh_index.insert(1, min_hash.create_signature(S4.split_whitespace()));
        assert_eq!(lsh_index.size(), 1);

        let ret = lsh_index.query_one(&min_hash.create_signature(S1.split_whitespace()));
        assert_eq!(None, ret);

        let (id, similarity) = lsh_index.query_one(&min_hash.create_signature(S4.split_whitespace())).unwrap();
        assert_eq!(*id, 1);

        lsh_index.insert(6, min_hash.create_signature(S6.split_whitespace()));
        assert_eq!(lsh_index.size(), 2);

        // We remove entry with ID 1. Because S1 was overwritten with S4 the signature
        // for S1 will still be in the index
        lsh_index.remove(&1);
        let ret = lsh_index.query(&min_hash.create_signature(S1.split_whitespace()));
        assert_eq!(HashSet::new(), ret);
        let ret = lsh_index.query_one(&min_hash.create_signature(S1.split_whitespace()));
        assert_eq!(None, ret);
    }
}
